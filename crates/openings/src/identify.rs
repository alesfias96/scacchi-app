//! Il riconoscimento dell'apertura di una partita.

use shakmaty::uci::UciMove;
use shakmaty::{CastlingMode, Chess, Position};

use crate::catalog::lookup;
use crate::opening::Opening;

/// Riconosce l'apertura giocata in una partita.
///
/// - Riceve: la posizione di partenza (FEN) e le mosse giocate, in notazione UCI.
/// - Restituisce: l'apertura più precisa raggiunta, cioè l'ultima posizione
///   della partita che compare nell'elenco; `None` se nessuna posizione vi
///   compare (ad esempio una partita che parte da una posizione particolare)
///   oppure se la FEN non è valida.
///
/// Una mossa non valida interrompe la ricerca: vale l'apertura trovata fin lì.
#[must_use]
pub fn identify(start_fen: &str, moves: &[String]) -> Option<Opening> {
    let mut position: Chess = start_fen
        .parse::<shakmaty::fen::Fen>()
        .ok()?
        .into_position(CastlingMode::Standard)
        .ok()?;
    let mut found = None;
    for uci in moves {
        let Some(chess_move) = uci
            .parse::<UciMove>()
            .ok()
            .and_then(|parsed| parsed.to_move(&position).ok())
        else {
            break;
        };
        position.play_unchecked(chess_move);
        if let Some(opening) = lookup(&position) {
            found = Some(opening.clone());
        }
    }
    found
}

#[cfg(test)]
mod tests {
    use super::identify;

    const START: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    fn moves(text: &str) -> Vec<String> {
        text.split_whitespace().map(str::to_owned).collect()
    }

    #[test]
    fn recognises_the_najdorf() {
        let opening = identify(
            START,
            &moves("e2e4 c7c5 g1f3 d7d6 d2d4 c5d4 f3d4 g8f6 b1c3 a7a6"),
        )
        .expect("apertura riconosciuta");

        assert_eq!(opening.eco, "B90");
        assert!(opening.name.contains("Najdorf"));
    }

    #[test]
    fn keeps_the_deepest_opening_after_leaving_the_list() {
        // Dopo 1.e4 e5 2.Nf3 Nc6 3.Bb5 (Spagnola) la mossa assurda 3...h5
        // esce dall'elenco: resta la Spagnola.
        let opening = identify(START, &moves("e2e4 e7e5 g1f3 b8c6 f1b5 h7h5"))
            .expect("apertura riconosciuta");

        assert_eq!(opening.eco, "C60");
    }

    #[test]
    fn recognises_a_transposition() {
        // 1.Nf3 d5 2.d4 arriva alla stessa posizione di 1.d4 d5 2.Nf3.
        let by_knight = identify(START, &moves("g1f3 d7d5 d2d4")).expect("apertura");
        let by_pawn = identify(START, &moves("d2d4 d7d5 g1f3")).expect("apertura");

        assert_eq!(by_knight, by_pawn);
    }

    #[test]
    fn finds_nothing_without_moves() {
        assert_eq!(identify(START, &[]), None);
    }
}
