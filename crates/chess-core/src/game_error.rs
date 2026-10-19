//! Gli errori possibili durante una partita.

use crate::setup_error::SetupError;

/// Il motivo per cui un'operazione sulla partita non è riuscita.
///
/// Ogni variante contiene un messaggio leggibile da mostrare all'utente.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum GameError {
    /// La mossa non è legale nella posizione attuale.
    #[error("mossa non valida in questa posizione: {0}")]
    IllegalMove(String),
    /// La partita è finita: non si possono giocare altre mosse.
    #[error("la partita è finita: non si possono giocare altre mosse")]
    GameOver,
    /// La FEN da importare non è valida.
    #[error(transparent)]
    InvalidPosition(#[from] SetupError),
    /// Il PGN da importare contiene qualcosa che non è una mossa valida.
    #[error("PGN non valido vicino a «{0}»")]
    InvalidPgn(String),
    /// La mossa indicata non è (più) nell'albero delle mosse.
    #[error("questa mossa non c'è più nella partita")]
    UnknownMove,
    /// Si è chiesto di eliminare una mossa della linea principale.
    #[error("si possono eliminare solo le varianti, non la linea principale")]
    NotAVariation,
    /// Il testo da importare è vuoto.
    #[error("il testo da importare è vuoto")]
    EmptyImport,
}
