//! Comandi che riguardano i rilevatori (le "osservazioni" sulla posizione).

// Tauri consegna ai comandi i parametri "per valore": la regola di Clippy che
// chiede di usare riferimenti non si può applicare ai comandi di questo file.
#![allow(clippy::needless_pass_by_value)]

use analysis::PositionReport;
use tauri::{AppHandle, Manager};

use crate::state::session_state::SessionState;

/// Restituisce i fatti trovati dai rilevatori nella posizione mostrata
/// (minacce, tattiche, pedoni, re, attività dei pezzi, materiale).
///
/// - Nome per l'interfaccia: `get_position_report`.
/// - Restituisce: il rapporto; `None` solo se la posizione non fosse valida
///   (non dovrebbe mai succedere: arriva dalla partita in corso).
#[tauri::command]
#[must_use]
pub fn get_position_report(app: AppHandle) -> Option<PositionReport> {
    let fen = app
        .state::<SessionState>()
        .with_session(|session| session.shown_fen());
    analysis::analyze(&fen)
}
