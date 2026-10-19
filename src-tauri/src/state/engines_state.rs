//! I motori Stockfish dell'app: uno che gioca, uno che analizza e uno per il tutor.

use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::available_parallelism;

use engine::{EngineHandle, launch};
use tauri::{AppHandle, Manager};
use tokio::sync::{Mutex, mpsc};

use crate::events::analysis::forward_analysis;
use crate::events::engine_error::report_engine_error;
use crate::state::engine_path::engine_path;

/// Memoria per i calcoli del motore che analizza, in megabyte.
const ANALYZER_HASH_MB: &str = "64";
/// Memoria per i calcoli del motore del tutor, in megabyte.
const TUTOR_HASH_MB: &str = "32";
/// Quanti processori al massimo usa il motore che analizza.
const ANALYZER_MAX_THREADS: usize = 4;

/// Quale dei motori.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Role {
    /// Gioca le mosse del computer.
    Player,
    /// Analizza la posizione mostrata.
    Analyzer,
    /// Dà al tutor valutazioni brevi a richiesta.
    Tutor,
}

/// I motori in funzione.
///
/// Sono programmi Stockfish separati, così l'analisi non rallenta mai il
/// computer avversario (e viceversa), e le domande del tutor non
/// interrompono l'analisi in tempo reale. Ognuno viene avviato la prima volta che
/// serve e riavviato da solo se si chiude per un errore.
#[derive(Debug, Default)]
pub struct EnginesState {
    /// Il motore che gioca, se è già stato avviato.
    player: Mutex<Option<EngineHandle>>,
    /// Il motore che analizza, se è già stato avviato.
    analyzer: Mutex<Option<EngineHandle>>,
    /// Il motore del tutor, se è già stato avviato.
    tutor: Mutex<Option<EngineHandle>>,
    /// Se l'utente ha acceso l'analisi.
    analysis_enabled: AtomicBool,
}

impl EnginesState {
    /// Il motore che gioca, avviandolo se serve.
    ///
    /// # Errors
    ///
    /// Un messaggio leggibile se Stockfish non si avvia.
    pub async fn player(&self, app: &AppHandle) -> Result<EngineHandle, String> {
        self.get_or_launch(app, Role::Player).await
    }

    /// Il motore che analizza, avviandolo se serve.
    ///
    /// # Errors
    ///
    /// Un messaggio leggibile se Stockfish non si avvia.
    pub async fn analyzer(&self, app: &AppHandle) -> Result<EngineHandle, String> {
        self.get_or_launch(app, Role::Analyzer).await
    }

    /// Il motore del tutor, avviandolo se serve.
    ///
    /// # Errors
    ///
    /// Un messaggio leggibile se Stockfish non si avvia.
    pub async fn tutor(&self, app: &AppHandle) -> Result<EngineHandle, String> {
        self.get_or_launch(app, Role::Tutor).await
    }

    /// Il motore che gioca, solo se è già in funzione e libero in questo
    /// istante (per fermarlo senza avviarlo e senza aspettare).
    pub fn try_running_player(&self) -> Option<EngineHandle> {
        self.player.try_lock().ok().and_then(|slot| slot.clone())
    }

    /// Il motore che analizza, solo se è già in funzione e libero in questo
    /// istante (senza aspettare).
    pub fn try_running_analyzer(&self) -> Option<EngineHandle> {
        self.analyzer.try_lock().ok().and_then(|slot| slot.clone())
    }

    /// Accende o spegne l'analisi.
    pub fn set_analysis_enabled(&self, enabled: bool) {
        self.analysis_enabled.store(enabled, Ordering::Relaxed);
    }

    /// `true` se l'analisi è accesa.
    pub fn analysis_enabled(&self) -> bool {
        self.analysis_enabled.load(Ordering::Relaxed)
    }

    /// Lo spazio in cui è custodito il motore indicato.
    fn slot(&self, role: Role) -> &Mutex<Option<EngineHandle>> {
        match role {
            Role::Player => &self.player,
            Role::Analyzer => &self.analyzer,
            Role::Tutor => &self.tutor,
        }
    }

    /// Restituisce il motore indicato, avviandolo se non è ancora in funzione.
    async fn get_or_launch(&self, app: &AppHandle, role: Role) -> Result<EngineHandle, String> {
        let mut slot = self.slot(role).lock().await;
        if let Some(handle) = slot.as_ref() {
            return Ok(handle.clone());
        }

        let (updates, received) = mpsc::unbounded_channel();
        let settings = match role {
            // Il motore che gioca usa le impostazioni normali: per giocare bastano.
            Role::Player => Vec::new(),
            // Il tutor pensa mezzo secondo per volta: bastano poche risorse.
            Role::Tutor => vec![("Hash".to_owned(), TUTOR_HASH_MB.to_owned())],
            Role::Analyzer => {
                let threads = available_parallelism()
                    .map_or(1, |count| count.get() / 2)
                    .clamp(1, ANALYZER_MAX_THREADS);
                vec![
                    ("Threads".to_owned(), threads.to_string()),
                    ("Hash".to_owned(), ANALYZER_HASH_MB.to_owned()),
                ]
            }
        };
        let (handle, work) = launch(&engine_path()?, &settings, updates)
            .await
            .map_err(|error| error.to_string())?;

        if role == Role::Analyzer {
            tauri::async_runtime::spawn(forward_analysis(app.clone(), received));
        }
        // Il motore lavora in un compito separato; se si chiude per un errore,
        // lo si dimentica (la prossima richiesta lo riavvierà) e lo si segnala.
        let app = app.clone();
        tauri::async_runtime::spawn(async move {
            if let Err(error) = work.await {
                app.state::<EnginesState>().slot(role).lock().await.take();
                report_engine_error(&app, &error.to_string());
            }
        });

        *slot = Some(handle.clone());
        Ok(handle)
    }
}
