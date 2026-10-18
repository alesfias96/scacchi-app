//! Con quale colore gioca la persona, contro il computer.

/// Il colore scelto per giocare contro il computer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS), ts(export))]
pub enum ColorChoice {
    /// I pezzi bianchi: si muove per primi (predefinito).
    #[default]
    White,
    /// I pezzi neri.
    Black,
    /// Il colore viene estratto a sorte all'inizio della partita.
    Random,
}
