//! Comandi che riguardano la partita in corso.
//!
//! Ogni comando agisce sulla partita custodita in memoria ([`SessionState`])
//! e restituisce la sua nuova descrizione ([`SessionView`]), così l'interfaccia
//! ha sempre la situazione aggiornata con una sola richiesta. Dopo ogni
//! cambiamento avvia le "reazioni" (mossa del computer, orologio, analisi):
//! vedi `reactions/`.

// Tauri consegna ai comandi i parametri "per valore": la regola di Clippy che
// chiede di usare riferimenti non si può applicare ai comandi di questo file.
#![allow(clippy::needless_pass_by_value)]

use std::time::Instant;

use app_settings::GameSetup;
use chess_core::import_game;
use game_session::{GameSession, SessionError, SessionView, coin_flip};
use tauri::{AppHandle, Manager};

use crate::reactions::after_change::after_change;
use crate::state::session_state::SessionState;

/// Esegue un'operazione che cambia la partita, poi avvia le reazioni.
///
/// - Riceve: l'operazione, che può fallire con un errore della partita.
/// - Restituisce: la nuova descrizione della partita, oppure il messaggio
///   d'errore da mostrare (in quel caso la partita resta com'era).
fn change(
    app: &AppHandle,
    operation: impl FnOnce(&mut GameSession, Instant) -> Result<(), SessionError>,
) -> Result<SessionView, String> {
    let now = Instant::now();
    let view = app.state::<SessionState>().with_session(|session| {
        operation(session, now)
            .map(|()| session.view(now))
            .map_err(|error| error.to_string())
    })?;
    after_change(app);
    Ok(view)
}

/// Restituisce la partita in corso.
///
/// - Nome per l'interfaccia: `get_game`.
#[tauri::command]
pub fn get_game(app: AppHandle) -> SessionView {
    app.state::<SessionState>()
        .with_session(|session| session.view(Instant::now()))
}

/// Comincia una nuova partita come scelto nella finestra "Nuova partita".
///
/// - Nome per l'interfaccia: `new_game`.
/// - Riceve: avversario, colore, livello e tempo.
#[tauri::command]
pub fn new_game(app: AppHandle, setup: GameSetup) -> Result<SessionView, String> {
    change(&app, |session, _| {
        *session = GameSession::start(&setup, coin_flip());
        Ok(())
    })
}

/// Gioca una mossa della persona nella posizione mostrata.
///
/// - Nome per l'interfaccia: `play_move`.
/// - Riceve: la mossa in notazione UCI (es. `"e2e4"`, `"e7e8q"`).
/// - Restituisce: la partita aggiornata, oppure il motivo per cui la mossa
///   non è stata accettata (non legale, non è il proprio turno, ...).
#[tauri::command]
pub fn play_move(app: AppHandle, uci: String) -> Result<SessionView, String> {
    change(&app, |session, now| session.play(&uci, now))
}

/// Ritira l'ultima mossa (contro il computer anche la sua risposta).
///
/// - Nome per l'interfaccia: `undo_move`.
#[tauri::command]
pub fn undo_move(app: AppHandle) -> Result<SessionView, String> {
    change(&app, |session, _| session.undo())
}

/// Mostra la posizione dopo un certo numero di mosse.
///
/// - Nome per l'interfaccia: `go_to_ply`.
/// - Riceve: quante mosse (0 = posizione di partenza).
#[tauri::command]
pub fn go_to_ply(app: AppHandle, ply: usize) -> Result<SessionView, String> {
    change(&app, |session, _| {
        session.go_to(ply);
        Ok(())
    })
}

/// Sostituisce la partita con quella descritta da un testo FEN o PGN.
/// La partita incollata si gioca a due giocatori, senza orologio.
///
/// - Nome per l'interfaccia: `import_game_text` (il nome `import_game` è già
///   usato dalla funzione di `chess_core` che fa il lavoro vero e proprio).
/// - Riceve: il testo incollato dall'utente.
/// - Restituisce: la nuova partita, oppure un messaggio d'errore; in caso
///   di errore la partita in corso resta com'era.
#[tauri::command]
pub fn import_game_text(app: AppHandle, text: String) -> Result<SessionView, String> {
    let imported = import_game(&text).map_err(|error| error.to_string())?;
    change(&app, |session, _| {
        *session = GameSession::from_game(imported);
        Ok(())
    })
}

/// Abbandona la partita.
///
/// - Nome per l'interfaccia: `resign_game`.
#[tauri::command]
pub fn resign_game(app: AppHandle) -> Result<SessionView, String> {
    change(&app, GameSession::resign)
}

/// Restituisce la partita in formato PGN, pronta da copiare.
///
/// - Nome per l'interfaccia: `export_pgn`.
#[tauri::command]
pub fn export_pgn(app: AppHandle) -> String {
    app.state::<SessionState>()
        .with_session(|session| session.export_pgn())
}
