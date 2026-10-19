//! La modalità di allenamento scelta.

/// Come ci si allena con i puzzle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS), ts(export))]
pub enum TrainingMode {
    /// Puzzle con il numero di mosse da trovare scelto (predefinito).
    #[default]
    Depth,
    /// La posizione si vede per pochi secondi, poi si gioca la soluzione a memoria.
    Visualization,
    /// Si è dalla parte di chi difende: si indovina la mossa forte dell'avversario.
    Threat,
    /// Si ripassano i puzzle sbagliati, alle scadenze previste.
    Review,
}
