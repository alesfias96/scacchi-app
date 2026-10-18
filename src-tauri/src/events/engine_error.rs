//! Evento "il motore ha un problema".

use tauri::{AppHandle, Emitter};

/// Il nome dell'evento, da ascoltare nell'interfaccia (`src/services/`).
pub const ENGINE_ERROR: &str = "engine-error";

/// Avvisa l'interfaccia di un problema del motore (ad esempio Stockfish non
/// si avvia), con un messaggio leggibile da mostrare.
pub fn report_engine_error(app: &AppHandle, message: &str) {
    let _ = app.emit(ENGINE_ERROR, message);
}
