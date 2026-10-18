//! Il computer avversario, come lo mostra l'interfaccia.

use chess_core::Side;
use engine::LevelInfo;

/// Con che colore e a che livello gioca il computer.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct ComputerView {
    /// Il colore del computer.
    pub side: Side,
    /// Il livello del computer, con nome e forza indicativa.
    pub level: LevelInfo,
}
