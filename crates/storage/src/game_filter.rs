//! I filtri dell'archivio.

use app_settings::OpponentKind;

/// Come è andata una partita, dal punto di vista di chi gioca.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Deserialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub enum OutcomeFilter {
    /// Vinte contro il computer.
    Won,
    /// Perse contro il computer.
    Lost,
    /// Finite patte (contro il computer o a due giocatori).
    Drawn,
    /// Non ancora finite.
    Unfinished,
}

/// Quali partite mostrare nell'archivio. I filtri vuoti non escludono nulla.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Deserialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct GameFilter {
    /// Solo contro il computer, o solo a due giocatori.
    pub opponent: Option<OpponentKind>,
    /// Solo vinte, perse, patte o non finite.
    pub outcome: Option<OutcomeFilter>,
    /// Un testo da cercare nel nome dell'apertura (in inglese, come nell'elenco di
    /// Lichess) o nel suo codice ECO: ad esempio `"Sicilian"` o `"B90"`.
    pub text: Option<String>,
}
