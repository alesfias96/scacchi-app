//! Gli errori possibili dell'analisi delle partite.

/// Il motivo per cui una partita non si può analizzare.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ReviewError {
    /// La partita salvata contiene una posizione o una mossa non valida.
    #[error("la partita non si può rigiocare: {0}")]
    InvalidGame(String),
}
