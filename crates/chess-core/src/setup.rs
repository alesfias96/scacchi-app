//! Come si crea una posizione: quella iniziale o una descritta da una FEN.

use shakmaty::fen::Fen;
use shakmaty::{CastlingMode, Chess};

use crate::position_view::PositionView;
use crate::setup_error::SetupError;

/// Restituisce la posizione iniziale di una partita a scacchi.
#[must_use]
pub fn initial_position() -> PositionView {
    PositionView::from_position(&Chess::default())
}

/// Crea una posizione a partire da una stringa in notazione FEN.
///
/// - Riceve: una FEN, ad esempio
///   `"rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1"`.
/// - Restituisce: la [`PositionView`] della posizione descritta.
///
/// # Errors
///
/// - [`SetupError::InvalidFen`] se il testo non è una FEN scritta correttamente;
/// - [`SetupError::IllegalPosition`] se la FEN descrive una posizione impossibile.
pub fn position_from_fen(fen: &str) -> Result<PositionView, SetupError> {
    parse_fen(fen).map(|position| PositionView::from_position(&position))
}

/// Trasforma una FEN nella posizione di `shakmaty`, pronta per giocarci.
///
/// Usata da [`position_from_fen`] e dalla partita ([`crate::Game`]).
///
/// # Errors
///
/// Gli stessi di [`position_from_fen`].
pub(crate) fn parse_fen(fen: &str) -> Result<Chess, SetupError> {
    // `parse::<Fen>()` dice esplicitamente a Rust quale tipo ottenere dal testo.
    let parsed = fen
        .trim()
        .parse::<Fen>()
        .map_err(|error| SetupError::InvalidFen(error.to_string()))?;

    parsed
        .into_position(CastlingMode::Standard)
        .map_err(|error| SetupError::IllegalPosition(error.to_string()))
}

#[cfg(test)]
mod tests {
    use super::{initial_position, position_from_fen};
    use crate::setup_error::SetupError;
    use crate::side::Side;

    #[test]
    fn the_initial_position_has_twenty_legal_moves() {
        assert_eq!(initial_position().legal_moves.len(), 20);
    }

    #[test]
    fn reads_a_position_after_one_move() {
        let view = position_from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1")
            .expect("FEN valida");

        assert_eq!(view.turn, Side::Black);
        assert_eq!(view.legal_moves.len(), 20);
    }

    #[test]
    fn a_checkmated_position_has_no_legal_moves() {
        // "Matto dell'imbecille" (fool's mate): il bianco subisce matto in 2 mosse.
        let view =
            position_from_fen("rnb1kbnr/pppp1ppp/8/4p3/6Pq/5P2/PPPPP2P/RNBQKBNR w KQkq - 1 3")
                .expect("FEN valida");

        assert!(view.in_check);
        // Se il test fallisce, assert_eq! mostra quali mosse sono state trovate.
        assert_eq!(view.legal_moves, Vec::new());
    }

    #[test]
    fn castling_is_described_as_a_king_move() {
        // Il bianco può arroccare corto: la mossa deve essere e1 → g1.
        let view = position_from_fen("4k3/8/8/8/8/8/8/4K2R w K - 0 1").expect("FEN valida");

        assert!(
            view.legal_moves
                .iter()
                .any(|legal_move| legal_move.from == "e1" && legal_move.to == "g1")
        );
    }

    #[test]
    fn rejects_text_that_is_not_a_fen() {
        let result = position_from_fen("questa non è una fen");

        assert!(matches!(result, Err(SetupError::InvalidFen(_))));
    }

    #[test]
    fn rejects_an_impossible_position() {
        // Scacchiera senza re: è scritta correttamente ma non è una posizione possibile.
        let result = position_from_fen("8/8/8/8/8/8/8/8 w - - 0 1");

        assert!(matches!(result, Err(SetupError::IllegalPosition(_))));
    }
}
