//! L'analisi di una partita, in sottofondo.
//!
//! Stockfish (il quarto motore, dedicato) valuta una posizione dopo l'altra,
//! circa un secondo ciascuna; dopo ogni posizione l'interfaccia riceve
//! l'avanzamento. Alla fine la crate `review` giudica le mosse, i dati si
//! salvano nel database e gli errori gravi diventano puzzle da ripassare.

use std::time::Duration;

use chess_core::Side;
use engine::{Evaluation, SearchPosition};
use review::{MoveAnalysis, PositionEval, ReviewFinished, ReviewInput, ReviewProgress};
use storage::{MoveAnalysisRecord, OwnPuzzleRecord, StoredReview};
use tauri::{AppHandle, Manager};
use training::START_RATING;

use crate::events::review::{emit_review_finished, emit_review_progress};
use crate::state::database_state::DatabaseState;
use crate::state::engines_state::EnginesState;
use crate::state::review_state::ReviewState;
use crate::wall_clock::now_ms;

/// Quanto pensa Stockfish su ogni posizione della partita.
const THINK_TIME: Duration = Duration::from_secs(1);

/// Una valutazione divisa nelle due colonne del database (centipedoni, matto).
fn split(evaluation: Option<Evaluation>) -> (Option<i32>, Option<i32>) {
    match evaluation {
        Some(Evaluation::Centipawns(value)) => (Some(value), None),
        Some(Evaluation::Mate(moves)) => (None, Some(moves)),
        None => (None, None),
    }
}

/// I dati di una mossa come si salvano.
fn to_record(analysis: MoveAnalysis) -> MoveAnalysisRecord {
    let (eval_before_cp, eval_before_mate) = split(analysis.evaluation_before);
    let (eval_after_cp, eval_after_mate) = split(analysis.evaluation_after);
    MoveAnalysisRecord {
        ply: analysis.ply,
        eval_before_cp,
        eval_before_mate,
        eval_after_cp,
        eval_after_mate,
        best_move_uci: analysis.best_move,
        centipawn_loss: analysis.centipawn_loss,
        game_phase: Some(review::phase_code(analysis.phase).to_owned()),
        tactical_themes: analysis.tactical_themes,
        missed_threats: analysis.missed_threats,
    }
}

/// Analizza la partita. Restituisce `false` se è stata fermata prima della fine.
async fn analyze_game(app: &AppHandle, game_id: &str, generation: u64) -> Result<bool, String> {
    let state = app.state::<DatabaseState>();
    let database = state.database().await?;
    let snapshot = database
        .load(game_id)
        .await
        .map_err(|error| error.to_string())?
        .ok_or_else(|| "la partita non è più nell'archivio".to_owned())?;
    let moves: Vec<String> = snapshot
        .moves
        .iter()
        .map(|move_| move_.record.uci.clone())
        .collect();
    let engine = app.state::<EnginesState>().reviewer(app).await?;
    let reviews = app.state::<ReviewState>();

    let total = moves.len() + 1;
    let mut evaluations = Vec::with_capacity(total);
    for index in 0..total {
        if !reviews.is_current(generation) {
            return Ok(false);
        }
        let position = SearchPosition {
            start_fen: snapshot.starting_fen.clone(),
            moves: moves[..index].to_vec(),
        };
        // Una posizione finita (matto, stallo) non ha valutazione: la ricava `review`.
        let update = engine.evaluate(position, THINK_TIME).await.ok();
        evaluations.push(PositionEval {
            evaluation: update.as_ref().map(|update| update.evaluation),
            best_move: update.and_then(|update| update.line.first_move.map(|best| best.uci)),
        });
        emit_review_progress(
            app,
            &ReviewProgress {
                game_id: game_id.to_owned(),
                done: u32::try_from(index + 1).unwrap_or(u32::MAX),
                total: u32::try_from(total).unwrap_or(u32::MAX),
            },
        );
    }

    let input = ReviewInput {
        start_fen: snapshot.starting_fen.clone(),
        moves,
        evaluations,
    };
    let reviewed = review::review_game(&input).map_err(|error| error.to_string())?;
    let records: Vec<MoveAnalysisRecord> = review::move_analysis(&input, &reviewed)
        .map_err(|error| error.to_string())?
        .into_iter()
        .map(to_record)
        .collect();
    let now = now_ms();
    database
        .save_analysis(game_id, &records, now)
        .await
        .map_err(|error| error.to_string())?;

    // Gli errori gravi diventano puzzle: contro il computer solo i propri.
    let players = match snapshot.computer_side {
        Some(computer) => vec![computer.opponent()],
        None => vec![Side::White, Side::Black],
    };
    let rating = database
        .latest_puzzle_rating()
        .await
        .map_err(|error| error.to_string())?
        .unwrap_or(START_RATING);
    for puzzle in
        review::own_puzzles(&input, &reviewed, &players).map_err(|error| error.to_string())?
    {
        let id = format!("own-{game_id}-{}", puzzle.ply);
        let record = OwnPuzzleRecord {
            id: id.clone(),
            game_id: game_id.to_owned(),
            ply: puzzle.ply,
            fen: puzzle.fen,
            moves: puzzle.moves.join(" "),
            rating: u32::try_from(rating).unwrap_or(START_RATING.unsigned_abs()),
            created_at: now,
        };
        let is_new = database
            .save_own_puzzle(&record)
            .await
            .map_err(|error| error.to_string())?;
        if is_new {
            // Il puzzle è subito da ripassare: poi seguirà le solite scadenze.
            let review = StoredReview {
                box_index: 0,
                due_at_ms: now,
            };
            database
                .set_puzzle_review(&id, Some(review))
                .await
                .map_err(|error| error.to_string())?;
        }
    }
    Ok(true)
}

/// Analizza una partita in sottofondo e, alla fine, avvisa l'interfaccia.
///
/// Restituisce `true` se l'analisi è arrivata in fondo (né fermata né fallita).
pub async fn run_review(app: AppHandle, game_id: String, generation: u64) -> bool {
    let outcome = analyze_game(&app, &game_id, generation).await;
    app.state::<ReviewState>().finish();
    let completed = matches!(outcome, Ok(true));
    let finished = match outcome {
        Ok(completed) => ReviewFinished {
            game_id,
            cancelled: !completed,
            error: None,
        },
        Err(error) => ReviewFinished {
            game_id,
            cancelled: false,
            error: Some(error),
        },
    };
    emit_review_finished(&app, &finished);
    completed
}
