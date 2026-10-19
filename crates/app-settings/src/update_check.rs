//! Quando l'app controlla se c'è una versione nuova.

/// La scelta del controllo degli aggiornamenti.
///
/// L'app funziona senza Internet: per questo, se non lo si chiede, non si
/// collega mai a nulla.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS), ts(export))]
pub enum UpdateCheck {
    /// Solo quando si preme «Controlla aggiornamenti» nelle Impostazioni (predefinito).
    #[default]
    Manual,
    /// Anche a ogni avvio dell'app.
    OnStart,
}
