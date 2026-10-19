//! Comandi delle varianti: andare a una mossa dell'albero, promuovere, eliminare.

// Tauri consegna ai comandi i parametri "per valore": la regola di Clippy che
// chiede di usare riferimenti non si può applicare ai comandi di questo file.
#![allow(clippy::needless_pass_by_value)]

use game_session::SessionView;
use tauri::AppHandle;

use crate::commands::game::change;

/// Il numero di una mossa dell'albero ricevuto dall'interfaccia.
fn node_id(node: u32) -> usize {
    usize::try_from(node).unwrap_or(usize::MAX)
}

/// Mostra la posizione dopo una mossa dell'albero, anche in una variante.
///
/// - Nome per l'interfaccia: `go_to_node`.
/// - Riceve: il numero della mossa (`None` = posizione di partenza).
#[tauri::command]
pub fn go_to_node(app: AppHandle, node: Option<u32>) -> Result<SessionView, String> {
    change(&app, |session, _| session.go_to_node(node.map(node_id)))
}

/// Fa diventare linea principale la variante che passa dalla mossa indicata.
///
/// - Nome per l'interfaccia: `promote_variation`.
#[tauri::command]
pub fn promote_variation(app: AppHandle, node: u32) -> Result<SessionView, String> {
    change(&app, |session, now| {
        session.promote_variation(node_id(node), now)
    })
}

/// Elimina una variante (la mossa indicata e il suo seguito).
///
/// - Nome per l'interfaccia: `delete_variation`.
#[tauri::command]
pub fn delete_variation(app: AppHandle, node: u32) -> Result<SessionView, String> {
    change(&app, |session, _| session.delete_variation(node_id(node)))
}
