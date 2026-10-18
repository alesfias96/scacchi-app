//! Comandi che riguardano le impostazioni dell'utente.
//!
//! Il file delle impostazioni sta nella cartella di configurazione che Tauri
//! fornisce per ogni sistema operativo (su Windows in `AppData\Roaming`),
//! così il codice non contiene mai percorsi scritti a mano.

use std::path::PathBuf;

use app_settings::Settings;
use tauri::{AppHandle, Manager};

/// Nome del file delle impostazioni, dentro la cartella di configurazione dell'app.
const SETTINGS_FILE_NAME: &str = "settings.json";

/// Calcola il percorso completo del file delle impostazioni.
///
/// Restituisce un messaggio d'errore leggibile se il sistema non fornisce
/// una cartella di configurazione (caso molto raro).
fn settings_path(app: &AppHandle) -> Result<PathBuf, String> {
    app.path()
        .app_config_dir()
        .map(|folder| folder.join(SETTINGS_FILE_NAME))
        .map_err(|error| format!("cartella di configurazione non disponibile: {error}"))
}

/// Restituisce le impostazioni salvate.
///
/// - Nome per l'interfaccia: `get_settings`.
/// - Restituisce: le impostazioni salvate oppure, al primo avvio o se il file
///   è rovinato, quelle predefinite. Non fallisce mai: l'app deve sempre aprirsi.
// Tauri consegna ai comandi i parametri "per valore": la regola di Clippy che
// chiede di usare un riferimento qui non si può applicare.
#[allow(clippy::needless_pass_by_value)]
#[tauri::command]
pub fn get_settings(app: AppHandle) -> Settings {
    settings_path(&app)
        .map(|path| app_settings::load_or_default(&path))
        .unwrap_or_default()
}

/// Salva le impostazioni scelte dall'utente.
///
/// - Nome per l'interfaccia: `save_settings`.
/// - Riceve: le nuove impostazioni complete.
/// - Restituisce: niente se il salvataggio riesce, altrimenti un messaggio
///   d'errore leggibile da mostrare all'utente.
#[allow(clippy::needless_pass_by_value)]
#[tauri::command]
pub fn save_settings(app: AppHandle, settings: Settings) -> Result<(), String> {
    let path = settings_path(&app)?;
    app_settings::save(&path, &settings).map_err(|error| error.to_string())
}
