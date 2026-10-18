//! L'analisi che segue la posizione mostrata.

use engine::EngineHandle;
use tauri::{AppHandle, Manager};

use crate::events::engine_error::report_engine_error;
use crate::state::engines_state::EnginesState;
use crate::state::session_state::SessionState;

/// Se l'analisi è accesa, fa analizzare la posizione mostrata adesso (oppure
/// ferma il motore se non c'è nulla da analizzare, ad esempio a partita finita).
///
/// Se il motore che analizza è già in funzione la richiesta parte subito, così
/// le richieste arrivano nell'ordine giusto anche quando le posizioni cambiano
/// in fretta (frecce tenute premute). Altrimenti il motore viene avviato in un
/// compito separato, che al termine dell'avvio legge la posizione più recente.
pub fn follow_analysis(app: &AppHandle) {
    let engines = app.state::<EnginesState>();
    if !engines.analysis_enabled() {
        return;
    }
    if let Some(analyzer) = engines.try_running_analyzer() {
        analyze_current(app, &analyzer);
        return;
    }
    let app = app.clone();
    tauri::async_runtime::spawn(async move {
        match app.state::<EnginesState>().analyzer(&app).await {
            Ok(analyzer) => analyze_current(&app, &analyzer),
            Err(message) => report_engine_error(&app, &message),
        }
    });
}

/// Manda al motore la posizione mostrata adesso, o lo ferma se non ce n'è una
/// (o se nel frattempo l'analisi è stata spenta).
fn analyze_current(app: &AppHandle, analyzer: &EngineHandle) {
    if !app.state::<EnginesState>().analysis_enabled() {
        analyzer.stop();
        return;
    }
    let position = app
        .state::<SessionState>()
        .with_session(|session| session.analysis_position());
    match position {
        Some(position) => analyzer.analyze(position),
        None => analyzer.stop(),
    }
}
