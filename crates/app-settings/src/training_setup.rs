//! Le scelte dell'ultimo allenamento.

use crate::difficulty::Difficulty;
use crate::training_mode::TrainingMode;

/// La profondità proposta la prima volta: 2 mosse da trovare.
const DEFAULT_DEPTH: u8 = 2;
/// Per quanti secondi si vede la posizione, la prima volta, nella visualizzazione.
const DEFAULT_VISUALIZATION_SECONDS: u8 = 5;

/// Le scelte fatte nella sezione "Allenamento".
///
/// Viene salvata con le impostazioni, così al prossimo allenamento la sezione
/// propone le stesse scelte.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS), ts(export))]
pub struct TrainingSetup {
    /// La modalità.
    pub mode: TrainingMode,
    /// Quante mosse trovare (da 1 a 6), per la modalità a profondità.
    pub depth: u8,
    /// Il tema dei puzzle (es. `"fork"`); `None` = tutti i temi.
    pub theme: Option<String>,
    /// La difficoltà rispetto al proprio livello.
    pub difficulty: Difficulty,
    /// Per quanti secondi si vede la posizione nella visualizzazione (da 2 a 15).
    pub visualization_seconds: u8,
}

impl Default for TrainingSetup {
    /// Puzzle a profondità 2, di tutti i temi, difficoltà normale, 5 secondi.
    fn default() -> Self {
        Self {
            mode: TrainingMode::default(),
            depth: DEFAULT_DEPTH,
            theme: None,
            difficulty: Difficulty::default(),
            visualization_seconds: DEFAULT_VISUALIZATION_SECONDS,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TrainingSetup;
    use crate::{Difficulty, TrainingMode};

    #[test]
    fn is_written_as_readable_json() {
        let setup = TrainingSetup {
            mode: TrainingMode::Visualization,
            depth: 3,
            theme: Some("fork".to_owned()),
            difficulty: Difficulty::Harder,
            visualization_seconds: 8,
        };

        let json = serde_json::to_string(&setup).expect("conversione in JSON");

        assert_eq!(
            json,
            r#"{"mode":"visualization","depth":3,"theme":"fork","difficulty":"harder","visualizationSeconds":8}"#
        );
    }

    #[test]
    fn missing_values_take_the_default() {
        let setup: TrainingSetup = serde_json::from_str("{}").expect("JSON valido");

        assert_eq!(setup, TrainingSetup::default());
    }
}
