//! Comandi delle statistiche (sezione «Progressi»).

// Tauri consegna ai comandi i parametri "per valore": la regola di Clippy che
// chiede di usare riferimenti non si può applicare ai comandi di questo file.
#![allow(clippy::needless_pass_by_value)]

use analysis::GamePhase;
use app_settings::{OpponentKind, StatsFilter, StatsOpponent};
use stats::{GameFacts, MoveFacts, PuzzleFacts, StatsInput, StatsReport};
use storage::{Database, GameStatRow, MoveStatRow, PuzzleStatRow, StatsQuery};
use tauri::{AppHandle, Manager};
use training::{OWN_GAME_THEME, START_RATING, puzzle_by_id};

use crate::commands::review::join;
use crate::reactions::review_queue::run_review_queue;
use crate::state::database_state::DatabaseState;
use crate::wall_clock::now_ms;

/// Un giorno, in millesimi di secondo.
const DAY_MS: i64 = 24 * 60 * 60 * 1000;

/// Cosa leggere dal database, dati i filtri scelti.
fn query(filter: StatsFilter) -> StatsQuery {
    StatsQuery {
        since_ms: filter
            .period
            .days()
            .map(|days| now_ms() - i64::from(days) * DAY_MS),
        opponent: match filter.opponent {
            StatsOpponent::All => None,
            StatsOpponent::Computer => Some(OpponentKind::Computer),
            StatsOpponent::TwoPlayers => Some(OpponentKind::TwoPlayers),
        },
    }
}

/// La fase di gioco dal suo testo nel database.
fn phase(code: Option<&str>) -> Option<GamePhase> {
    match code? {
        "opening" => Some(GamePhase::Opening),
        "middlegame" => Some(GamePhase::Middlegame),
        "endgame" => Some(GamePhase::Endgame),
        _ => None,
    }
}

fn game_facts(row: GameStatRow) -> GameFacts {
    GameFacts {
        computer: row.computer_side.zip(row.level),
        id: row.id,
        started_at_ms: row.started_at_ms,
        opponent: row.opponent,
        finished: row.finished,
        winner: row.winner,
        opening_name: row.opening_name,
        time_initial_ms: row
            .time_initial_seconds
            .map(|seconds| seconds.saturating_mul(1000)),
        analyzed: row.analyzed,
    }
}

fn move_facts(row: MoveStatRow) -> MoveFacts {
    MoveFacts {
        phase: phase(row.game_phase.as_deref()),
        evaluation_before: join(row.eval_before_cp, row.eval_before_mate),
        evaluation_after: join(row.eval_after_cp, row.eval_after_mate),
        game_id: row.game_id,
        side: row.side,
        played: row.uci,
        best_move: row.best_move_uci,
        clock_after_ms: row.clock_after_ms,
    }
}

/// Un tentativo, con profondità e temi del puzzle (dall'elenco di Lichess, o
/// "dalle tue partite" per i puzzle nati dagli errori gravi: una mossa da trovare).
fn puzzle_facts(row: PuzzleStatRow) -> PuzzleFacts {
    let (depth, themes) = puzzle_by_id(&row.puzzle_id).map_or_else(
        || (1, vec![OWN_GAME_THEME.to_owned()]),
        |puzzle| {
            (
                u32::try_from(puzzle.depth()).unwrap_or(u32::MAX),
                puzzle.themes.clone(),
            )
        },
    );
    PuzzleFacts {
        played_at_ms: row.played_at_ms,
        is_review: row.mode == "review",
        solved: row.solved,
        rating_after: row.rating_after,
        depth,
        themes,
    }
}

/// Legge i dati grezzi e li prepara per la crate `stats`.
async fn input(database: &Database, filter: StatsFilter) -> Result<StatsInput, String> {
    let filter = query(filter);
    let text = |error: storage::StorageError| error.to_string();
    Ok(StatsInput {
        games: database
            .stats_games(&filter)
            .await
            .map_err(text)?
            .into_iter()
            .map(game_facts)
            .collect(),
        moves: database
            .stats_moves(&filter)
            .await
            .map_err(text)?
            .into_iter()
            .map(move_facts)
            .collect(),
        puzzles: database
            .stats_puzzles(filter.since_ms)
            .await
            .map_err(text)?
            .into_iter()
            .map(puzzle_facts)
            .collect(),
        puzzle_rating: database
            .latest_puzzle_rating()
            .await
            .map_err(text)?
            .unwrap_or(START_RATING),
        due_reviews: u32::try_from(
            database
                .due_puzzle_reviews(now_ms())
                .await
                .map_err(text)?
                .len(),
        )
        .unwrap_or(u32::MAX),
    })
}

/// Le statistiche della sezione «Progressi», con i filtri scelti.
///
/// - Nome per l'interfaccia: `get_stats`.
/// - Riceve: i filtri (periodo e avversario).
#[tauri::command]
pub async fn get_stats(app: AppHandle, filter: StatsFilter) -> Result<StatsReport, String> {
    let state = app.state::<DatabaseState>();
    let database = state.database().await?;
    Ok(stats::build_report(&input(database, filter).await?))
}

/// Analizza in sottofondo, una alla volta, tutte le partite finite non ancora analizzate.
///
/// - Nome per l'interfaccia: `analyze_missing_games`.
/// - Restituisce: quante partite sono in coda; l'avanzamento arriva con gli
///   eventi `review-queue` (la coda) e `review-progress` (la partita in corso).
#[tauri::command]
pub async fn analyze_missing_games(app: AppHandle) -> Result<u32, String> {
    let state = app.state::<DatabaseState>();
    let games = state
        .database()
        .await?
        .unanalyzed_games()
        .await
        .map_err(|error| error.to_string())?;
    let count = u32::try_from(games.len()).unwrap_or(u32::MAX);
    if count > 0 {
        tauri::async_runtime::spawn(run_review_queue(app.clone(), games));
    }
    Ok(count)
}
