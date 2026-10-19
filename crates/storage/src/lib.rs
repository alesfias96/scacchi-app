//! # storage — il database delle partite e dell'allenamento
//!
//! Ogni partita giocata viene salvata in un file **SQLite** (un piccolo
//! database contenuto in un solo file), mossa per mossa, insieme ai dati che
//! serviranno alle statistiche: tempo impiegato per ogni mossa, tempo rimasto,
//! apertura, e le colonne (vuote per ora) che riempirà l'analisi automatica
//! della Fase 8.
//!
//! Lo schema è descritto dai file della cartella `migrations/`, applicati in
//! ordine all'apertura: quando lo schema cambia si aggiunge un file nuovo e i
//! database esistenti vengono aggiornati senza perdere nulla.
//!
//! Organizzazione (un file per responsabilità):
//! - [`database`]: l'apertura del database (file o memoria) e le migrazioni;
//! - [`save_game`]: il salvataggio di una partita (a ogni mossa);
//! - [`load_game`]: la lettura di una partita e della partita non finita più recente;
//! - [`list_games`]: l'elenco dell'archivio, con i filtri ([`game_filter`]);
//! - [`delete_game`]: l'eliminazione di una partita;
//! - [`puzzle_attempts`]: i tentativi dei puzzle (rating, statistiche);
//! - [`puzzle_reviews`]: il ripasso dei puzzle sbagliati;
//! - [`game_summary`]: il riassunto di una partita per l'elenco;
//! - [`codes`]: come i valori diventano testo nel database;
//! - [`storage_error`]: gli errori possibili.

pub mod codes;
pub mod database;
pub mod delete_game;
pub mod game_filter;
pub mod game_summary;
pub mod list_games;
pub mod load_game;
pub mod puzzle_attempts;
pub mod puzzle_reviews;
pub mod save_game;
pub mod storage_error;

pub use database::Database;
pub use game_filter::{GameFilter, OutcomeFilter};
pub use game_summary::GameSummary;
pub use puzzle_attempts::{PuzzleAttemptRecord, PuzzleTotals};
pub use puzzle_reviews::StoredReview;
pub use storage_error::StorageError;
