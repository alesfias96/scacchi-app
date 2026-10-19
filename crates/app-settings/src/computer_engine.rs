//! Quale "cervello" usa il computer avversario.

/// Il motore che sceglie le mosse del computer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS), ts(export))]
pub enum ComputerEngine {
    /// Stockfish, indebolito al livello scelto (predefinito).
    #[default]
    Stockfish,
    /// Maia: gioca come una persona del livello scelto (da 1100 a 1900 Elo).
    Maia,
}
