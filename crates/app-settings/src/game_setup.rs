//! Come si vuole giocare la prossima partita.

use crate::color_choice::ColorChoice;
use crate::computer_engine::ComputerEngine;
use crate::opponent_kind::OpponentKind;
use crate::time_control::TimeControl;

/// Il livello del computer proposto la prima volta: "Circolo", circa 1400 Elo.
const DEFAULT_LEVEL: u8 = 4;
/// Il livello di Maia proposto la prima volta: la rete addestrata su giocatori da 1500 Elo
/// (i livelli di Maia vanno da 1 = 1100 a 9 = 1900, vedi la crate `maia`).
const DEFAULT_MAIA_LEVEL: u8 = 5;
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
    /// Il motore del computer: Stockfish o Maia.
    pub engine: ComputerEngine,
    /// Il livello di Stockfish, da 1 (principiante) a 10 (sovrumano).
    pub level: u8,
    /// Il livello di Maia, da 1 (1100 Elo) a 9 (1900 Elo): si ricorda a parte,
    /// così passando da un motore all'altro ognuno ritrova il suo.
    pub maia_level: u8,
    /// Il tempo di riflessione; `None` = senza orologio.
    pub time_control: Option<TimeControl>,
}

impl Default for GameSetup {
    /// Contro il computer, con il bianco, livello Circolo, 10 minuti.
    fn default() -> Self {
        Self {
            opponent: OpponentKind::default(),
            color: ColorChoice::default(),
            engine: ComputerEngine::default(),
            level: DEFAULT_LEVEL,
            maia_level: DEFAULT_MAIA_LEVEL,
            time_control: Some(DEFAULT_TIME),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GameSetup;
    use crate::{ColorChoice, ComputerEngine, OpponentKind, TimeControl};

    #[test]
    fn is_written_as_readable_json() {
        let setup = GameSetup {
            opponent: OpponentKind::TwoPlayers,
            color: ColorChoice::Random,
            engine: ComputerEngine::Maia,
            level: 7,
            maia_level: 3,
            time_control: Some(TimeControl {
                initial_seconds: 180,
                increment_seconds: 2,
            }),
        };

        let json = serde_json::to_string(&setup).expect("conversione in JSON");

        assert_eq!(
            json,
            r#"{"opponent":"twoPlayers","color":"random","engine":"maia","level":7,"maiaLevel":3,"timeControl":{"initialSeconds":180,"incrementSeconds":2}}"#
        );
    }

    #[test]
    fn an_untimed_game_has_no_time_control() {
        let setup: GameSetup =
            serde_json::from_str(r#"{"timeControl":null}"#).expect("JSON valido");

        assert_eq!(setup.time_control, None);
        assert_eq!(setup.level, GameSetup::default().level);
        assert_eq!(
            setup.engine,
            ComputerEngine::Stockfish,
            "le impostazioni vecchie restano con Stockfish"
        );
    }
}
