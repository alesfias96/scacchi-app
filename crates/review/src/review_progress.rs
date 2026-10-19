//! I messaggi sull'andamento di un'analisi in corso, per l'interfaccia.

/// A che punto è l'analisi di una partita.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct ReviewProgress {
    /// Il nome unico della partita.
    pub game_id: String,
    /// Quante posizioni sono già state valutate.
    pub done: u32,
    /// Quante posizioni ha la partita.
    pub total: u32,
}

/// L'analisi di una partita è finita (o si è fermata).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct ReviewFinished {
    /// Il nome unico della partita.
    pub game_id: String,
    /// `true` se è stata interrotta prima della fine.
    pub cancelled: bool,
    /// Il motivo, se non è riuscita.
    pub error: Option<String>,
}

/// A che punto è la coda delle partite da analizzare («Analizza le partite mancanti»).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct ReviewQueueProgress {
    /// Quante partite della coda sono già analizzate.
    pub done: u32,
    /// Quante partite c'erano nella coda.
    pub total: u32,
    /// `true` quando la coda è finita (o è stata fermata).
    pub finished: bool,
}
