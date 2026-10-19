//! La descrizione di un puzzle in corso, pronta per l'interfaccia.

use analysis::Fact;
use app_settings::TrainingMode;
use chess_core::{GameView, LineView, Side};

/// Com'è andata l'ultima azione: decide il messaggio sotto la scacchiera.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub enum Feedback {
    /// Il puzzle è appena cominciato: tocca a chi risolve.
    Ready,
    /// Mossa giusta, ma la soluzione continua.
    Correct,
    /// Mossa sbagliata: si può riprovare.
    Wrong,
    /// Il puzzle è risolto.
    Solved,
    /// La soluzione è stata mostrata.
    SolutionShown,
}

/// Cosa si scopre a puzzle finito: la difficoltà, i temi e il perché della soluzione.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct Reveal {
    /// La difficoltà del puzzle, in punti.
    pub puzzle_rating: u32,
    /// I temi del puzzle, in italiano (es. "Forchetta").
    pub themes: Vec<String>,
    /// La soluzione completa in notazione SAN, dalla posizione del puzzle.
    pub solution: LineView,
    /// I fatti tattici della posizione a favore di chi risolve (dai rilevatori):
    /// spiegano perché la soluzione funziona.
    pub facts: Vec<Fact>,
}

/// Un puzzle in corso, come lo mostra la pagina "Allenamento".
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
// Sono stati indipendenti del puzzle (finito, risolto, ripasso): un enum
// renderebbe la lettura dall'interfaccia più complicata.
#[allow(clippy::struct_excessive_bools)]
pub struct AttemptView {
    /// Il codice del puzzle su Lichess.
    pub puzzle_id: String,
    /// La modalità di allenamento.
    pub mode: TrainingMode,
    /// La posizione e le mosse giocate (la prima è quella dell'avversario).
    pub game: GameView,
    /// Il colore di chi deve trovare la soluzione.
    pub solver: Side,
    /// Da che parte guardare la scacchiera: chi risolve, oppure chi difende
    /// nella modalità "cosa minaccia?".
    pub orientation: Side,
    /// Quante mosse bisogna trovare.
    pub depth: u32,
    /// Quante mosse giuste sono già state trovate.
    pub found: u32,
    /// Com'è andata l'ultima azione.
    pub feedback: Feedback,
    /// La casa del pezzo da muovere, se è stato chiesto un aiuto.
    pub hint_square: Option<String>,
    /// `true` quando il puzzle è finito (risolto o soluzione mostrata).
    pub finished: bool,
    /// `true` se il puzzle è stato risolto senza errori e senza aiuti.
    pub clean: bool,
    /// `true` se è un ripasso: non cambia il rating.
    pub is_review: bool,
    /// Il rating dei puzzle (già aggiornato, se il risultato è deciso).
    pub rating: i32,
    /// Di quanto è cambiato il rating, quando il risultato è deciso.
    pub rating_change: Option<i32>,
    /// Le informazioni scoperte a puzzle finito.
    pub reveal: Option<Reveal>,
}
