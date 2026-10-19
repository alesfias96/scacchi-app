//! Cosa serve per analizzare una partita: le mosse e le valutazioni del motore.

use engine::Evaluation;

/// Il giudizio del motore su una posizione della partita.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PositionEval {
    /// La valutazione (per il Bianco); `None` se il motore non ha risposto.
    pub evaluation: Option<Evaluation>,
    /// La mossa migliore in UCI; `None` se manca (o se la partita è già finita).
    pub best_move: Option<String>,
}

/// Una partita da analizzare.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewInput {
    /// La posizione di partenza (FEN).
    pub start_fen: String,
    /// Le mosse giocate (UCI).
    pub moves: Vec<String>,
    /// Il giudizio del motore su ogni posizione: una in più delle mosse
    /// (la posizione di partenza, poi quella dopo ogni mossa).
    pub evaluations: Vec<PositionEval>,
}
