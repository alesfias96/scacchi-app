//! Rigioca la partita e ne raccoglie le posizioni.

use chess_core::{Game, GameResult, MoveRecord, PositionView};

use crate::review_error::ReviewError;

/// La partita rigiocata.
#[derive(Debug, Clone)]
pub struct Replayed {
    /// Le posizioni in FEN: quella di partenza, poi quella dopo ogni mossa.
    pub fens: Vec<String>,
    /// Le posizioni con turno, scacco e mosse legali (per la scacchiera).
    pub positions: Vec<PositionView>,
    /// Le mosse giocate, con la loro notazione.
    pub records: Vec<MoveRecord>,
    /// Il risultato, se la partita è finita sulla scacchiera (matto, stallo…).
    pub result: Option<GameResult>,
}

/// Rigioca la partita dalla posizione di partenza.
///
/// # Errors
///
/// [`ReviewError::InvalidGame`] se la posizione o una mossa non sono valide.
pub fn replay(start_fen: &str, moves: &[String]) -> Result<Replayed, ReviewError> {
    let invalid = |error: chess_core::GameError| ReviewError::InvalidGame(error.to_string());
    let mut game = Game::from_fen(start_fen).map_err(invalid)?;
    let mut fens = vec![game.fen()];
    let mut positions = vec![game.view().position];
    for uci in moves {
        game.play_uci(uci).map_err(invalid)?;
        fens.push(game.fen());
        positions.push(game.view().position);
    }
    Ok(Replayed {
        fens,
        positions,
        records: game.moves().to_vec(),
        result: game.result(),
    })
}
