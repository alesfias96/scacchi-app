//! # app-settings — le impostazioni dell'app
//!
//! Le scelte dell'utente che valgono in tutta l'app:
//! - [`AppTheme`]: l'aspetto dell'interfaccia (Sera, Giorno, Studio o automatico);
//! - [`BoardStyle`]: il materiale della scacchiera (legni, marmo, ardesia, torneo);
//! - [`PieceStyle`]: lo stile dei pezzi;
//! - [`Notation`]: il modo di scrivere le mosse;
//! - [`SoundSetting`]: se suonano mosse, prese e fine partita;
//! - [`GameSetup`]: l'ultima configurazione di partita scelta (avversario,
//!   [`ColorChoice`], livello, [`TimeControl`]);
//! - [`TrainingSetup`]: l'ultima configurazione di allenamento scelta
//!   ([`TrainingMode`], profondità, tema, [`Difficulty`], secondi di visualizzazione).
//!
//! [`Settings`] le raccoglie; il modulo [`store`] le salva e le rilegge da un file.
//!
//! Organizzazione (un file per responsabilità):
//! - [`app_theme`], [`board_style`], [`piece_style`], [`notation`], [`sound_setting`]:
//!   le singole scelte;
//! - [`game_setup`], [`opponent_kind`], [`color_choice`], [`time_control`]: la
//!   configurazione della partita;
//! - [`training_setup`], [`training_mode`], [`difficulty`]: la configurazione dell'allenamento;
//! - [`stats_filter`]: i filtri della sezione «Progressi»;
//! - [`settings`]: l'insieme delle scelte e i valori predefiniti;
//! - [`store`]: lettura e scrittura del file;
//! - [`store_error`]: gli errori possibili durante la scrittura.

pub mod app_theme;
pub mod board_style;
pub mod color_choice;
pub mod difficulty;
pub mod game_setup;
pub mod notation;
pub mod opponent_kind;
pub mod piece_style;
pub mod settings;
pub mod sound_setting;
pub mod stats_filter;
pub mod store;
pub mod store_error;
pub mod time_control;
pub mod training_mode;
pub mod training_setup;

pub use app_theme::AppTheme;
pub use board_style::BoardStyle;
pub use color_choice::ColorChoice;
pub use difficulty::Difficulty;
pub use game_setup::GameSetup;
pub use notation::Notation;
pub use opponent_kind::OpponentKind;
pub use piece_style::PieceStyle;
pub use settings::Settings;
pub use sound_setting::SoundSetting;
pub use stats_filter::{StatsFilter, StatsOpponent, StatsPeriod};
pub use store::{load, load_or_default, save};
pub use store_error::StoreError;
pub use time_control::TimeControl;
pub use training_mode::TrainingMode;
pub use training_setup::TrainingSetup;
