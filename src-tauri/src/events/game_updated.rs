//! Evento "la partita è cambiata da sola".

use game_session::SessionView;
use tauri::{AppHandle, Emitter};

/// Il nome dell'evento, da ascoltare nell'interfaccia (`src/services/`).
pub const GAME_UPDATED: &str = "game-updated";

/// Avvisa l'interfaccia che la partita è cambiata senza che l'abbia chiesto
/// (mossa del computer, tempo scaduto) e le manda la nuova descrizione.
pub fn emit_game_updated(app: &AppHandle, view: &SessionView) {
    // Se la finestra è già chiusa non c'è nessuno da avvisare: si ignora.
    let _ = app.emit(GAME_UPDATED, view);
}
