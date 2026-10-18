//! La descrizione completa di una partita, pronta per l'interfaccia.

use crate::game_result::GameResult;
use crate::legal_move::LegalMove;
use crate::move_record::MoveRecord;
use crate::position_view::PositionView;

/// Tutto ciò che l'interfaccia deve sapere sulla partita in corso.
///
/// La posizione mostrata è quella del "cursore": di solito l'ultima, ma
/// l'utente può tornare indietro nella lista delle mosse.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct GameView {
    /// La posizione mostrata (quella del cursore).
    pub position: PositionView,
    /// Tutte le mosse della partita, dalla prima all'ultima.
    pub moves: Vec<MoveRecord>,
    /// Quante mosse sono state giocate fino alla posizione mostrata (0 = posizione di partenza).
    pub cursor: usize,
    /// La mossa che ha portato alla posizione mostrata, per evidenziarla sulla scacchiera.
    pub last_move: Option<LegalMove>,
    /// Il risultato, se la partita è finita nella posizione mostrata.
    pub result: Option<GameResult>,
    /// La posizione di partenza in notazione FEN.
    pub starting_fen: String,
}
