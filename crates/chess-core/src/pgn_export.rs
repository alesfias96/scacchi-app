//! Scrittura di una partita nel formato PGN, lo standard dei file di scacchi.

use shakmaty::Chess;
use shakmaty::EnPassantMode;
use shakmaty::fen::Fen;

use crate::game::Game;
use crate::side::Side;

/// Lunghezza massima delle righe delle mosse, come raccomandato dallo standard PGN.
const MAX_LINE_LENGTH: usize = 80;

/// Scrive la partita in formato PGN.
///
/// - Riceve: la partita.
/// - Restituisce: il testo PGN, con le intestazioni (giocatori, risultato,
///   eventuale posizione di partenza) e tutte le mosse, anche quelle dopo
///   la posizione mostrata.
#[must_use]
pub fn to_pgn(game: &Game) -> String {
    let score = game
        .final_result()
        .map_or_else(|| "*".to_owned(), |result| result.score);
    let starting_fen = game.starting_fen();
    let standard_start = Fen::from_position(&Chess::default(), EnPassantMode::Legal).to_string();

    let mut headers = vec![
        tag("Event", "Partita"),
        tag("Site", "Scacchi"),
        tag("Date", "????.??.??"),
        tag("Round", "-"),
        tag("White", "Bianco"),
        tag("Black", "Nero"),
        tag("Result", &score),
    ];
    if starting_fen != standard_start {
        headers.push(tag("SetUp", "1"));
        headers.push(tag("FEN", &starting_fen));
    }

    format!("{}\n\n{}\n", headers.join("\n"), movetext(game, &score))
}

/// Una riga di intestazione PGN, ad esempio `[White "Bianco"]`.
fn tag(name: &str, value: &str) -> String {
    format!("[{name} \"{value}\"]")
}

/// Le mosse numerate (`1. e4 e5 2. Nf3 ...`) seguite dal risultato,
/// divise in righe di al massimo 80 caratteri.
fn movetext(game: &Game, score: &str) -> String {
    let mut tokens = Vec::new();
    for (index, record) in game.moves().iter().enumerate() {
        match record.side {
            Side::White => tokens.push(format!("{}. {}", record.move_number, record.san)),
            // Il numero davanti a una mossa del nero serve solo se la partita
            // comincia con il nero (es. "1... e5").
            Side::Black if index == 0 => {
                tokens.push(format!("{}... {}", record.move_number, record.san));
            }
            Side::Black => tokens.push(record.san.clone()),
        }
    }
    tokens.push(score.to_owned());

    let mut lines: Vec<String> = Vec::new();
    for token in tokens {
        let fits_on_last_line = lines
            .last()
            .is_some_and(|line| line.len() + 1 + token.len() <= MAX_LINE_LENGTH);
        match lines.last_mut() {
            Some(line) if fits_on_last_line => {
                line.push(' ');
                line.push_str(&token);
            }
            _ => lines.push(token),
        }
    }
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::to_pgn;
    use crate::game::Game;

    #[test]
    fn writes_numbered_moves_and_an_open_result() {
        let mut game = Game::new();
        for uci in ["e2e4", "e7e5", "g1f3"] {
            game.play_uci(uci).expect("mossa legale");
        }

        let pgn = to_pgn(&game);

        assert!(pgn.contains("[White \"Bianco\"]"));
        assert!(pgn.contains("[Result \"*\"]"));
        assert!(pgn.contains("1. e4 e5 2. Nf3 *"));
        assert!(!pgn.contains("[FEN"));
    }

    #[test]
    fn writes_the_result_of_a_finished_game() {
        let mut game = Game::new();
        for uci in ["f2f3", "e7e5", "g2g4", "d8h4"] {
            game.play_uci(uci).expect("mossa legale");
        }

        let pgn = to_pgn(&game);

        assert!(pgn.contains("[Result \"0-1\"]"));
        assert!(pgn.contains("1. f3 e5 2. g4 Qh4# 0-1"));
    }

    #[test]
    fn includes_the_starting_position_when_it_is_not_the_standard_one() {
        let mut game = Game::from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1")
            .expect("FEN valida");
        game.play_uci("e7e5").expect("mossa legale");

        let pgn = to_pgn(&game);

        assert!(pgn.contains("[SetUp \"1\"]"));
        assert!(
            pgn.contains("[FEN \"rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1\"]")
        );
        assert!(pgn.contains("1... e5 *"));
    }

    #[test]
    fn keeps_lines_short() {
        let mut game = Game::new();
        for _ in 0..10 {
            for uci in ["g1f3", "g8f6", "f3g1", "f6g8"] {
                if game.play_uci(uci).is_err() {
                    break;
                }
            }
        }

        let pgn = to_pgn(&game);

        assert!(pgn.lines().all(|line| line.len() <= 80));
    }
}
