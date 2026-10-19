//! Lettura e scrittura del file delle impostazioni (formato JSON).

use std::fs;
use std::io::ErrorKind;
use std::path::Path;

use crate::settings::Settings;
use crate::store_error::StoreError;

/// Legge le impostazioni dal file indicato.
///
/// - Riceve: il percorso del file.
/// - Restituisce: le impostazioni lette; quelle predefinite se il file
///   non esiste ancora (primo avvio dell'app).
///
/// # Errors
///
/// - [`StoreError::Io`] se il file esiste ma non si riesce a leggerlo;
/// - [`StoreError::Format`] se il contenuto non è un JSON valido.
pub fn load(path: &Path) -> Result<Settings, StoreError> {
    match fs::read_to_string(path) {
        Ok(text) => Ok(serde_json::from_str(&text)?),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(Settings::default()),
        Err(error) => Err(StoreError::Io(error)),
    }
}

/// Legge le impostazioni e, in caso di qualsiasi problema, usa quelle predefinite.
///
/// È la funzione usata all'avvio: un file rovinato non deve mai impedire
/// all'app di aprirsi. Il problema viene segnalato nel terminale.
#[must_use]
pub fn load_or_default(path: &Path) -> Settings {
    load(path).unwrap_or_else(|error| {
        eprintln!("Impostazioni ignorate, uso quelle predefinite: {error}");
        Settings::default()
    })
}

/// Salva le impostazioni nel file indicato, creando la cartella se serve.
///
/// # Errors
///
/// [`StoreError::Io`] se non si riesce a creare la cartella o a scrivere il file.
pub fn save(path: &Path, settings: &Settings) -> Result<(), StoreError> {
    if let Some(folder) = path.parent() {
        fs::create_dir_all(folder)?;
    }
    let text = serde_json::to_string_pretty(settings)?;
    fs::write(path, text)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{load, load_or_default, save};
    use crate::{
        AppTheme, BoardStyle, GameSetup, Notation, PieceStyle, Settings, SoundSetting, StoreError,
        TrainingSetup,
    };
    use std::path::PathBuf;

    /// Una cartella temporanea diversa per ogni test, così i test non si disturbano.
    fn temp_file(test_name: &str) -> PathBuf {
        std::env::temp_dir()
            .join(format!(
                "scacchi-app-settings-{}-{test_name}",
                std::process::id()
            ))
            .join("settings.json")
    }

    #[test]
    fn a_missing_file_gives_the_defaults() {
        let path = temp_file("missing");

        assert_eq!(load(&path).expect("nessun errore"), Settings::default());
    }

    #[test]
    fn saved_settings_are_read_back_identical() {
        let path = temp_file("roundtrip");
        let settings = Settings {
            theme: AppTheme::Giorno,
            board: BoardStyle::Ardesia,
            pieces: PieceStyle::Metallo,
            notation: Notation::Internazionale,
            sounds: SoundSetting::Off,
            game_setup: GameSetup::default(),
            training_setup: TrainingSetup::default(),
        };

        save(&path, &settings).expect("salvataggio riuscito");

        assert_eq!(load(&path).expect("lettura riuscita"), settings);
    }

    #[test]
    fn a_damaged_file_is_reported_as_an_error() {
        let path = temp_file("damaged");
        std::fs::create_dir_all(path.parent().expect("cartella")).expect("cartella creata");
        std::fs::write(&path, "non è JSON").expect("file scritto");

        assert!(matches!(load(&path), Err(StoreError::Format(_))));
    }

    #[test]
    fn a_damaged_file_never_blocks_the_app() {
        let path = temp_file("fallback");
        std::fs::create_dir_all(path.parent().expect("cartella")).expect("cartella creata");
        std::fs::write(&path, "non è JSON").expect("file scritto");

        assert_eq!(load_or_default(&path), Settings::default());
    }
}
