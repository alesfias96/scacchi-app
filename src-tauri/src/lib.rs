//! Libreria principale dell'app desktop.
//!
//! Qui si costruisce l'applicazione Tauri: finestra, comandi che
//! l'interfaccia può chiamare, eventi e stato condiviso.
//!
//! - `commands/`: le richieste che l'interfaccia può fare.
//! - `state/`: le informazioni tenute in memoria mentre l'app è aperta
//!   (la partita in corso, i motori Stockfish, il database, il puzzle in corso).
//! - `events/`: i messaggi inviati all'interfaccia in tempo reale
//!   (mossa del computer, analisi, problemi del motore).
//! - `reactions/`: ciò che succede da solo dopo ogni cambiamento della partita
//!   (e, all'avvio, la ripresa della partita lasciata a metà).

mod commands;
mod events;
mod reactions;
mod state;
mod wall_clock;

use tauri::Manager;

/// Il nome del file del database delle partite, nella cartella dati dell'app.
const DATABASE_FILE_NAME: &str = "partite.sqlite";

/// Avvia l'applicazione desktop.
///
/// - Riceve: niente.
/// - Restituisce: niente; resta in esecuzione finché l'utente chiude l'app.
///
/// # Panics
///
/// Si ferma con un messaggio d'errore esplicito se l'app non riesce a partire
/// (ad esempio per una configurazione non valida), invece di restare in uno
/// stato incerto.
pub fn run() {
    tauri::Builder::default()
        // Le finestre di sistema per salvare e aprire i file .pgn.
        .plugin(tauri_plugin_dialog::init())
        // Gli appunti del sistema (Copia FEN, Copia PGN).
        .plugin(tauri_plugin_clipboard_manager::init())
        // La partita in corso, i motori e il puzzle in corso: Tauri li custodisce e li passa ai
        // comandi che li chiedono.
        .manage(state::session_state::SessionState::default())
        .manage(state::engines_state::EnginesState::default())
        .manage(state::training_state::TrainingState::default())
        .manage(state::review_state::ReviewState::default())
        // All'avvio: il database delle partite (nella cartella dati dell'app,
        // fornita da Tauri per ogni sistema), il lavoratore che salva le
        // partite e la ripresa della partita lasciata a metà.
        .setup(|app| {
            let path = app.path().app_data_dir()?.join(DATABASE_FILE_NAME);
            let (database, saves) = state::database_state::DatabaseState::new(path);
            app.manage(database);
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(reactions::save_worker::run_save_worker(
                handle.clone(),
                saves,
            ));
            tauri::async_runtime::spawn(reactions::resume_game::resume_last_game(handle));
            Ok(())
        })
        // Elenco dei comandi che l'interfaccia può chiamare per nome.
        .invoke_handler(tauri::generate_handler![
            commands::position::initial_position,
            commands::game::get_game,
            commands::game::new_game,
            commands::game::play_move,
            commands::game::undo_move,
            commands::game::go_to_ply,
            commands::variations::go_to_node,
            commands::variations::promote_variation,
            commands::variations::delete_variation,
            commands::clipboard::copy_text,
            commands::game::import_game_text,
            commands::game::export_pgn,
            commands::game::resign_game,
            commands::pgn_file::save_pgn_file,
            commands::pgn_file::open_pgn_file,
            commands::analysis::get_position_report,
            commands::archive::list_games,
            commands::archive::open_game,
            commands::archive::delete_game,
            commands::engine::get_engine_levels,
            commands::engine::get_maia_levels,
            commands::engine::set_analysis,
            commands::settings::get_settings,
            commands::settings::save_settings,
            commands::training::get_training_overview,
            commands::training::start_puzzle,
            commands::training::play_puzzle_move,
            commands::training::puzzle_hint,
            commands::training::show_puzzle_solution,
            commands::tutor::explain_position,
            commands::tutor::try_what_if,
            commands::review::start_review,
            commands::review::cancel_review,
            commands::review::running_review,
            commands::review::get_review,
            commands::stats::get_stats,
            commands::stats::analyze_missing_games,
        ])
        // `generate_context!` legge tauri.conf.json durante la compilazione
        // e incorpora nell'eseguibile configurazione, icone e interfaccia.
        .run(tauri::generate_context!())
        .expect("errore durante l'avvio dell'applicazione Tauri");
}
