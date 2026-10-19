//! I comandi che l'interfaccia può chiamare.
//!
//! Ogni file raccoglie i comandi di un argomento. I comandi non contengono
//! logica: ricevono la richiesta, chiamano il nucleo (`crates/`) e
//! restituiscono il risultato.

pub mod analysis;
pub mod archive;
pub mod clipboard;
pub mod engine;
pub mod game;
pub mod pgn_file;
pub mod position;
pub mod review;
pub mod settings;
pub mod stats;
pub mod training;
pub mod tutor;
pub mod updates;
pub mod variations;
