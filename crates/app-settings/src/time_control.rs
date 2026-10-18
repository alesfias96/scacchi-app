//! Il tempo di riflessione della partita.

/// Il tempo a disposizione di ogni giocatore, come "minuti + secondi":
/// ad esempio 5+3 = 5 minuti a testa, più 3 secondi aggiunti a ogni mossa giocata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS), ts(export))]
pub struct TimeControl {
    /// Il tempo iniziale di ogni giocatore, in secondi.
    pub initial_seconds: u32,
    /// I secondi aggiunti dopo ogni mossa (incremento "Fischer").
    pub increment_seconds: u32,
}
