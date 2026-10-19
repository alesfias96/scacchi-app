//! La strategia della posizione, pronta per l'interfaccia.

use chess_core::Side;

use crate::explanation_view::CardView;

/// Una voce del bilancio "chi ha cosa" (es. "più spazio (9 case contro 4)").
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct ImbalanceItem {
    /// Di chi è.
    pub side: Side,
    /// La frase.
    pub text: String,
    /// Le case da evidenziare.
    pub squares: Vec<String>,
}

/// Un piano consigliato, con perché, come, quando usarlo e quando no.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct StrategyPlanView {
    /// Il nome della scheda del piano (es. `"middlegame/minority-attack"`).
    pub key: String,
    /// Per chi è il piano.
    pub side: Side,
    /// Il titolo (quello della scheda).
    pub title: String,
    /// Perché proprio qui: la ragione concreta in questa posizione.
    pub why: String,
    /// Come si fa: i passi tipici.
    pub how: Vec<String>,
    /// Quando conviene.
    pub use_when: Vec<String>,
    /// Quando no.
    pub avoid_when: Vec<String>,
    /// I segnali che sta funzionando.
    pub signs: Vec<String>,
    /// Le case da evidenziare.
    pub squares: Vec<String>,
}

/// La strategia: com'è il centro, chi ha cosa, i piani consigliati.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct StrategyView {
    /// Com'è il centro, in una frase.
    pub center: String,
    /// Il bilancio della posizione, voce per voce.
    pub imbalances: Vec<ImbalanceItem>,
    /// I piani consigliati a chi deve muovere (uno o due).
    pub plans: Vec<StrategyPlanView>,
    /// Il piano principale dell'avversario, se ce n'è uno chiaro.
    pub opponent_plan: Option<StrategyPlanView>,
    /// Le schede dei piani, da aprire.
    pub cards: Vec<CardView>,
}
