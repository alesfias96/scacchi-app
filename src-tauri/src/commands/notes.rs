//! Comandi degli appunti personali.

// Tauri consegna ai comandi i parametri "per valore": la regola di Clippy che
// chiede di usare riferimenti non si può applicare ai comandi di questo file.
#![allow(clippy::needless_pass_by_value)]

use notes::{Note, NoteInput, NoteQuery, all_tags, prepare_note, search_notes};
use tauri::{AppHandle, Manager};

use crate::state::database_state::DatabaseState;
use crate::wall_clock::now_ms;

/// Il testo di un errore del database.
fn text(error: storage::StorageError) -> String {
    error.to_string()
}

/// Gli appunti che corrispondono alla ricerca, dal più recente.
///
/// - Nome per l'interfaccia: `list_notes`.
/// - Riceve: le parole da cercare e l'etichetta (entrambe facoltative).
#[tauri::command]
pub async fn list_notes(app: AppHandle, query: NoteQuery) -> Result<Vec<Note>, String> {
    let state = app.state::<DatabaseState>();
    let all = state.database().await?.all_notes().await.map_err(text)?;
    Ok(search_notes(all, &query))
}

/// Tutte le etichette usate negli appunti, in ordine alfabetico.
///
/// - Nome per l'interfaccia: `note_tags`.
#[tauri::command]
pub async fn note_tags(app: AppHandle) -> Result<Vec<String>, String> {
    let state = app.state::<DatabaseState>();
    let all = state.database().await?.all_notes().await.map_err(text)?;
    Ok(all_tags(&all))
}

/// Salva un appunto: lo crea se è nuovo, altrimenti lo aggiorna.
///
/// - Nome per l'interfaccia: `save_note`.
/// - Riceve: l'appunto come l'ha scritto la persona.
/// - Restituisce: l'appunto salvato (con il suo nome unico, le etichette ripulite e le date).
#[tauri::command]
pub async fn save_note(app: AppHandle, note: NoteInput) -> Result<Note, String> {
    let state = app.state::<DatabaseState>();
    let database = state.database().await?;
    let existing = match &note.id {
        Some(id) => database.note_by_id(id).await.map_err(text)?,
        None => None,
    };
    let prepared = prepare_note(note, existing.as_ref(), now_ms());
    database.save_note(&prepared).await.map_err(text)?;
    Ok(prepared)
}

/// Elimina un appunto.
///
/// - Nome per l'interfaccia: `delete_note`.
/// - Riceve: il nome unico dell'appunto.
#[tauri::command]
pub async fn delete_note(app: AppHandle, id: String) -> Result<(), String> {
    let state = app.state::<DatabaseState>();
    state.database().await?.delete_note(&id).await.map_err(text)
}
