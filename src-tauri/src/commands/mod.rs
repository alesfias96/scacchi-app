//! I comandi che l'interfaccia può chiamare.
//!
//! Ogni file raccoglie i comandi di un argomento. I comandi non contengono
//! logica: ricevono la richiesta, chiamano il nucleo (`crates/`) e
//! restituiscono il risultato.

pub mod game;
pub mod position;
pub mod settings;
