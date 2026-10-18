//! Comandi che riguardano il motore (Stockfish).

// Tauri consegna ai comandi i parametri "per valore": la regola di Clippy che
// chiede di usare riferimenti non si può applicare ai comandi di questo file.
#![allow(clippy::needless_pass_by_value)]

use engine::LevelInfo;
use tauri::{AppHandle, Manager};

use crate::reactions::follow_analysis::follow_analysis;
use crate::state::engines_state::EnginesState;

/// Restituisce i livelli di gioco del computer, dal più debole al più forte.
///
/// - Nome per l'interfaccia: `get_engine_levels`.
#[tauri::command]
#[must_use]
pub fn get_engine_levels() -> Vec<LevelInfo> {
    engine::levels()
}

/// Accende o spegne l'analisi in tempo reale della posizione mostrata.
///
/// - Nome per l'interfaccia: `set_analysis`.
/// - Riceve: `true` per accenderla, `false` per spegnerla.
/// - Gli aggiornamenti arrivano poi con l'evento `analysis-update`.
#[tauri::command]
pub async fn set_analysis(app: AppHandle, enabled: bool) {
    let engines = app.state::<EnginesState>();
    engines.set_analysis_enabled(enabled);
    if enabled {
        follow_analysis(&app);
    } else if let Some(analyzer) = engines.try_running_analyzer() {
        analyzer.stop();
    }
}
