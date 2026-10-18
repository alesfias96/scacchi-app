//! Gli errori possibili quando si crea una posizione.

/// Il motivo per cui non è stato possibile creare una posizione.
///
/// Ogni variante contiene un messaggio leggibile che spiega il problema.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SetupError {
    /// Il testo non è una FEN scritta correttamente (es. caratteri non validi).
    #[error("FEN non valida: {0}")]
    InvalidFen(String),
    /// La FEN è scritta bene ma descrive una posizione impossibile
    /// (es. manca un re, o il giocatore che non muove è sotto scacco).
    #[error("posizione non valida: {0}")]
    IllegalPosition(String),
}
