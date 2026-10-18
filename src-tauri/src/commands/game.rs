//! Comandi che riguardano la partita in corso.
//!
//! Ogni comando agisce sulla partita custodita in memoria ([`GameState`])
//! e restituisce la sua nuova descrizione ([`GameView`]), così l'interfaccia
//! ha sempre la situazione aggiornata con una sola richiesta.

// Tauri consegna ai comandi i parametri "per valore": la regola di Clippy che
// chiede di usare riferimenti non si può applicare ai comandi di questo file.
#![allow(clippy::needless_pass_by_value)]

use chess_core::{GameView, import_game, to_pgn};
use tauri::State;

use crate::state::game_state::GameState;

/// Restituisce la partita in corso.
///
/// - Nome per l'interfaccia: `get_game`.
#[tauri::command]
pub fn get_game(state: State<'_, GameState>) -> GameView {
    state.with_game(|game| game.view())
}

/// Comincia una nuova partita dalla posizione iniziale.
///
/// - Nome per l'interfaccia: `new_game`.
#[tauri::command]
pub fn new_game(state: State<'_, GameState>) -> GameView {
    state.with_game(|game| {
        *game = chess_core::Game::new();
        game.view()
    })
}

/// Gioca una mossa nella posizione mostrata.
///
/// - Nome per l'interfaccia: `play_move`.
/// - Riceve: la mossa in notazione UCI (es. `"e2e4"`, `"e7e8q"`).
/// - Restituisce: la partita aggiornata, oppure un messaggio d'errore se la
///   mossa non è legale o la partita è finita.
#[tauri::command]
pub fn play_move(state: State<'_, GameState>, uci: String) -> Result<GameView, String> {
    state.with_game(|game| {
        game.play_uci(&uci)
            .map(|()| game.view())
            .map_err(|error| error.to_string())
    })
}

/// Ritira l'ultima mossa della partita.
///
/// - Nome per l'interfaccia: `undo_move`.
#[tauri::command]
pub fn undo_move(state: State<'_, GameState>) -> GameView {
    state.with_game(|game| {
        game.undo();
        game.view()
    })
}

/// Mostra la posizione dopo un certo numero di mosse.
///
/// - Nome per l'interfaccia: `go_to_ply`.
/// - Riceve: quante mosse (0 = posizione di partenza).
#[tauri::command]
pub fn go_to_ply(state: State<'_, GameState>, ply: usize) -> GameView {
    state.with_game(|game| {
        game.go_to(ply);
        game.view()
    })
}

/// Sostituisce la partita con quella descritta da un testo FEN o PGN.
///
/// - Nome per l'interfaccia: `import_game_text` (il nome `import_game` è già
///   usato dalla funzione di `chess_core` che fa il lavoro vero e proprio).
/// - Riceve: il testo incollato dall'utente.
/// - Restituisce: la nuova partita, oppure un messaggio d'errore; in caso
///   di errore la partita in corso resta com'era.
#[tauri::command]
pub fn import_game_text(state: State<'_, GameState>, text: String) -> Result<GameView, String> {
    let imported = import_game(&text).map_err(|error| error.to_string())?;
    Ok(state.with_game(|game| {
        *game = imported;
        game.view()
    }))
}

/// Restituisce la partita in formato PGN, pronta da copiare.
///
/// - Nome per l'interfaccia: `export_pgn`.
#[tauri::command]
pub fn export_pgn(state: State<'_, GameState>) -> String {
    state.with_game(|game| to_pgn(game))
}
