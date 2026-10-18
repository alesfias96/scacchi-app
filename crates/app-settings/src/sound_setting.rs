//! Se l'app riproduce i suoni del gioco (mosse, prese, fine partita).

/// La scelta dei suoni.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS), ts(export))]
pub enum SoundSetting {
    /// I suoni sono attivi (predefinito).
    #[default]
    On,
    /// Nessun suono.
    Off,
}
