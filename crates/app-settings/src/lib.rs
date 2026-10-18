//! # app-settings — le impostazioni dell'app
//!
//! Le scelte dell'utente che valgono in tutta l'app:
//! - [`AppTheme`]: l'aspetto dell'interfaccia (Sera, Giorno, Studio o automatico);
//! - [`BoardStyle`]: il materiale della scacchiera (legni, marmo, ardesia, torneo);
//! - [`PieceStyle`]: lo stile dei pezzi;
//! - [`Notation`]: il modo di scrivere le mosse;
//! - [`SoundSetting`]: se suonano mosse, prese e fine partita.
//!
//! [`Settings`] le raccoglie; il modulo [`store`] le salva e le rilegge da un file.
//!
//! Organizzazione (un file per responsabilità):
//! - [`app_theme`], [`board_style`], [`piece_style`], [`notation`], [`sound_setting`]:
//!   le singole scelte;
//! - [`settings`]: l'insieme delle scelte e i valori predefiniti;
//! - [`store`]: lettura e scrittura del file;
//! - [`store_error`]: gli errori possibili durante la scrittura.

pub mod app_theme;
pub mod board_style;
pub mod notation;
pub mod piece_style;
pub mod settings;
pub mod sound_setting;
pub mod store;
pub mod store_error;

pub use app_theme::AppTheme;
pub use board_style::BoardStyle;
pub use notation::Notation;
pub use piece_style::PieceStyle;
pub use settings::Settings;
pub use sound_setting::SoundSetting;
pub use store::{load, load_or_default, save};
pub use store_error::StoreError;
