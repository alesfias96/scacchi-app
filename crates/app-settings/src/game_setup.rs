//! Come si vuole giocare la prossima partita.

use crate::color_choice::ColorChoice;
use crate::opponent_kind::OpponentKind;
use crate::time_control::TimeControl;

/// Il livello del computer proposto la prima volta: "Circolo", circa 1400 Elo.
const DEFAULT_LEVEL: u8 = 4;
/// Il tempo proposto la prima volta: 10 minuti a testa, senza incremento.
const DEFAULT_TIME: TimeControl = TimeControl {
    initial_seconds: 600,
    increment_seconds: 0,
};

/// Le scelte fatte nella finestra "Nuova partita".
///
/// Viene salvata con le impostazioni, così alla partita successiva la
/// finestra propone le stesse scelte.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS), ts(export))]
pub struct GameSetup {
    /// Contro chi si gioca.
    pub opponent: OpponentKind,
    /// Il colore della persona (conta solo contro il computer).
    pub color: ColorChoice,
    /// Il livello del computer, da 1 (principiante) a 10 (sovrumano).
    pub level: u8,
    /// Il tempo di riflessione; `None` = senza orologio.
    pub time_control: Option<TimeControl>,
}

impl Default for GameSetup {
    /// Contro il computer, con il bianco, livello Circolo, 10 minuti.
    fn default() -> Self {
        Self {
            opponent: OpponentKind::default(),
            color: ColorChoice::default(),
            level: DEFAULT_LEVEL,
            time_control: Some(DEFAULT_TIME),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GameSetup;
    use crate::{ColorChoice, OpponentKind, TimeControl};

    #[test]
    fn is_written_as_readable_json() {
        let setup = GameSetup {
            opponent: OpponentKind::TwoPlayers,
            color: ColorChoice::Random,
            level: 7,
            time_control: Some(TimeControl {
                initial_seconds: 180,
                increment_seconds: 2,
            }),
        };

        let json = serde_json::to_string(&setup).expect("conversione in JSON");

        assert_eq!(
            json,
            r#"{"opponent":"twoPlayers","color":"random","level":7,"timeControl":{"initialSeconds":180,"incrementSeconds":2}}"#
        );
    }

    #[test]
    fn an_untimed_game_has_no_time_control() {
        let setup: GameSetup =
            serde_json::from_str(r#"{"timeControl":null}"#).expect("JSON valido");

        assert_eq!(setup.time_control, None);
        assert_eq!(setup.level, GameSetup::default().level);
    }
}
