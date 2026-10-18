//! Il modo di scrivere le mosse nella lista della partita.

/// La notazione delle mosse.
///
/// Rust tiene le mosse sempre in notazione internazionale; l'interfaccia le
/// mostra nella notazione scelta qui.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS), ts(export))]
pub enum Notation {
    /// L'iniziale del pezzo è sostituita dalla sua figurina (predefinito).
    #[default]
    Figurine,
    /// Iniziali italiane: R re, D donna, T torre, A alfiere, C cavallo.
    Italiana,
    /// Iniziali inglesi, usate nei libri e nei siti internazionali: K, Q, R, B, N.
    Internazionale,
}
