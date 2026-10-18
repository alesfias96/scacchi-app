//! Comandi che riguardano l'archivio delle partite.

// Tauri consegna ai comandi i parametri "per valore": la regola di Clippy che
// chiede di usare riferimenti non si può applicare ai comandi di questo file.
#![allow(clippy::needless_pass_by_value)]

use std::time::Instant;

use game_session::{GameSession, SessionView};
use storage::{GameFilter, GameSummary};
use tauri::{AppHandle, Manager};

use crate::events::game_updated::emit_game_updated;
use crate::reactions::after_change::after_change;
use crate::state::database_state::DatabaseState;
use crate::state::session_state::SessionState;

/// Restituisce le partite salvate che rispettano i filtri, dalla più recente.
///
/// - Nome per l'interfaccia: `list_games`.
/// - Riceve: i filtri (avversario, esito, testo da cercare nell'apertura).
#[tauri::command]
pub async fn list_games(app: AppHandle, filter: GameFilter) -> Result<Vec<GameSummary>, String> {
    let database = app.state::<DatabaseState>();
    database
        .database()
        .await?
        .list(&filter)
        .await
        .map_err(|error| error.to_string())
}

/// Apre una partita dell'archivio in "Gioca": da rivedere se è finita, da
/// continuare se non lo è (con l'orologio fermo fino alla prossima mossa).
///
/// - Nome per l'interfaccia: `open_game`.
/// - Riceve: il nome unico della partita.
/// - Restituisce: la partita aperta.
#[tauri::command]
pub async fn open_game(app: AppHandle, id: String) -> Result<SessionView, String> {
    let database = app.state::<DatabaseState>();
    let snapshot = database
        .database()
        .await?
        .load(&id)
        .await
        .map_err(|error| error.to_string())?
        .ok_or_else(|| "la partita non è più nell'archivio".to_owned())?;
    let now = Instant::now();
    let restored = GameSession::restore(&snapshot, now).map_err(|error| error.to_string())?;
    let view = app.state::<SessionState>().with_session(|session| {
        *session = restored;
        session.view(now)
    });
    after_change(&app);
    Ok(view)
}

/// Elimina una partita dall'archivio. Se è quella aperta in "Gioca", al suo
/// posto comincia una partita nuova (altrimenti verrebbe salvata di nuovo).
///
/// - Nome per l'interfaccia: `delete_game`.
/// - Riceve: il nome unico della partita.
#[tauri::command]
pub async fn delete_game(app: AppHandle, id: String) -> Result<(), String> {
    let now = Instant::now();
    let replaced = app.state::<SessionState>().with_session(|session| {
        (session.id() == id).then(|| {
            *session = GameSession::default();
            session.view(now)
        })
    });
    if let Some(view) = replaced {
        emit_game_updated(&app, &view);
        after_change(&app);
    }
    let database = app.state::<DatabaseState>();
    database
        .database()
        .await?
        .delete(&id)
        .await
        .map_err(|error| error.to_string())
}
