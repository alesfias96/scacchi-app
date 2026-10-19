//! Comando degli appunti del sistema.
//!
//! Gli appunti si scrivono da Rust, con il plugin ufficiale di Tauri: dal
//! motore web dell'app la scrittura negli appunti non sempre è permessa.

// Tauri consegna ai comandi i parametri "per valore": la regola di Clippy che
// chiede di usare un riferimento qui non si può applicare.
#![allow(clippy::needless_pass_by_value)]

use tauri::AppHandle;
use tauri_plugin_clipboard_manager::ClipboardExt;

/// Copia un testo negli appunti del sistema (es. la FEN o il PGN della partita).
///
/// - Nome per l'interfaccia: `copy_text`.
/// - Riceve: il testo da copiare.
#[tauri::command]
pub fn copy_text(app: AppHandle, text: String) -> Result<(), String> {
    app.clipboard()
        .write_text(text)
        .map_err(|error| format!("non riesco a scrivere negli appunti: {error}"))
}
