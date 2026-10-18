//! Gli errori possibili durante la partita in corso.

use chess_core::GameError;

/// Il motivo per cui un'operazione sulla partita non è riuscita.
///
/// Ogni variante contiene un messaggio leggibile da mostrare all'utente.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SessionError {
    /// Un errore delle regole (mossa illegale, partita finita, ...).
    #[error(transparent)]
    Game(#[from] GameError),
    /// Tocca al computer: la persona deve aspettare la sua mossa.
    #[error("tocca al computer: aspetta la sua mossa")]
    NotYourTurn,
    /// Contro il computer o con l'orologio si gioca solo dall'ultima posizione.
    #[error("per giocare torna all'ultima mossa della partita")]
    NotAtEnd,
    /// Con l'orologio le mosse non si possono ritirare.
    #[error("con l'orologio le mosse non si possono ritirare")]
    UndoNotAllowed,
    /// La mossa del computer è arrivata quando la partita era già cambiata:
    /// viene scartata (non è un problema per l'utente).
    #[error("la mossa del computer non vale più per questa posizione")]
    StaleEngineMove,
}
