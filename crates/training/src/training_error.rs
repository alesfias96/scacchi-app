//! Gli errori possibili durante l'allenamento.

/// Il motivo per cui un'operazione dell'allenamento non è riuscita.
///
/// Ogni variante contiene un messaggio leggibile da mostrare all'utente.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum TrainingError {
    /// Nessun puzzle corrisponde alle scelte fatte.
    #[error("nessun puzzle con queste scelte: prova un altro tema o un'altra profondità")]
    NoPuzzle,
    /// Non ci sono puzzle da ripassare oggi.
    #[error("oggi non ci sono puzzle da ripassare")]
    NothingToReview,
    /// Non c'è un puzzle in corso.
    #[error("nessun puzzle in corso")]
    NoAttempt,
    /// Il puzzle è già finito: non si possono giocare altre mosse.
    #[error("il puzzle è finito")]
    Finished,
    /// La mossa non è legale nella posizione del puzzle.
    #[error("mossa non valida in questa posizione: {0}")]
    IllegalMove(String),
    /// I dati del puzzle non sono validi (non dovrebbe mai succedere).
    #[error("il puzzle {0} non è valido")]
    InvalidPuzzle(String),
}
