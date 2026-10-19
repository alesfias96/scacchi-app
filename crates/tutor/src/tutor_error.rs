//! Gli errori possibili del tutor.

/// Il motivo per cui il tutor non può spiegare una posizione.
///
/// Ogni variante contiene un messaggio leggibile da mostrare all'utente.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum TutorError {
    /// La posizione di partenza non è valida.
    #[error("posizione non valida: {0}")]
    InvalidPosition(String),
    /// Una mossa non è legale.
    #[error("mossa non valida in questa posizione: {0}")]
    IllegalMove(String),
}
