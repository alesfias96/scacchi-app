//! Gli errori possibili nel dialogo con il motore.

/// Il motivo per cui un'operazione con il motore non è riuscita.
///
/// Ogni variante contiene un messaggio leggibile da mostrare all'utente.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum EngineError {
    /// Il programma del motore non si è avviato (file mancante o non eseguibile).
    #[error("non riesco ad avviare il motore: {0}")]
    Launch(String),
    /// La comunicazione con il motore si è interrotta per un errore.
    #[error("errore di comunicazione con il motore: {0}")]
    Io(String),
    /// Il motore si è chiuso.
    #[error("il motore si è chiuso inaspettatamente")]
    Closed,
    /// Il motore non ha risposto in tempo durante l'avvio.
    #[error("il motore non risponde")]
    Timeout,
    /// La ricerca è stata annullata (ad esempio perché è cominciata una nuova partita).
    #[error("la ricerca del motore è stata annullata")]
    Cancelled,
    /// Il motore non ha trovato nessuna mossa (la partita è già finita).
    #[error("il motore non ha mosse da giocare")]
    NoMove,
}

impl From<std::io::Error> for EngineError {
    /// Gli errori di lettura e scrittura diventano [`EngineError::Io`].
    fn from(error: std::io::Error) -> Self {
        Self::Io(error.to_string())
    }
}
