//! Evento "nuovo aggiornamento dell'analisi".

use engine::AnalysisUpdate;
use tauri::{AppHandle, Emitter};
use tokio::sync::mpsc;

/// Il nome dell'evento, da ascoltare nell'interfaccia (`src/services/`).
pub const ANALYSIS_UPDATE: &str = "analysis-update";

/// Inoltra all'interfaccia gli aggiornamenti del motore che analizza, man
/// mano che arrivano, finché il motore resta in funzione.
pub async fn forward_analysis(
    app: AppHandle,
    mut updates: mpsc::UnboundedReceiver<AnalysisUpdate>,
) {
    while let Some(update) = updates.recv().await {
        let _ = app.emit(ANALYSIS_UPDATE, &update);
    }
}
