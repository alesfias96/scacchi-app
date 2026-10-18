//! Importazione di una partita da testo: riconosce da solo FEN o PGN.

use crate::game::Game;
use crate::game_error::GameError;
use crate::pgn_import::from_pgn;

/// Crea una partita dal testo incollato dall'utente.
///
/// - Se il testo è una FEN (una sola posizione), la partita parte da lì, senza mosse.
/// - Altrimenti viene letto come PGN (una partita con le sue mosse).
///
/// # Errors
///
/// - [`GameError::EmptyImport`] se il testo è vuoto;
/// - gli errori di [`Game::from_fen`] o di [`from_pgn`].
pub fn import_game(text: &str) -> Result<Game, GameError> {
    let text = text.trim();
    if text.is_empty() {
        return Err(GameError::EmptyImport);
    }
    if looks_like_fen(text) {
        Game::from_fen(text)
    } else {
        from_pgn(text)
    }
}

/// Una FEN è una sola riga, senza intestazioni `[...]` e senza numeri di mossa
/// con il punto; la prima parte descrive le 8 righe della scacchiera, separate da `/`.
fn looks_like_fen(text: &str) -> bool {
    !text.contains('\n')
        && !text.starts_with('[')
        && !text.contains('.')
        && text
            .split_whitespace()
            .next()
            .is_some_and(|board| board.matches('/').count() == 7)
}

#[cfg(test)]
mod tests {
    use super::import_game;
    use crate::game_error::GameError;
    use crate::side::Side;

    #[test]
    fn recognises_a_fen() {
        let game = import_game("  rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1 \n")
            .expect("FEN valida");

        assert_eq!(game.moves(), []);
        assert_eq!(game.view().position.turn, Side::Black);
    }

    #[test]
    fn recognises_a_pgn() {
        let game = import_game("1. d4 d5 2. c4 *").expect("PGN valido");

        assert_eq!(game.moves().len(), 3);
    }

    #[test]
    fn rejects_empty_text() {
        assert!(matches!(import_game("   "), Err(GameError::EmptyImport)));
    }
}
