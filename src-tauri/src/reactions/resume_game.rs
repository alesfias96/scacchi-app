//! La ripresa, all'avvio, della partita lasciata a metà.

use std::time::Instant;

use game_session::GameSession;
use tauri::{AppHandle, Manager};

use crate::events::game_updated::emit_game_updated;
use crate::events::storage_error::report_storage_error;
use crate::reactions::after_change::after_change;
use crate::state::database_state::DatabaseState;
use crate::state::session_state::SessionState;

/// Cerca nell'archivio l'ultima partita non finita e, se c'è, la rimette in
/// "Gioca" (con l'orologio fermo fino alla prossima mossa).
///
/// Se nel frattempo l'utente ha già cominciato a giocare (ha mosso un pezzo),
/// la partita nuova non viene toccata: quella vecchia resta nell'archivio.
pub async fn resume_last_game(app: AppHandle) {
    let state = app.state::<DatabaseState>();
    let found = match state.database().await {
        Ok(database) => database
            .latest_unfinished()
            .await
            .map_err(|error| error.to_string()),
        Err(message) => Err(message),
    };
    let snapshot = match found {
        Ok(Some(snapshot)) => snapshot,
        Ok(None) => return,
        Err(message) => {
            report_storage_error(&app, &message);
            return;
        }
    };
    let now = Instant::now();
    let restored = match GameSession::restore(&snapshot, now) {
        Ok(restored) => restored,
        Err(error) => {
            report_storage_error(&app, &error.to_string());
            return;
        }
    };
    let view = app.state::<SessionState>().with_session(|session| {
        if session.has_moves() {
            return None;
        }
        *session = restored;
        Some(session.view(now))
    });
    if let Some(view) = view {
        emit_game_updated(&app, &view);
        after_change(&app);
    }
}
