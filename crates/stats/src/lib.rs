//! # stats — le statistiche dei progressi
//!
//! Riceve i dati grezzi letti dal database (partite, mosse delle partite
//! analizzate, tentativi dei puzzle) e calcola le statistiche della sezione
//! «Progressi». Il punto d'ingresso è [`build_report`].
//!
//! Organizzazione (un file per responsabilità):
//! - [`stats_input`]: i dati grezzi;
//! - [`stats_view`]: le statistiche pronte per l'interfaccia;
//! - [`judged_move`]: le mosse della persona con il loro giudizio (come nell'analisi);
//! - [`game_score`]: il risultato di una partita dal punto di vista della persona;
//! - [`level_stats`]: Elo stimato, rating dei puzzle, precisione nel tempo;
//! - [`game_stats`]: risultati, errori per fase, errori con poco tempo;
//! - [`opening_stats`]: le aperture più giocate;
//! - [`tactics_stats`]: puzzle per profondità e per tema;
//! - [`report`]: tutto insieme.

pub mod game_score;
pub mod game_stats;
pub mod judged_move;
pub mod level_stats;
pub mod opening_stats;
pub mod report;
pub mod stats_input;
pub mod stats_view;
pub mod tactics_stats;

pub use report::build_report;
pub use stats_input::{GameFacts, MoveFacts, PuzzleFacts, StatsInput};
pub use stats_view::{
    ChartPoint, ColorScore, DepthStats, GameStats, LevelScore, LevelStats, OpeningStats,
    PhaseStats, Score, StatsReport, TacticsStats, ThemeStats, TimePressure,
};
