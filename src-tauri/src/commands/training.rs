//! Comandi che riguardano l'allenamento con i puzzle.

// Tauri consegna ai comandi i parametri "per valore": la regola di Clippy che
// chiede di usare riferimenti non si può applicare ai comandi di questo file.
#![allow(clippy::needless_pass_by_value)]

use std::collections::HashSet;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use app_settings::{TrainingMode, TrainingSetup};
use storage::{Database, PuzzleAttemptRecord, StoredReview};
use tauri::{AppHandle, Manager};
use training::{
    AttemptView, PuzzleAttempt, ReviewState, START_RATING, TrainingError, TrainingOverview,
    available_themes, choose_puzzle, choose_review, next_review, seed_from_clock,
};

use crate::state::database_state::DatabaseState;
use crate::state::training_state::{ActivePuzzle, TrainingState};

/// L'istante attuale in millesimi di secondo dal 1970 (come si salva nel database).
fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| {
            i64::try_from(elapsed.as_millis()).unwrap_or(i64::MAX)
        })
}

/// Il rating dei puzzle salvato (1500 prima del primo puzzle).
async fn current_rating(database: &Database) -> Result<i32, String> {
    database
        .latest_puzzle_rating()
        .await
        .map(|rating| rating.unwrap_or(START_RATING))
        .map_err(|error| error.to_string())
}

/// Restituisce il riepilogo dell'allenamento: rating, statistiche, ripassi, temi.
///
/// - Nome per l'interfaccia: `get_training_overview`.
#[tauri::command]
pub async fn get_training_overview(app: AppHandle) -> Result<TrainingOverview, String> {
    let state = app.state::<DatabaseState>();
    let database = state.database().await?;
    let totals = database
        .puzzle_totals()
        .await
        .map_err(|error| error.to_string())?;
    let due = database
        .due_puzzle_reviews(now_ms())
        .await
        .map_err(|error| error.to_string())?;
    Ok(TrainingOverview {
        rating: current_rating(database).await?,
        attempted: totals.attempted,
        solved: totals.solved,
        due_reviews: u32::try_from(due.len()).unwrap_or(u32::MAX),
        themes: available_themes(),
    })
}

/// Comincia un puzzle adatto alle scelte fatte (oppure uno da ripassare).
///
/// - Nome per l'interfaccia: `start_puzzle`.
/// - Riceve: le scelte dell'allenamento (modalità, profondità, tema, difficoltà).
/// - Restituisce: il puzzle appena cominciato.
#[tauri::command]
pub async fn start_puzzle(app: AppHandle, setup: TrainingSetup) -> Result<AttemptView, String> {
    let state = app.state::<DatabaseState>();
    let database = state.database().await?;
    let rating = current_rating(database).await?;
    let seed = seed_from_clock();
    let puzzle = if setup.mode == TrainingMode::Review {
        let due = database
            .due_puzzle_reviews(now_ms())
            .await
            .map_err(|error| error.to_string())?;
        choose_review(&setup, &due, seed).ok_or(TrainingError::NothingToReview)
    } else {
        let attempted: HashSet<String> = database
            .attempted_puzzle_ids()
            .await
            .map_err(|error| error.to_string())?
            .into_iter()
            .collect();
        choose_puzzle(&setup, rating, &attempted, seed).ok_or(TrainingError::NoPuzzle)
    }
    .map_err(|error| error.to_string())?;

    let attempt =
        PuzzleAttempt::start(puzzle, setup.mode, rating).map_err(|error| error.to_string())?;
    let view = attempt.view();
    app.state::<TrainingState>().with_active(|active| {
        *active = Some(ActivePuzzle {
            attempt,
            started: Instant::now(),
        });
    });
    Ok(view)
}

/// Esegue un'azione sul puzzle in corso e, se il risultato si è appena
/// deciso, lo salva (tentativo, rating e ripasso).
async fn act(
    app: &AppHandle,
    action: impl FnOnce(&mut PuzzleAttempt) -> Result<(), TrainingError>,
) -> Result<AttemptView, String> {
    let (view, decided) = app
        .state::<TrainingState>()
        .with_active(|active| {
            let active = active.as_mut().ok_or(TrainingError::NoAttempt)?;
            action(&mut active.attempt)?;
            let puzzle_rating = active.attempt.puzzle().rating;
            let decided = active.attempt.take_outcome().map(|outcome| {
                let elapsed = active.started.elapsed().as_millis();
                (
                    outcome,
                    puzzle_rating,
                    i64::try_from(elapsed).unwrap_or(i64::MAX),
                )
            });
            Ok((active.attempt.view(), decided))
        })
        .map_err(|error: TrainingError| error.to_string())?;

    if let Some((outcome, puzzle_rating, time_ms)) = decided {
        let state = app.state::<DatabaseState>();
        let database = state.database().await?;
        let now = now_ms();
        let record = PuzzleAttemptRecord {
            puzzle_id: view.puzzle_id.clone(),
            played_at_ms: now,
            mode: view.mode,
            solved: outcome.solved,
            time_ms,
            puzzle_rating,
            rating_after: outcome.rating,
        };
        database
            .record_puzzle_attempt(&record)
            .await
            .map_err(|error| error.to_string())?;
        let current = database
            .puzzle_review(&view.puzzle_id)
            .await
            .map_err(|error| error.to_string())?
            .map(|stored| ReviewState {
                box_index: stored.box_index,
                due_at_ms: stored.due_at_ms,
            });
        let next = next_review(current, outcome.solved, now).map(|review| StoredReview {
            box_index: review.box_index,
            due_at_ms: review.due_at_ms,
        });
        database
            .set_puzzle_review(&view.puzzle_id, next)
            .await
            .map_err(|error| error.to_string())?;
    }
    Ok(view)
}

/// Prova una mossa nel puzzle in corso.
///
/// - Nome per l'interfaccia: `play_puzzle_move`.
/// - Riceve: la mossa in notazione UCI (es. `"e2e4"`).
/// - Restituisce: il puzzle aggiornato (con la risposta dell'avversario, se la mossa è giusta).
#[tauri::command]
pub async fn play_puzzle_move(app: AppHandle, uci: String) -> Result<AttemptView, String> {
    act(&app, |attempt| attempt.play(&uci)).await
}

/// Chiede un aiuto: il pezzo da muovere viene evidenziato.
///
/// - Nome per l'interfaccia: `puzzle_hint`.
#[tauri::command]
pub async fn puzzle_hint(app: AppHandle) -> Result<AttemptView, String> {
    act(&app, PuzzleAttempt::hint).await
}

/// Mostra la soluzione del puzzle in corso, giocandola sulla scacchiera.
///
/// - Nome per l'interfaccia: `show_puzzle_solution`.
#[tauri::command]
pub async fn show_puzzle_solution(app: AppHandle) -> Result<AttemptView, String> {
    act(&app, PuzzleAttempt::show_solution).await
}
