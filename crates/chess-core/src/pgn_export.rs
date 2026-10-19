//! Scrittura di una partita nel formato PGN, lo standard dei file di scacchi.

use shakmaty::Chess;
use shakmaty::EnPassantMode;
use shakmaty::fen::Fen;

use crate::game::{Game, NodeId};
use crate::side::Side;

/// Lunghezza massima delle righe delle mosse, come raccomandato dallo standard PGN.
const MAX_LINE_LENGTH: usize = 80;

/// Scrive la partita in formato PGN.
///
/// - Riceve: la partita.
/// - Restituisce: il testo PGN, con le intestazioni (giocatori, risultato,
///   eventuale posizione di partenza) e tutte le mosse: la linea principale
///   e le varianti tra parentesi.
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

/// Una mossa scritta nel PGN: con il numero davanti per il Bianco (`12. Nf3`)
/// e, per il Nero, solo quando serve (`12... e5`: all'inizio della partita, di
/// una variante o dopo una variante).
fn move_token(record: &crate::move_record::MoveRecord, with_number: bool) -> String {
    match record.side {
        Side::White => format!("{}. {}", record.move_number, record.san),
        Side::Black if with_number => format!("{}... {}", record.move_number, record.san),
        Side::Black => record.san.clone(),
    }
}

/// Scrive le mosse che seguono `parent` (`None` = la partenza): la linea
/// principale e, dopo ogni sua mossa, le varianti tra parentesi.
fn write_line(game: &Game, parent: Option<NodeId>, with_number: bool, tokens: &mut Vec<String>) {
    let mut current = parent;
    let mut with_number = with_number;
    while let Some((&main, variations)) = game.children(current).split_first() {
        tokens.push(move_token(game.record(main), with_number));
        with_number = false;
        for &variation in variations {
            let mut inner = vec![move_token(game.record(variation), true)];
            write_line(game, Some(variation), false, &mut inner);
            tokens.push(format!("({})", inner.join(" ")));
            // Dopo una variante, anche la mossa del Nero riprende con il numero.
            with_number = true;
        }
        current = Some(main);
    }
}

/// Le mosse numerate (`1. e4 e5 2. Nf3 ...`), con le varianti tra parentesi,
/// seguite dal risultato e divise in righe di al massimo 80 caratteri.
fn movetext(game: &Game, score: &str) -> String {
    let mut tokens = Vec::new();
    write_line(game, None, true, &mut tokens);
    tokens.push(score.to_owned());

    let mut lines: Vec<String> = Vec::new();
    for token in tokens.iter().flat_map(|token| token.split(' ')) {
        let fits_on_last_line = lines
            .last()
            .is_some_and(|line| line.len() + 1 + token.len() <= MAX_LINE_LENGTH);
        match lines.last_mut() {
            Some(line) if fits_on_last_line => {
                line.push(' ');
                line.push_str(token);
            }
            _ => lines.push(token.to_owned()),
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
