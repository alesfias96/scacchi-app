//! Dove si trova il programma di Stockfish.

use std::path::PathBuf;

/// Il nome del programma, come dichiarato in `tauri.conf.json` (`bundle.externalBin`).
const ENGINE_NAME: &str = "stockfish";

/// Il percorso di Stockfish.
///
/// Tauri mette i programmi esterni ("sidecar") nella stessa cartella
/// dell'eseguibile dell'app, togliendo dal nome l'indicazione del sistema:
/// `stockfish.exe` su Windows, `stockfish` su macOS e Linux. Vale sia
/// durante lo sviluppo sia nell'app installata.
///
/// # Errors
///
/// Un messaggio leggibile se non si riesce a sapere dove si trova l'app.
pub fn engine_path() -> Result<PathBuf, String> {
    let executable = std::env::current_exe()
        .map_err(|error| format!("non trovo la cartella dell'app: {error}"))?;
    let folder = executable
        .parent()
        .ok_or_else(|| "non trovo la cartella dell'app".to_owned())?;
    Ok(folder.join(format!("{ENGINE_NAME}{}", std::env::consts::EXE_SUFFIX)))
}
