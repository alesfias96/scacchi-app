//! Gli errori possibili di Maia.

/// Il motivo per cui Maia non può scegliere una mossa.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum MaiaError {
    /// Il file della rete è rovinato o di un tipo che questa crate non sa calcolare.
    #[error("la rete di Maia non è valida: {0}")]
    BadNetwork(&'static str),
    /// Il livello richiesto non esiste.
    #[error("il livello {0} di Maia non esiste")]
    UnknownLevel(u8),
    /// La posizione di partenza o una mossa della partita non sono valide.
    #[error("la partita non si può rigiocare: {0}")]
    InvalidGame(String),
    /// Nella posizione non ci sono mosse legali (matto o stallo).
    #[error("nella posizione non ci sono mosse da giocare")]
    NoLegalMoves,
}
