//! Evento "il database delle partite ha un problema".

use tauri::{AppHandle, Emitter};

/// Il nome dell'evento, da ascoltare nell'interfaccia (`src/services/`).
pub const STORAGE_ERROR: &str = "storage-error";

/// Avvisa l'interfaccia che una partita non si è potuta salvare o leggere,
/// con un messaggio leggibile da mostrare.
pub fn report_storage_error(app: &AppHandle, message: &str) {
    let _ = app.emit(STORAGE_ERROR, message);
}
