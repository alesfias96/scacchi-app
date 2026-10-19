//! Comandi dello "sguardo lungo" del tutor: le idee della posizione, della
//! mossa giocata e le strade possibili dell'avversario.
//!
//! Il motore del tutor pensa più a lungo del solito (qualche secondo) e
//! calcola più varianti insieme; il tutor le rigioca e ne spiega l'idea.
//! Una richiesta nuova al motore del tutor annulla quella in corso: se nel
//! frattempo si gioca un'altra mossa, il calcolo vecchio si ferma e il comando
//! risponde con l'errore "annullato", che l'interfaccia ignora.

// Tauri consegna ai comandi i parametri "per valore": la regola di Clippy che
// chiede di usare riferimenti non si può applicare ai comandi di questo file.
#![allow(clippy::needless_pass_by_value)]

use std::time::Duration;

use engine::{AnalysisUpdate, EngineError, Evaluation, SearchPosition};
use tauri::{AppHandle, Manager};
use tutor::{MoveIdea, OpponentPaths, PositionIdeas};

use crate::state::engines_state::EnginesState;

/// Quante varianti chiedere per le idee della posizione.
const POSITION_LINES: u32 = 3;
/// Il tempo di riflessione più breve e più lungo accettati (millesimi di secondo).
const THINK_RANGE_MS: (u32, u32) = (500, 30_000);

/// Il tempo di riflessione chiesto, riportato nei limiti.
fn think_time(think_ms: u32) -> Duration {
    Duration::from_millis(u64::from(
        think_ms.clamp(THINK_RANGE_MS.0, THINK_RANGE_MS.1),
    ))
}

/// Le varianti di Stockfish come le vuole il tutor: mosse e valutazione.
fn as_lines(updates: Vec<AnalysisUpdate>) -> Vec<(Vec<String>, Evaluation)> {
    updates
        .into_iter()
        .map(|update| (update.line.san, update.evaluation))
        .collect()
}

/// Perché le varianti non sono arrivate.
enum LinesError {
    /// Il motore non si è potuto avviare (il messaggio spiega il motivo).
    Unavailable(String),
    /// Il motore ha risposto con un errore (annullato, partita finita, chiuso).
    Engine(EngineError),
}

impl LinesError {
    /// Il messaggio per l'interfaccia.
    fn message(self) -> String {
        match self {
            Self::Unavailable(message) => message,
            Self::Engine(error) => error.to_string(),
        }
    }
}

/// Chiede al motore del tutor `lines` varianti della posizione.
async fn lines_of(
    app: &AppHandle,
    start_fen: &str,
    moves: &[String],
    think: Duration,
    lines: u32,
) -> Result<Vec<(Vec<String>, Evaluation)>, LinesError> {
    let engine = app
        .state::<EnginesState>()
        .tutor(app)
        .await
        .map_err(LinesError::Unavailable)?;
    let position = SearchPosition {
        start_fen: start_fen.to_owned(),
        moves: moves.to_vec(),
    };
    engine
        .evaluate_lines(position, think, lines)
        .await
        .map(as_lines)
        .map_err(LinesError::Engine)
}

/// Le idee della posizione: le migliori varianti di Stockfish, ognuna spiegata.
///
/// - Nome per l'interfaccia: `position_ideas`.
/// - Riceve: la posizione di partenza, le mosse fino alla posizione (UCI) e
///   quanto può pensare il motore (millesimi di secondo).
#[tauri::command]
pub async fn position_ideas(
    app: AppHandle,
    start_fen: String,
    moves: Vec<String>,
    think_ms: u32,
) -> Result<PositionIdeas, String> {
    let lines = lines_of(
        &app,
        &start_fen,
        &moves,
        think_time(think_ms),
        POSITION_LINES,
    )
    .await
    .map_err(LinesError::message)?;
    tutor::position_ideas(&start_fen, &moves, &lines).map_err(|error| error.to_string())
}

/// L'idea della mossa appena giocata e, se Stockfish preferiva un'altra mossa, la sua idea.
///
/// - Nome per l'interfaccia: `move_idea`.
/// - Riceve: la posizione di partenza, le mosse prima della mossa giocata
///   (UCI), la mossa giocata (UCI) e quanto può pensare il motore per ciascuna
///   delle due posizioni (prima e dopo la mossa).
#[tauri::command]
pub async fn move_idea(
    app: AppHandle,
    start_fen: String,
    moves_before: Vec<String>,
    played: String,
    think_ms: u32,
) -> Result<MoveIdea, String> {
    let think = think_time(think_ms);
    let best_before = lines_of(&app, &start_fen, &moves_before, think, 1)
        .await
        .map_err(LinesError::message)?;
    let mut moves_after = moves_before.clone();
    moves_after.push(played.clone());
    // Dopo la mossa la partita può essere finita (matto, stallo): allora non c'è
    // variante. Ogni altro errore (ad esempio il calcolo annullato) va all'interfaccia.
    let after = match lines_of(&app, &start_fen, &moves_after, think, 1).await {
        Ok(lines) => lines.into_iter().next(),
        Err(LinesError::Engine(EngineError::NoMove)) => None,
        Err(error) => return Err(error.message()),
    };
    let fallback = (
        Vec::new(),
        best_before
            .first()
            .map_or(Evaluation::Centipawns(0), |line| line.1),
    );
    tutor::move_idea(
        &start_fen,
        &moves_before,
        &played,
        after.as_ref().unwrap_or(&fallback),
        best_before.first(),
    )
    .map_err(|error| error.to_string())
}

/// Le strade dell'avversario: cosa potrebbe fare se toccasse a lui.
///
/// Si passa la mano all'avversario (stessa posizione, muove lui), il motore
/// del tutor calcola le sue varianti migliori e il tutor le spiega al
/// condizionale. Se chi muove è sotto scacco non si può passare la mano: il
/// tutor dice di parare prima lo scacco.
///
/// - Nome per l'interfaccia: `opponent_paths`.
/// - Riceve: la posizione di partenza, le mosse fino alla posizione (UCI) e
///   quanto può pensare il motore (millesimi di secondo).
#[tauri::command]
pub async fn opponent_paths(
    app: AppHandle,
    start_fen: String,
    moves: Vec<String>,
    think_ms: u32,
) -> Result<OpponentPaths, String> {
    let flipped = tutor::flipped_fen(&start_fen, &moves).map_err(|error| error.to_string())?;
    let Some(flipped) = flipped else {
        return tutor::paths_when_in_check(&start_fen, &moves).map_err(|error| error.to_string());
    };
    // Senza mosse per l'avversario (stallo dopo aver passato la mano) non ci sono strade.
    let lines = match lines_of(&app, &flipped, &[], think_time(think_ms), POSITION_LINES).await {
        Ok(lines) => lines,
        Err(LinesError::Engine(EngineError::NoMove)) => Vec::new(),
        Err(error) => return Err(error.message()),
    };
    tutor::opponent_paths(&flipped, &lines).map_err(|error| error.to_string())
}
