//! L'orologio come lo mostra l'interfaccia.

use chess_core::Side;

/// I tempi rimasti in un certo istante.
///
/// L'interfaccia li mostra e, per far scorrere i secondi in modo fluido,
/// toglie da sola al lato `running` il tempo passato da quando li ha ricevuti.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct ClockView {
    /// Il tempo rimasto al Bianco, in millesimi di secondo.
    pub white_ms: u32,
    /// Il tempo rimasto al Nero, in millesimi di secondo.
    pub black_ms: u32,
    /// Chi sta consumando il proprio tempo; `None` se l'orologio è fermo.
    pub running: Option<Side>,
}
