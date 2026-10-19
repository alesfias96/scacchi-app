//! Comandi degli aggiornamenti dell'app.
//!
//! L'app si collega a Internet solo qui, e solo quando lo si chiede: legge
//! dalla pagina delle versioni su GitHub il piccolo file `latest.json`, che
//! dice qual è l'ultima versione e dove scaricarla. Il file scaricato viene
//! installato solo se la sua **firma** corrisponde alla chiave pubblica scritta
//! in `tauri.conf.json`: così nessuno può spacciare per aggiornamento un
//! programma diverso.

// Tauri consegna ai comandi i parametri "per valore": la regola di Clippy che
// chiede di usare riferimenti non si può applicare ai comandi di questo file.
#![allow(clippy::needless_pass_by_value)]

use tauri::AppHandle;
use tauri_plugin_updater::UpdaterExt;

/// La versione dell'app installata (es. "0.10.0").
///
/// - Nome per l'interfaccia: `app_version`.
#[tauri::command]
#[must_use]
pub fn app_version(app: AppHandle) -> String {
    app.package_info().version.to_string()
}

/// Controlla se c'è una versione più nuova.
///
/// - Nome per l'interfaccia: `check_update`.
/// - Restituisce: la versione nuova (es. "0.11.0"), oppure niente se si ha già l'ultima.
#[tauri::command]
pub async fn check_update(app: AppHandle) -> Result<Option<String>, String> {
    let updater = app.updater().map_err(|error| error.to_string())?;
    let update = updater.check().await.map_err(|error| error.to_string())?;
    Ok(update.map(|update| update.version))
}

/// Scarica e installa la versione nuova, poi riavvia l'app.
///
/// - Nome per l'interfaccia: `install_update`.
/// - Su Windows l'installatore chiude l'app da solo e la riapre alla fine.
#[tauri::command]
pub async fn install_update(app: AppHandle) -> Result<(), String> {
    let updater = app.updater().map_err(|error| error.to_string())?;
    let Some(update) = updater.check().await.map_err(|error| error.to_string())? else {
        return Err("non c'è nessuna versione nuova da installare".to_owned());
    };
    update
        .download_and_install(|_downloaded, _total| {}, || {})
        .await
        .map_err(|error| error.to_string())?;
    app.restart();
}
