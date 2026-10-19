//! La spiegazione di una posizione, pronta per l'interfaccia.

use analysis::Fact;
use chess_core::{PositionView, Side};

use crate::strategy_view::StrategyView;
use crate::topic::Topic;

/// Una frase della spiegazione, con le case da evidenziare (dentro `fact`)
/// e, se c'è, la scheda della base di conoscenza che la approfondisce.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct TutorItem {
    /// La frase, con le case, le frecce e le mosse citate.
    pub fact: Fact,
    /// Il nome della scheda che approfondisce la frase (es. `"structures/carlsbad"`).
    pub card_id: Option<String>,
}

/// Una parte della spiegazione, con il suo titolo (es. "Cosa succede adesso").
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct TutorSection {
    /// Il titolo.
    pub title: String,
    /// Le frasi.
    pub items: Vec<TutorItem>,
}

/// Un piano di una scheda, con chi deve seguirlo già deciso.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct PlanView {
    /// Per chi è (es. "Chi ha l'isolano (il Bianco)").
    pub label: String,
    /// I passi.
    pub steps: Vec<String>,
}

/// Una scheda della base di conoscenza che riguarda la posizione.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct CardView {
    /// Il nome della scheda (es. `"structures/carlsbad"`).
    pub id: String,
    /// L'argomento.
    pub topic: Topic,
    /// Il titolo.
    pub title: String,
    /// Perché la scheda vale qui (es. "Il Bianco ha un pedone isolato di donna in d4").
    pub context: String,
    /// Di cosa si tratta.
    pub summary: String,
    /// Le idee principali.
    pub ideas: Vec<String>,
    /// I piani.
    pub plans: Vec<PlanView>,
    /// Gli errori tipici.
    pub mistakes: Vec<String>,
    /// Quando usarlo.
    pub use_when: Vec<String>,
    /// Quando no.
    pub avoid_when: Vec<String>,
    /// I segnali che funziona.
    pub signs: Vec<String>,
    /// Il tema dei puzzle per allenarsi su questa scheda, se c'è (es. `"fork"`).
    pub puzzle_theme: Option<String>,
}

/// Un passo del metodo «Prima di muovere».
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct ThinkingStep {
    /// Il nome del passo (es. "I tuoi scacchi").
    pub title: String,
    /// Cosa dice il passo in questa posizione.
    pub text: String,
    /// Le mosse del passo (in notazione internazionale), se ce ne sono.
    pub moves: Vec<String>,
}

/// Il metodo «Prima di muovere», riempito con la posizione.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct ThinkingView {
    /// I passi, in ordine.
    pub steps: Vec<ThinkingStep>,
    /// La scheda del metodo, da aprire.
    pub card: Option<CardView>,
}

/// La spiegazione completa di una posizione.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct TutorExplanation {
    /// La posizione spiegata, in notazione FEN.
    pub fen: String,
    /// La posizione con le mosse legali (serve per provare le mosse "e se…").
    pub position: PositionView,
    /// Chi deve muovere.
    pub turn: Side,
    /// Le parti della spiegazione (solo quelle che hanno qualcosa da dire).
    pub sections: Vec<TutorSection>,
    /// Le schede che riguardano la posizione: apertura, struttura, temi, finale, principi.
    pub cards: Vec<CardView>,
    /// La strategia: com'è il centro, chi ha cosa, i piani consigliati.
    pub strategy: StrategyView,
    /// Il metodo «Prima di muovere» per questa posizione.
    pub thinking: ThinkingView,
}
