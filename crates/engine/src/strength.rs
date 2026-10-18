//! I livelli di gioco del computer, dal principiante al sovrumano.
//!
//! Stockfish al massimo è molto più forte di qualsiasi persona. Per farlo
//! giocare come una persona di un certo livello si usano due strumenti:
//! - sopra i 1320 Elo l'impostazione ufficiale `UCI_LimitStrength` con
//!   `UCI_Elo`, che lo fa giocare (e sbagliare) come un giocatore di quel valore;
//! - sotto, dove `UCI_Elo` non arriva, lo `Skill Level` basso insieme a una
//!   profondità di calcolo molto ridotta: "vede" poche mosse avanti.
//!
//! Ogni livello imposta **tutte** le opzioni, così il passaggio da un livello
//! all'altro non lascia mai un'impostazione del livello precedente.
//! I valori Elo sono indicativi.

/// Il livello più basso.
pub const MIN_LEVEL: u8 = 1;
/// Il livello più alto (piena forza).
pub const MAX_LEVEL: u8 = 10;

/// Nome e forza di un livello, da mostrare nella scelta della partita.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct LevelInfo {
    /// Il numero del livello, da [`MIN_LEVEL`] a [`MAX_LEVEL`].
    pub level: u8,
    /// Il nome del livello (es. `"Circolo"`).
    pub name: String,
    /// La forza indicativa, in punti Elo.
    pub elo: u32,
}

/// Come far giocare il motore a un certo livello.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Strength {
    /// Le impostazioni da mandare al motore prima di ogni ricerca: (nome, valore).
    pub options: Vec<(String, String)>,
    /// La profondità massima di calcolo, per i livelli più bassi.
    pub depth: Option<u32>,
    /// Il tempo per mossa quando la partita non ha l'orologio, in millisecondi.
    pub move_time_ms: u64,
}

/// La descrizione di un livello.
struct LevelSpec {
    /// Il nome mostrato.
    name: &'static str,
    /// La forza indicativa.
    elo: u32,
    /// Lo `Skill Level` di Stockfish (0–20).
    skill: u8,
    /// Il valore di `UCI_Elo`, per i livelli che limitano la forza in questo modo.
    limit_elo: Option<u32>,
    /// La profondità massima, per i livelli più bassi.
    depth: Option<u32>,
    /// Il tempo per mossa senza orologio.
    move_time_ms: u64,
}

/// I dieci livelli, dal più debole al più forte.
const LEVELS: [LevelSpec; 10] = [
    LevelSpec {
        name: "Principiante",
        elo: 800,
        skill: 0,
        limit_elo: None,
        depth: Some(1),
        move_time_ms: 200,
    },
    LevelSpec {
        name: "Esordiente",
        elo: 1000,
        skill: 2,
        limit_elo: None,
        depth: Some(3),
        move_time_ms: 300,
    },
    LevelSpec {
        name: "Dilettante",
        elo: 1200,
        skill: 5,
        limit_elo: None,
        depth: Some(5),
        move_time_ms: 400,
    },
    LevelSpec {
        name: "Circolo",
        elo: 1400,
        skill: 20,
        limit_elo: Some(1400),
        depth: None,
        move_time_ms: 500,
    },
    LevelSpec {
        name: "Agonista",
        elo: 1600,
        skill: 20,
        limit_elo: Some(1600),
        depth: None,
        move_time_ms: 600,
    },
    LevelSpec {
        name: "Esperto",
        elo: 1800,
        skill: 20,
        limit_elo: Some(1800),
        depth: None,
        move_time_ms: 700,
    },
    LevelSpec {
        name: "Candidato maestro",
        elo: 2000,
        skill: 20,
        limit_elo: Some(2000),
        depth: None,
        move_time_ms: 800,
    },
    LevelSpec {
        name: "Maestro",
        elo: 2300,
        skill: 20,
        limit_elo: Some(2300),
        depth: None,
        move_time_ms: 1000,
    },
    LevelSpec {
        name: "Grande maestro",
        elo: 2600,
        skill: 20,
        limit_elo: Some(2600),
        depth: None,
        move_time_ms: 1500,
    },
    LevelSpec {
        name: "Sovrumano",
        elo: 3500,
        skill: 20,
        limit_elo: None,
        depth: None,
        move_time_ms: 2000,
    },
];

/// L'elenco dei livelli, dal più debole al più forte.
#[must_use]
pub fn levels() -> Vec<LevelInfo> {
    (MIN_LEVEL..=MAX_LEVEL)
        .zip(&LEVELS)
        .map(|(level, spec)| LevelInfo {
            level,
            name: spec.name.to_owned(),
            elo: spec.elo,
        })
        .collect()
}

/// Come far giocare il motore al livello indicato.
///
/// - Riceve: il livello; un numero fuori dall'intervallo viene riportato al
///   livello valido più vicino.
/// - Restituisce: impostazioni, profondità e tempo per mossa.
#[must_use]
pub fn strength(level: u8) -> Strength {
    let index = usize::from(level.clamp(MIN_LEVEL, MAX_LEVEL) - MIN_LEVEL);
    let spec = &LEVELS[index];
    let limit = spec.limit_elo.is_some();
    let mut options = vec![
        ("Skill Level".to_owned(), spec.skill.to_string()),
        ("UCI_LimitStrength".to_owned(), limit.to_string()),
    ];
    if let Some(elo) = spec.limit_elo {
        options.push(("UCI_Elo".to_owned(), elo.to_string()));
    }
    Strength {
        options,
        depth: spec.depth,
        move_time_ms: spec.move_time_ms,
    }
}

#[cfg(test)]
mod tests {
    use super::{MAX_LEVEL, MIN_LEVEL, levels, strength};

    #[test]
    fn lists_ten_levels_from_weakest_to_strongest() {
        let all = levels();

        assert_eq!(all.len(), 10);
        assert_eq!(all[0].level, MIN_LEVEL);
        assert_eq!(all[9].level, MAX_LEVEL);
        assert!(all.windows(2).all(|pair| pair[0].elo < pair[1].elo));
    }

    #[test]
    fn low_levels_limit_the_depth() {
        let beginner = strength(1);

        assert_eq!(beginner.depth, Some(1));
        assert!(
            beginner
                .options
                .contains(&("Skill Level".to_owned(), "0".to_owned()))
        );
        assert!(
            beginner
                .options
                .contains(&("UCI_LimitStrength".to_owned(), "false".to_owned()))
        );
    }

    #[test]
    fn middle_levels_use_the_official_elo_limit() {
        let club = strength(4);

        assert_eq!(club.depth, None);
        assert!(
            club.options
                .contains(&("UCI_LimitStrength".to_owned(), "true".to_owned()))
        );
        assert!(
            club.options
                .contains(&("UCI_Elo".to_owned(), "1400".to_owned()))
        );
    }

    #[test]
    fn the_top_level_plays_at_full_strength() {
        let full = strength(MAX_LEVEL);

        assert!(
            full.options
                .contains(&("Skill Level".to_owned(), "20".to_owned()))
        );
        assert!(
            full.options
                .contains(&("UCI_LimitStrength".to_owned(), "false".to_owned()))
        );
    }

    #[test]
    fn levels_out_of_range_are_brought_back() {
        assert_eq!(strength(0), strength(MIN_LEVEL));
        assert_eq!(strength(99), strength(MAX_LEVEL));
    }
}
