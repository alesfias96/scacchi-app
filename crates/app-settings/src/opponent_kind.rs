//! Contro chi si gioca.

/// L'avversario della partita.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS), ts(export))]
pub enum OpponentKind {
    /// Il computer (Stockfish o Maia), al livello scelto (predefinito).
    #[default]
    Computer,
    /// Due persone sulla stessa scacchiera.
    TwoPlayers,
}
