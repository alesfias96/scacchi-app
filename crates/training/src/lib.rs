//! # training — l'allenamento con i puzzle
//!
//! Circa 100.000 puzzle scelti dal database di Lichess (licenza CC0), inclusi
//! nell'app. Questa crate sa sceglierne uno adatto, verificare le mosse di chi
//! lo risolve, aggiornare il rating dei puzzle e decidere quando un puzzle
//! sbagliato torna per il ripasso. Salvare i risultati è compito di `storage`.
//!
//! Organizzazione (un file per responsabilità):
//! - [`puzzle`]: un puzzle (posizione, soluzione, difficoltà, temi);
//! - [`catalog`]: l'elenco dei puzzle inclusi nell'app;
//! - [`theme`]: i temi, con i nomi in italiano;
//! - [`selection`]: la scelta del prossimo puzzle;
//! - [`random`]: una scelta "a caso" semplice e ripetibile;
//! - [`attempt`]: il puzzle in corso (verifica delle mosse, aiuto, soluzione);
//! - [`attempt_view`]: la descrizione del puzzle in corso per l'interfaccia;
//! - [`rating`]: il rating dei puzzle (formula Elo);
//! - [`review_schedule`]: le scadenze del ripasso (metodo delle scatole);
//! - [`training_overview`]: il riepilogo per la sezione "Allenamento";
//! - [`training_error`]: gli errori possibili.

pub mod attempt;
pub mod attempt_view;
pub mod catalog;
pub mod puzzle;
pub mod random;
pub mod rating;
pub mod review_schedule;
pub mod selection;
pub mod theme;
pub mod training_error;
pub mod training_overview;

pub use attempt::{Outcome, PuzzleAttempt};
pub use attempt_view::{AttemptView, Feedback, Reveal};
pub use catalog::{all_puzzles, puzzle_by_id};
pub use puzzle::Puzzle;
pub use random::seed_from_clock;
pub use rating::{START_RATING, updated_rating};
pub use review_schedule::{ReviewState, next_review};
pub use selection::{choose_puzzle, choose_review};
pub use theme::{ThemeInfo, available_themes, theme_name};
pub use training_error::TrainingError;
pub use training_overview::TrainingOverview;
