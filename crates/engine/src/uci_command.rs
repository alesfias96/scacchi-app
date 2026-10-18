//! I comandi che si mandano al motore.

use std::fmt;

use crate::search_limits::SearchLimits;
use crate::search_position::SearchPosition;

/// Un comando del protocollo UCI, che diventa una riga di testo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UciCommand {
    /// `uci`: "presentati" (il motore elenca nome e impostazioni, poi `uciok`).
    Uci,
    /// `isready`: "sei pronto?" (il motore risponde `readyok`).
    IsReady,
    /// `setoption name <nome> value <valore>`: cambia un'impostazione.
    SetOption {
        /// Il nome dell'impostazione (es. `"UCI_Elo"`).
        name: String,
        /// Il nuovo valore, come testo.
        value: String,
    },
    /// `position ...`: la posizione su cui pensare.
    Position(SearchPosition),
    /// `go ...`: comincia a pensare, entro i limiti indicati.
    Go(SearchLimits),
    /// `stop`: smetti di pensare e dai subito la mossa migliore trovata.
    Stop,
    /// `quit`: chiuditi.
    Quit,
}

impl fmt::Display for UciCommand {
    /// Scrive il comando come riga di testo (senza "a capo").
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Uci => formatter.write_str("uci"),
            Self::IsReady => formatter.write_str("isready"),
            Self::SetOption { name, value } => {
                write!(formatter, "setoption name {name} value {value}")
            }
            Self::Position(position) => position.fmt(formatter),
            Self::Go(limits) => limits.fmt(formatter),
            Self::Stop => formatter.write_str("stop"),
            Self::Quit => formatter.write_str("quit"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::UciCommand;

    #[test]
    fn writes_simple_commands() {
        assert_eq!(UciCommand::Uci.to_string(), "uci");
        assert_eq!(UciCommand::Stop.to_string(), "stop");
    }

    #[test]
    fn writes_an_option_with_spaces_in_the_name() {
        let command = UciCommand::SetOption {
            name: "Skill Level".to_owned(),
            value: "5".to_owned(),
        };

        assert_eq!(command.to_string(), "setoption name Skill Level value 5");
    }
}
