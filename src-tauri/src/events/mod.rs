//! I messaggi che Rust invia all'interfaccia in tempo reale.
//!
//! I comandi rispondono a una richiesta; gli eventi invece partono da soli,
//! quando succede qualcosa che l'interfaccia non ha chiesto in quel momento:
//! il computer ha mosso, è scaduto il tempo, il motore ha approfondito l'analisi.
//! Ogni file definisce un evento: il suo nome e come si invia.

pub mod analysis;
pub mod engine_error;
pub mod game_updated;
