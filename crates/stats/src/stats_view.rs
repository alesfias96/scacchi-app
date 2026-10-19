//! Le statistiche pronte per la sezione «Progressi».

use analysis::GamePhase;
use app_settings::ComputerEngine;
use chess_core::Side;

/// Un punto di un grafico nel tempo.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct ChartPoint {
    /// Quando (millesimi di secondo dal 1970).
    pub time_ms: f64,
    /// Il valore.
    pub value: f64,
}

/// Vittorie, patte e sconfitte.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct Score {
    /// Le partite vinte.
    pub wins: u32,
    /// Le partite patte.
    pub draws: u32,
    /// Le partite perse.
    pub losses: u32,
}

impl Score {
    /// Quante partite in tutto.
    #[must_use]
    pub fn games(self) -> u32 {
        self.wins + self.draws + self.losses
    }
}

/// La scheda «Livello».
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct LevelStats {
    /// L'Elo stimato dai risultati contro i livelli del computer (con almeno 5 partite).
    pub estimated_elo: Option<i32>,
    /// Su quante partite contro il computer si basa l'Elo stimato.
    pub elo_games: u32,
    /// Il rating dei puzzle attuale.
    pub puzzle_rating: i32,
    /// Il rating dei puzzle dopo ogni tentativo.
    pub puzzle_history: Vec<ChartPoint>,
    /// La precisione media nelle partite analizzate, da 0 a 100.
    pub accuracy: Option<u32>,
    /// La precisione di ogni partita analizzata.
    pub accuracy_history: Vec<ChartPoint>,
}

/// I risultati con un colore.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct ColorScore {
    /// Il colore giocato.
    pub side: Side,
    /// I risultati.
    pub score: Score,
}

/// I risultati contro un livello del computer.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct LevelScore {
    /// Il motore: Stockfish o Maia.
    pub engine: ComputerEngine,
    /// Il livello (da 1 a 10 per Stockfish, da 1 a 9 per Maia).
    pub level: u8,
    /// Il nome del livello (es. "Circolo", "Maia 1500").
    pub name: String,
    /// L'Elo indicativo del livello.
    pub elo: u32,
    /// I risultati.
    pub score: Score,
}

/// Come si gioca in una fase della partita.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct PhaseStats {
    /// La fase.
    pub phase: GamePhase,
    /// Quante mosse giocate in questa fase.
    pub moves: u32,
    /// La precisione media, da 0 a 100.
    pub accuracy: Option<u32>,
    /// Le imprecisioni, gli errori e gli errori gravi.
    pub inaccuracies: u32,
    /// Vedi `inaccuracies`.
    pub mistakes: u32,
    /// Vedi `inaccuracies`.
    pub blunders: u32,
}

/// Gli errori con poco tempo sull'orologio (meno di un quinto del tempo iniziale).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct TimePressure {
    /// Le mosse giocate con poco tempo e quante erano errori (anche imprecisioni).
    pub low_time_moves: u32,
    /// Vedi `low_time_moves`.
    pub low_time_errors: u32,
    /// Le altre mosse con l'orologio e quante erano errori.
    pub other_moves: u32,
    /// Vedi `other_moves`.
    pub other_errors: u32,
}

/// La scheda «Partite».
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct GameStats {
    /// I risultati contro il computer.
    pub against_computer: Score,
    /// Quante partite a due giocatori.
    pub two_player_games: u32,
    /// I risultati contro il computer con il Bianco e con il Nero.
    pub by_color: Vec<ColorScore>,
    /// I risultati per livello del computer.
    pub by_level: Vec<LevelScore>,
    /// Come si gioca in apertura, nel mediogioco e nel finale (partite analizzate).
    pub phases: Vec<PhaseStats>,
    /// Gli errori con poco tempo (partite analizzate con l'orologio), se ce ne sono.
    pub time_pressure: Option<TimePressure>,
}

/// Un'apertura giocata.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct OpeningStats {
    /// La famiglia dell'apertura (nome di Lichess, es. "Sicilian Defense").
    pub name: String,
    /// Il colore giocato.
    pub side: Side,
    /// I risultati (contro il computer).
    pub score: Score,
    /// Quante partite in tutto con questa apertura e questo colore.
    pub games: u32,
    /// La precisione media nelle partite analizzate, da 0 a 100.
    pub accuracy: Option<u32>,
}

/// I puzzle con un certo numero di mosse da trovare.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct DepthStats {
    /// Le mosse da trovare.
    pub depth: u32,
    /// I tentativi e quanti risolti.
    pub attempts: u32,
    /// Vedi `attempts`.
    pub solved: u32,
}

/// I puzzle di un tema.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct ThemeStats {
    /// Il nome di Lichess del tema.
    pub key: String,
    /// Il nome in italiano.
    pub name: String,
    /// I tentativi e quanti risolti.
    pub attempts: u32,
    /// Vedi `attempts`.
    pub solved: u32,
}

/// La scheda «Calcolo e tattica».
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct TacticsStats {
    /// I puzzle tentati e risolti (senza i ripassi).
    pub attempts: u32,
    /// Vedi `attempts`.
    pub solved: u32,
    /// Risultati per numero di mosse da trovare.
    pub by_depth: Vec<DepthStats>,
    /// La profondità media dei puzzle risolti, giorno per giorno.
    pub depth_history: Vec<ChartPoint>,
    /// I temi in cui si sbaglia di più (almeno 3 tentativi).
    pub weakest_themes: Vec<ThemeStats>,
    /// Quanti puzzle sono da ripassare oggi.
    pub due_reviews: u32,
}

/// Tutte le statistiche della sezione «Progressi».
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct StatsReport {
    /// Quante partite nel periodo.
    pub games: u32,
    /// Quante di queste sono analizzate.
    pub analyzed_games: u32,
    /// Quante partite finite non sono ancora analizzate (nel periodo).
    pub unanalyzed_games: u32,
    /// La scheda «Livello».
    pub level: LevelStats,
    /// La scheda «Partite».
    pub results: GameStats,
    /// La scheda «Aperture»: le più giocate, per colore.
    pub openings: Vec<OpeningStats>,
    /// La scheda «Calcolo e tattica».
    pub tactics: TacticsStats,
}
