//! Il livello del computer, per ciascuno dei due motori.
//!
//! Stockfish ha 10 livelli (da "Principiante" a "Sovrumano", crate `engine`),
//! Maia ne ha 9 (da 1100 a 1900 Elo, crate `maia`). Il numero del livello da
//! solo non basta: il livello 5 di Stockfish e il livello 5 di Maia sono cose
//! diverse, per questo il motore va sempre insieme al livello.

use app_settings::ComputerEngine;
use engine::LevelInfo;

/// Riporta il livello nei limiti del motore (es. 12 → 10 per Stockfish, 9 per Maia).
#[must_use]
pub fn clamp_level(computer_engine: ComputerEngine, level: u8) -> u8 {
    match computer_engine {
        ComputerEngine::Stockfish => level.clamp(engine::MIN_LEVEL, engine::MAX_LEVEL),
        ComputerEngine::Maia => level.clamp(maia::MIN_LEVEL, maia::MAX_LEVEL),
    }
}

/// Il livello con il suo nome e la sua forza indicativa (es. "Circolo", "Maia 1500").
#[must_use]
pub fn computer_level(computer_engine: ComputerEngine, level: u8) -> LevelInfo {
    let level = clamp_level(computer_engine, level);
    match computer_engine {
        ComputerEngine::Stockfish => engine::levels()
            .into_iter()
            .find(|info| info.level == level)
            .unwrap_or_else(|| engine::levels().remove(0)),
        ComputerEngine::Maia => maia::level_info(level),
    }
}

#[cfg(test)]
mod tests {
    use app_settings::ComputerEngine;

    use super::{clamp_level, computer_level};

    #[test]
    fn each_engine_has_its_own_levels() {
        assert_eq!(computer_level(ComputerEngine::Stockfish, 4).name, "Circolo");
        assert_eq!(computer_level(ComputerEngine::Maia, 5).elo, 1500);
        assert_eq!(clamp_level(ComputerEngine::Maia, 10), 9);
        assert_eq!(clamp_level(ComputerEngine::Stockfish, 10), 10);
    }
}
