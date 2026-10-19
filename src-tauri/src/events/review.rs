//! Eventi dell'analisi delle partite: avanzamento e fine.

use review::{ReviewFinished, ReviewProgress, ReviewQueueProgress};
use tauri::{AppHandle, Emitter};

/// Il nome dell'evento con l'avanzamento, da ascoltare nell'interfaccia (`src/services/`).
pub const REVIEW_PROGRESS: &str = "review-progress";
/// Il nome dell'evento con l'avanzamento della coda di partite da analizzare.
pub const REVIEW_QUEUE: &str = "review-queue";
/// Il nome dell'evento di fine analisi.
pub const REVIEW_FINISHED: &str = "review-finished";

/// Dice all'interfaccia quante posizioni sono già state valutate.
pub fn emit_review_progress(app: &AppHandle, progress: &ReviewProgress) {
    let _ = app.emit(REVIEW_PROGRESS, progress);
}

/// Dice all'interfaccia che l'analisi è finita (riuscita, fermata o fallita).
pub fn emit_review_finished(app: &AppHandle, finished: &ReviewFinished) {
    let _ = app.emit(REVIEW_FINISHED, finished);
}

/// Dice all'interfaccia a che punto è la coda di partite da analizzare.
pub fn emit_review_queue(app: &AppHandle, progress: &ReviewQueueProgress) {
    let _ = app.emit(REVIEW_QUEUE, progress);
}
