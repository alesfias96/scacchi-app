//! Comandi dell'analisi delle partite giocate.

// Tauri consegna ai comandi i parametri "per valore": la regola di Clippy che
// chiede di usare riferimenti non si può applicare ai comandi di questo file.
#![allow(clippy::needless_pass_by_value)]

use engine::Evaluation;
use review::{GameReview, PositionEval, ReviewInput};
use tauri::{AppHandle, Manager};

use crate::reactions::review_worker::run_review;
use crate::state::database_state::DatabaseState;
use crate::state::review_state::ReviewState;

/// La valutazione salvata nelle due colonne del database.
pub(crate) fn join(cp: Option<i32>, mate: Option<i32>) -> Option<Evaluation> {
    cp.map(Evaluation::Centipawns)
        .or(mate.map(Evaluation::Mate))
}

/// Comincia ad analizzare una partita, in sottofondo.
///
/// - Nome per l'interfaccia: `start_review`.
/// - Riceve: il nome unico della partita.
/// - Restituisce: niente; l'avanzamento arriva con l'evento `review-progress`
///   e la fine con `review-finished`.
#[tauri::command]
pub fn start_review(app: AppHandle, id: String) -> Result<(), String> {
    let generation = app
        .state::<ReviewState>()
        .try_start(&id)
        .ok_or_else(|| "è già in corso l'analisi di un'altra partita".to_owned())?;
    tauri::async_runtime::spawn(async move {
        run_review(app.clone(), id, generation).await;
    });
    Ok(())
}

/// Ferma l'analisi in corso (i dati della partita restano come prima).
///
/// - Nome per l'interfaccia: `cancel_review`.
#[tauri::command]
pub fn cancel_review(app: AppHandle) {
    app.state::<ReviewState>().cancel();
}

/// La partita in analisi in questo momento, se c'è.
///
/// - Nome per l'interfaccia: `running_review`.
#[tauri::command]
pub fn running_review(app: AppHandle) -> Option<String> {
    app.state::<ReviewState>().running()
}

/// La revisione di una partita già analizzata.
///
/// - Nome per l'interfaccia: `get_review`.
/// - Riceve: il nome unico della partita.
/// - Restituisce: la revisione, oppure `None` se la partita non è ancora stata analizzata.
#[tauri::command]
pub async fn get_review(app: AppHandle, id: String) -> Result<Option<GameReview>, String> {
    let state = app.state::<DatabaseState>();
    let database = state.database().await?;
    let Some(records) = database
        .load_analysis(&id)
        .await
        .map_err(|error| error.to_string())?
    else {
        return Ok(None);
    };
    let snapshot = database
        .load(&id)
        .await
        .map_err(|error| error.to_string())?
        .ok_or_else(|| "la partita non è più nell'archivio".to_owned())?;

    // Ogni posizione ha la valutazione "prima" della mossa che parte da lì;
    // l'ultima ha la valutazione "dopo" l'ultima mossa.
    let mut evaluations: Vec<PositionEval> = records
        .iter()
        .map(|record| PositionEval {
            evaluation: join(record.eval_before_cp, record.eval_before_mate),
            best_move: record.best_move_uci.clone(),
        })
        .collect();
    evaluations.push(PositionEval {
        evaluation: records
            .last()
            .and_then(|record| join(record.eval_after_cp, record.eval_after_mate)),
        best_move: None,
    });
    let input = ReviewInput {
        start_fen: snapshot.starting_fen,
        moves: snapshot
            .moves
            .into_iter()
            .map(|move_| move_.record.uci)
            .collect(),
        evaluations,
    };
    review::review_game(&input)
        .map(Some)
        .map_err(|error| error.to_string())
}
