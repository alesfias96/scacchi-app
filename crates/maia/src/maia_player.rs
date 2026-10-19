//! Il giocatore Maia: data una partita, le probabilità delle mosse e la mossa scelta.
//!
//! Le nove reti sono incluse nell'app (da `data/maia/`). Ognuna si legge e si
//! prepara la prima volta che serve, poi resta in memoria.

use std::sync::OnceLock;

use shakmaty::fen::Fen;
use shakmaty::uci::UciMove;
use shakmaty::{CastlingMode, Chess, Position};

use crate::input_planes::encode;
use crate::maia_error::MaiaError;
use crate::maia_level::{MAX_LEVEL, MIN_LEVEL};
use crate::move_choice::{MoveProbability, pick, softmax};
use crate::move_index::nn_index;
use crate::net_weights::read_net;
use crate::network::Network;

/// Il contenuto di un file di rete, incluso nell'app.
macro_rules! net_file {
    ($elo:literal) => {
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../data/maia/maia-",
            $elo,
            ".pb"
        ))
    };
}

/// I file delle reti, dal livello 1 (1100) al livello 9 (1900).
const NET_FILES: [&[u8]; 9] = [
    net_file!("1100"),
    net_file!("1200"),
    net_file!("1300"),
    net_file!("1400"),
    net_file!("1500"),
    net_file!("1600"),
    net_file!("1700"),
    net_file!("1800"),
    net_file!("1900"),
];

/// La rete del livello, preparata la prima volta che serve.
fn network(level: u8) -> Result<&'static Network, MaiaError> {
    static NETWORKS: [OnceLock<Result<Network, MaiaError>>; 9] = [const { OnceLock::new() }; 9];
    if !(MIN_LEVEL..=MAX_LEVEL).contains(&level) {
        return Err(MaiaError::UnknownLevel(level));
    }
    let index = usize::from(level - MIN_LEVEL);
    NETWORKS[index]
        .get_or_init(|| read_net(NET_FILES[index]).map(Network::new))
        .as_ref()
        .map_err(Clone::clone)
}

/// Rigioca la partita: tutte le posizioni, dalla partenza all'ultima.
fn replay(start_fen: &str, moves: &[String]) -> Result<Vec<Chess>, MaiaError> {
    let invalid = |reason: String| MaiaError::InvalidGame(reason);
    let start: Chess = Fen::from_ascii(start_fen.as_bytes())
        .map_err(|error| invalid(error.to_string()))?
        .into_position(CastlingMode::Standard)
        .map_err(|error| invalid(error.to_string()))?;
    let mut positions = vec![start];
    for uci in moves {
        let last = positions.last().expect("c'è sempre la partenza");
        let chess_move = UciMove::from_ascii(uci.as_bytes())
            .map_err(|error| invalid(error.to_string()))?
            .to_move(last)
            .map_err(|error| invalid(error.to_string()))?;
        let mut next = last.clone();
        next.play_unchecked(chess_move);
        positions.push(next);
    }
    Ok(positions)
}

/// Le probabilità delle mosse legali nell'ultima posizione della partita, dalla più probabile.
///
/// # Errors
///
/// - [`MaiaError::UnknownLevel`] se il livello non è tra 1 e 9;
/// - [`MaiaError::InvalidGame`] se la posizione o una mossa non sono valide;
/// - [`MaiaError::NoLegalMoves`] se nell'ultima posizione non si può muovere.
pub fn move_probabilities(
    level: u8,
    start_fen: &str,
    moves: &[String],
) -> Result<Vec<MoveProbability>, MaiaError> {
    let network = network(level)?;
    let history = replay(start_fen, moves)?;
    let current = history
        .last()
        .ok_or_else(|| MaiaError::InvalidGame("partita vuota".to_owned()))?;
    let legal = current.legal_moves();
    if legal.is_empty() {
        return Err(MaiaError::NoLegalMoves);
    }
    let logits = network.policy(&encode(&history));
    let scored = legal
        .into_iter()
        .map(|chess_move| {
            let logit = nn_index(chess_move, current.turn())
                .map_or(f32::NEG_INFINITY, |index| logits[index]);
            (UciMove::from_standard(chess_move).to_string(), logit)
        })
        .collect();
    Ok(softmax(scored))
}

/// La mossa che gioca Maia nell'ultima posizione della partita, in notazione UCI.
///
/// `roll` è un numero a caso tra 0 e 1 (vedi `move_choice::random_roll`): con
/// lo stesso numero la scelta è sempre la stessa, comodo per i test.
///
/// # Errors
///
/// Gli stessi di [`move_probabilities`].
pub fn choose_move(
    level: u8,
    start_fen: &str,
    moves: &[String],
    roll: f64,
) -> Result<String, MaiaError> {
    let probabilities = move_probabilities(level, start_fen, moves)?;
    pick(&probabilities, roll)
        .map(|chosen| chosen.uci.clone())
        .ok_or(MaiaError::NoLegalMoves)
}

#[cfg(test)]
mod tests {
    use super::{choose_move, move_probabilities};
    use crate::maia_error::MaiaError;

    /// La posizione iniziale.
    const START: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    /// Le mosse scritte come elenco di testi.
    fn moves(list: &[&str]) -> Vec<String> {
        list.iter().map(|uci| (*uci).to_owned()).collect()
    }

    /// Le prime `count` mosse più probabili.
    fn top(level: u8, fen: &str, played: &[&str], count: usize) -> Vec<String> {
        move_probabilities(level, fen, &moves(played))
            .expect("posizione valida")
            .into_iter()
            .take(count)
            .map(|candidate| candidate.uci)
            .collect()
    }

    #[test]
    fn plays_like_a_person_in_the_opening() {
        let first = top(1, START, &[], 2);
        assert!(first.contains(&"e2e4".to_owned()) && first.contains(&"d2d4".to_owned()));

        // Il Nero (scacchiera ribaltata per la rete) risponde con una mossa classica.
        let reply = top(1, START, &["e2e4"], 1);
        assert_eq!(reply, ["e7e5"]);
    }

    #[test]
    fn castles_and_promotes_to_a_queen() {
        let italian = [
            "e2e4", "e7e5", "g1f3", "b8c6", "f1c4", "f8c5", "c2c3", "g8f6", "d2d3", "d7d6",
        ];
        assert!(top(9, START, &italian, 3).contains(&"e1g1".to_owned()));

        assert_eq!(top(5, "8/P6k/8/8/8/8/8/K7 w - - 0 1", &[], 1), ["a7a8q"]);
    }

    #[test]
    fn probabilities_cover_all_legal_moves() {
        let all = move_probabilities(5, START, &[]).expect("posizione valida");

        assert_eq!(all.len(), 20);
        let total: f32 = all.iter().map(|candidate| candidate.probability).sum();
        assert!((total - 1.0).abs() < 1e-4);
    }

    #[test]
    fn the_chosen_move_is_one_of_the_likely_ones() {
        let chosen = choose_move(5, START, &[], 0.0).expect("mossa");

        assert_eq!(chosen, top(5, START, &[], 1)[0]);
    }

    #[test]
    fn wrong_requests_are_errors() {
        assert_eq!(
            move_probabilities(10, START, &[]),
            Err(MaiaError::UnknownLevel(10))
        );
        assert!(matches!(
            move_probabilities(5, START, &moves(&["e2e5"])),
            Err(MaiaError::InvalidGame(_))
        ));
        // Matto del barbiere: il Nero non ha mosse.
        let mated = moves(&["e2e4", "e7e5", "f1c4", "b8c6", "d1h5", "g8f6", "h5f7"]);
        assert_eq!(
            move_probabilities(5, START, &mated),
            Err(MaiaError::NoLegalMoves)
        );
    }
}
