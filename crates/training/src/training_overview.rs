//! Il riepilogo mostrato all'apertura della sezione "Allenamento".

use crate::theme::ThemeInfo;

/// Il riepilogo dell'allenamento: livello, ripassi in attesa e temi disponibili.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct TrainingOverview {
    /// Il rating dei puzzle attuale (1500 prima del primo puzzle).
    pub rating: i32,
    /// Quanti puzzle nuovi sono stati affrontati.
    pub attempted: u32,
    /// Quanti di questi sono stati risolti senza errori né aiuti.
    pub solved: u32,
    /// Quanti puzzle sono da ripassare oggi.
    pub due_reviews: u32,
    /// I temi che si possono scegliere, in ordine alfabetico.
    pub themes: Vec<ThemeInfo>,
}
