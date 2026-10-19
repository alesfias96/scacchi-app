//! I livelli di Maia: nove reti, addestrate su partite di persone da 1100 a 1900 Elo.

use engine::LevelInfo;

/// Il livello più basso (la rete maia-1100).
pub const MIN_LEVEL: u8 = 1;
/// Il livello più alto (la rete maia-1900).
pub const MAX_LEVEL: u8 = 9;
/// Il livello proposto la prima volta (maia-1500).
pub const DEFAULT_LEVEL: u8 = 5;

/// L'Elo delle persone con cui è stata addestrata la rete di quel livello.
#[must_use]
pub fn level_elo(level: u8) -> u32 {
    1000 + 100 * u32::from(level.clamp(MIN_LEVEL, MAX_LEVEL))
}

/// Il livello con il suo nome e la sua forza indicativa (es. "Maia 1500", 1500).
#[must_use]
pub fn level_info(level: u8) -> LevelInfo {
    let level = level.clamp(MIN_LEVEL, MAX_LEVEL);
    LevelInfo {
        level,
        name: format!("Maia {}", level_elo(level)),
        elo: level_elo(level),
    }
}

/// Tutti i livelli, dal più debole al più forte.
#[must_use]
pub fn levels() -> Vec<LevelInfo> {
    (MIN_LEVEL..=MAX_LEVEL).map(level_info).collect()
}

#[cfg(test)]
mod tests {
    use super::{level_info, levels};

    #[test]
    fn nine_levels_from_1100_to_1900() {
        let all = levels();

        assert_eq!(all.len(), 9);
        assert_eq!(all[0].elo, 1100);
        assert_eq!(all[8].name, "Maia 1900");
        assert_eq!(level_info(42).elo, 1900, "fuori misura torna nei limiti");
    }
}
