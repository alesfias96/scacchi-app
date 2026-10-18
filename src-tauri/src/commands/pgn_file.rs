//! Comandi per salvare e aprire file `.pgn` con la finestra del sistema.

// Tauri consegna ai comandi i parametri "per valore": la regola di Clippy che
// chiede di usare riferimenti non si può applicare ai comandi di questo file.
#![allow(clippy::needless_pass_by_value)]

use std::path::PathBuf;
use std::time::Instant;

use chess_core::import_game;
use game_session::{GameSession, SessionView};
use tauri::{AppHandle, Manager};
use tauri_plugin_dialog::{DialogExt, FilePath};

use crate::reactions::after_change::after_change;
use crate::state::session_state::SessionState;

/// Il nome del tipo di file mostrato nella finestra del sistema.
const PGN_FILTER_NAME: &str = "Partita a scacchi (PGN)";
/// Il nome proposto per il file da salvare.
const DEFAULT_FILE_NAME: &str = "partita.pgn";

/// Salva la partita in un file `.pgn`, scelto con la finestra del sistema.
///
/// - Nome per l'interfaccia: `save_pgn_file`.
/// - Restituisce: `true` se il file è stato salvato, `false` se l'utente ha annullato.
#[tauri::command]
pub async fn save_pgn_file(app: AppHandle) -> Result<bool, String> {
    let pgn = app
        .state::<SessionState>()
        .with_session(|session| session.export_pgn());
    let dialog = app
        .dialog()
        .file()
        .add_filter(PGN_FILTER_NAME, &["pgn"])
        .set_file_name(DEFAULT_FILE_NAME);
    // La finestra del sistema "aspetta" l'utente: la si apre fuori dal compito
    // principale, così l'app resta reattiva.
    let chosen = tauri::async_runtime::spawn_blocking(move || dialog.blocking_save_file())
        .await
        .map_err(|error| error.to_string())?;
    let Some(path) = chosen.map(to_path).transpose()? else {
        return Ok(false);
    };
    std::fs::write(&path, pgn).map_err(|error| format!("non riesco a salvare il file: {error}"))?;
    Ok(true)
}

/// Apre un file `.pgn` (o con una posizione FEN), scelto con la finestra del
/// sistema, al posto della partita in corso; la partita si gioca a due giocatori.
///
/// - Nome per l'interfaccia: `open_pgn_file`.
/// - Restituisce: la nuova partita, oppure `None` se l'utente ha annullato.
#[tauri::command]
pub async fn open_pgn_file(app: AppHandle) -> Result<Option<SessionView>, String> {
    let dialog = app.dialog().file().add_filter(PGN_FILTER_NAME, &["pgn"]);
    let chosen = tauri::async_runtime::spawn_blocking(move || dialog.blocking_pick_file())
        .await
        .map_err(|error| error.to_string())?;
    let Some(path) = chosen.map(to_path).transpose()? else {
        return Ok(None);
    };
    let bytes =
        std::fs::read(&path).map_err(|error| format!("non riesco a leggere il file: {error}"))?;
    // Alcuni programmi salvano i PGN con accenti in una codifica vecchia: i
    // caratteri non leggibili diventano "�" invece di bloccare tutto.
    let text = String::from_utf8_lossy(&bytes);
    let imported = import_game(&text).map_err(|error| error.to_string())?;
    let now = Instant::now();
    let view = app.state::<SessionState>().with_session(|session| {
        *session = GameSession::from_game(imported, now);
        session.view(now)
    });
    after_change(&app);
    Ok(Some(view))
}

/// Il percorso del file scelto nella finestra del sistema.
fn to_path(file: FilePath) -> Result<PathBuf, String> {
    file.into_path().map_err(|error| error.to_string())
}
