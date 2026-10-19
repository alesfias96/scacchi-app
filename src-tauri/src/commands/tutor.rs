//! Comandi del tutor: la spiegazione della posizione e gli scenari "e se…".

// Tauri consegna ai comandi i parametri "per valore": la regola di Clippy che
// chiede di usare riferimenti non si può applicare ai comandi di questo file.
#![allow(clippy::needless_pass_by_value)]

use std::time::Duration;

use engine::{AnalysisUpdate, SearchPosition};
use tauri::{AppHandle, Manager};
use tutor::{TutorExplanation, WhatIfView};

use crate::state::engines_state::EnginesState;

/// Quanto pensa Stockfish per ogni valutazione del tutor.
const TUTOR_THINK_TIME: Duration = Duration::from_millis(500);

/// La valutazione breve di una posizione, oppure `None` se Stockfish non c'è,
/// non risponde o la partita in quella posizione è già finita. Il tutor
/// funziona anche senza: semplicemente non dà giudizi del motore.
async fn evaluate(app: &AppHandle, start_fen: &str, moves: &[String]) -> Option<AnalysisUpdate> {
    let engine = app.state::<EnginesState>().tutor(app).await.ok()?;
    let position = SearchPosition {
        start_fen: start_fen.to_owned(),
        moves: moves.to_vec(),
    };
    engine.evaluate(position, TUTOR_THINK_TIME).await.ok()
}

/// Spiega una posizione.
///
/// - Nome per l'interfaccia: `explain_position`.
/// - Riceve: la posizione di partenza, le mosse fino alla posizione da
///   spiegare (UCI) e se chiedere anche il giudizio di Stockfish.
/// - Restituisce: la spiegazione.
#[tauri::command]
pub async fn explain_position(
    app: AppHandle,
    start_fen: String,
    moves: Vec<String>,
    with_engine: bool,
) -> Result<TutorExplanation, String> {
    let evaluation = if with_engine {
        evaluate(&app, &start_fen, &moves).await
    } else {
        None
    };
    tutor::explain(&start_fen, &moves, evaluation.as_ref()).map_err(|error| error.to_string())
}

/// Prova una mossa e dice cosa cambia. La partita vera non cambia.
///
/// - Nome per l'interfaccia: `try_what_if`.
/// - Riceve: la posizione di partenza, le mosse fino alla posizione in cui si
///   prova (comprese quelle già provate prima), la mossa da provare e se
///   chiedere anche il giudizio di Stockfish (prima e dopo la mossa).
/// - Restituisce: lo scenario.
#[tauri::command]
pub async fn try_what_if(
    app: AppHandle,
    start_fen: String,
    moves: Vec<String>,
    uci: String,
    with_engine: bool,
) -> Result<WhatIfView, String> {
    let (before, after) = if with_engine {
        let mut after_moves = moves.clone();
        after_moves.push(uci.clone());
        (
            evaluate(&app, &start_fen, &moves).await,
            evaluate(&app, &start_fen, &after_moves).await,
        )
    } else {
        (None, None)
    };
    tutor::what_if(&start_fen, &moves, &uci, before.as_ref(), after.as_ref())
        .map_err(|error| error.to_string())
}
