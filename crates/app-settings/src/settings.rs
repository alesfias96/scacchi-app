//! L'insieme delle impostazioni dell'utente.

use crate::app_theme::AppTheme;
use crate::board_style::BoardStyle;
use crate::game_setup::GameSetup;
use crate::notation::Notation;
use crate::piece_style::PieceStyle;
use crate::sound_setting::SoundSetting;

/// Tutte le scelte dell'utente che valgono in ogni sezione dell'app.
///
/// `#[serde(default)]`: se nel file manca un valore (ad esempio perché
/// l'impostazione è stata aggiunta in una versione successiva dell'app),
/// viene usato quello predefinito invece di scartare tutto il file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS), ts(export))]
pub struct Settings {
    /// L'aspetto dell'interfaccia.
    pub theme: AppTheme,
    /// Lo stile della scacchiera.
    pub board: BoardStyle,
    /// Lo stile dei pezzi.
    pub pieces: PieceStyle,
    /// Il modo di scrivere le mosse nella lista della partita.
    pub notation: Notation,
    /// Se suonano mosse, prese e fine partita.
    pub sounds: SoundSetting,
    /// L'ultima configurazione scelta per una nuova partita.
    pub game_setup: GameSetup,
}

#[cfg(test)]
mod tests {
    use super::Settings;
    use crate::{AppTheme, BoardStyle, GameSetup, Notation, PieceStyle, SoundSetting};

    #[test]
    fn the_defaults_are_studio_rovere_icona_figurine_with_sounds() {
        let settings = Settings::default();

        assert_eq!(settings.theme, AppTheme::Studio);
        assert_eq!(settings.board, BoardStyle::Rovere);
        assert_eq!(settings.pieces, PieceStyle::Icona);
        assert_eq!(settings.notation, Notation::Figurine);
        assert_eq!(settings.sounds, SoundSetting::On);
    }

    #[test]
    fn is_written_as_readable_json() {
        let settings = Settings {
            theme: AppTheme::Sera,
            board: BoardStyle::Marmo,
            pieces: PieceStyle::Gettoni,
            notation: Notation::Italiana,
            sounds: SoundSetting::Off,
            game_setup: GameSetup::default(),
        };

        let json = serde_json::to_string(&settings).expect("conversione in JSON");

        assert_eq!(
            json,
            r#"{"theme":"sera","board":"marmo","pieces":"gettoni","notation":"italiana","sounds":"off","gameSetup":{"opponent":"computer","color":"white","level":4,"timeControl":{"initialSeconds":600,"incrementSeconds":0}}}"#
        );
    }

    #[test]
    fn missing_values_take_the_default() {
        let settings: Settings =
            serde_json::from_str(r#"{"theme":"giorno"}"#).expect("JSON valido");

        assert_eq!(settings.theme, AppTheme::Giorno);
        assert_eq!(settings.board, BoardStyle::Rovere);
        assert_eq!(settings.pieces, PieceStyle::Icona);
        assert_eq!(settings.notation, Notation::Figurine);
        assert_eq!(settings.sounds, SoundSetting::On);
    }
}
