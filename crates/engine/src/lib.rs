//! # engine — il dialogo con i motori di scacchi
//!
//! Un "motore" è un programma che calcola le mosse, come Stockfish. Si parla
//! con lui a righe di testo, secondo il protocollo **UCI**: gli si dice la
//! posizione (`position ...`), gli si chiede di pensare (`go ...`) e lui
//! racconta cosa sta vedendo (`info ...`) e infine sceglie (`bestmove ...`).
//!
//! Questa crate:
//! - avvia il motore e lo prepara ([`launch`]);
//! - gli manda le richieste senza mai bloccare chi le fa ([`EngineHandle`]):
//!   analisi continua di una posizione oppure ricerca della mossa da giocare;
//! - traduce le sue risposte in dati semplici ([`AnalysisUpdate`]);
//! - sa come farlo giocare a dieci livelli diversi ([`strength`]).
//!
//! Organizzazione (un file per responsabilità):
//! - [`uci_command`], [`search_position`], [`search_limits`]: ciò che si manda al motore;
//! - [`engine_message`], [`engine_info`], [`engine_option`], [`score`]: ciò che il motore risponde;
//! - [`uci_engine`]: la connessione (presentazione, invio, lettura);
//! - [`engine_job`], [`engine_handle`], [`engine_worker`]: le richieste e chi le esegue;
//! - [`connect`]: l'avvio del programma del motore;
//! - [`evaluation`], [`analysis_update`]: i dati per l'interfaccia;
//! - [`strength`]: i livelli di gioco;
//! - [`engine_error`]: gli errori possibili.

pub mod analysis_update;
pub mod connect;
pub mod engine_error;
pub mod engine_handle;
pub mod engine_info;
pub mod engine_job;
pub mod engine_message;
pub mod engine_option;
pub mod engine_worker;
pub mod evaluation;
pub mod score;
pub mod search_limits;
pub mod search_position;
pub mod strength;
pub mod uci_command;
pub mod uci_engine;

#[cfg(test)]
mod fake_engine;

pub use analysis_update::AnalysisUpdate;
pub use connect::{connect, launch};
pub use engine_error::EngineError;
pub use engine_handle::EngineHandle;
pub use engine_worker::EngineWorker;
pub use evaluation::Evaluation;
pub use search_limits::SearchLimits;
pub use search_position::SearchPosition;
pub use strength::{LevelInfo, MAX_LEVEL, MIN_LEVEL, Strength, levels, strength};
