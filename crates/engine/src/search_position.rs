//! La posizione da far analizzare al motore.

use std::fmt;

/// Una posizione descritta come "partenza + mosse giocate".
///
/// Al motore si manda tutta la storia della partita, non solo la posizione
/// finale: così riconosce le ripetizioni e non propone mosse che ripetono la
/// posizione quando sta vincendo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchPosition {
    /// La posizione di partenza della partita, in notazione FEN.
    pub start_fen: String,
    /// Le mosse giocate da lì, in notazione UCI.
    pub moves: Vec<String>,
}

impl fmt::Display for SearchPosition {
    /// Scrive il comando UCI: `position fen <fen> moves <mosse>`.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "position fen {}", self.start_fen)?;
        if !self.moves.is_empty() {
            write!(formatter, " moves {}", self.moves.join(" "))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::SearchPosition;

    const START: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    #[test]
    fn writes_the_position_command() {
        let position = SearchPosition {
            start_fen: START.to_owned(),
            moves: vec!["e2e4".to_owned(), "e7e5".to_owned()],
        };

        assert_eq!(
            position.to_string(),
            format!("position fen {START} moves e2e4 e7e5")
        );
    }

    #[test]
    fn leaves_out_the_moves_when_there_are_none() {
        let position = SearchPosition {
            start_fen: START.to_owned(),
            moves: Vec::new(),
        };

        assert_eq!(position.to_string(), format!("position fen {START}"));
    }
}
