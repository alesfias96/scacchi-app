//! La scheda «Calcolo e tattica»: puzzle per profondità e per tema.

use std::collections::{BTreeMap, HashMap};

use training::theme_name;

use crate::stats_input::StatsInput;
use crate::stats_view::{ChartPoint, DepthStats, TacticsStats, ThemeStats};

/// Da quanti tentativi un tema entra nell'elenco dei temi più difficili.
const MIN_THEME_ATTEMPTS: u32 = 3;
/// Quanti temi difficili mostrare.
const MAX_THEMES: usize = 8;
/// Un giorno, in millesimi di secondo.
const DAY_MS: i64 = 24 * 60 * 60 * 1000;

/// La scheda «Calcolo e tattica» (i ripassi non contano: sono puzzle già visti).
#[must_use]
pub fn tactics_stats(input: &StatsInput) -> TacticsStats {
    let attempts: Vec<_> = input
        .puzzles
        .iter()
        .filter(|attempt| !attempt.is_review)
        .collect();

    let mut by_depth: BTreeMap<u32, DepthStats> = BTreeMap::new();
    let mut by_theme: HashMap<&str, (u32, u32)> = HashMap::new();
    let mut by_day: BTreeMap<i64, (u32, u32)> = BTreeMap::new();
    for attempt in &attempts {
        let solved = u32::from(attempt.solved);
        let depth = by_depth.entry(attempt.depth).or_insert(DepthStats {
            depth: attempt.depth,
            attempts: 0,
            solved: 0,
        });
        depth.attempts += 1;
        depth.solved += solved;
        for theme in &attempt.themes {
            let entry = by_theme.entry(theme.as_str()).or_default();
            entry.0 += 1;
            entry.1 += solved;
        }
        if attempt.solved {
            let day = by_day
                .entry(attempt.played_at_ms.div_euclid(DAY_MS))
                .or_default();
            day.0 += attempt.depth;
            day.1 += 1;
        }
    }

    let mut weakest_themes: Vec<ThemeStats> = by_theme
        .into_iter()
        .filter(|(_, (tried, _))| *tried >= MIN_THEME_ATTEMPTS)
        .filter_map(|(key, (tried, solved))| {
            Some(ThemeStats {
                key: key.to_owned(),
                name: theme_name(key)?.to_owned(),
                attempts: tried,
                solved,
            })
        })
        .collect();
    // Prima i temi con la percentuale di risolti più bassa.
    weakest_themes.sort_by(|a, b| {
        (u64::from(a.solved) * u64::from(b.attempts))
            .cmp(&(u64::from(b.solved) * u64::from(a.attempts)))
            .then_with(|| b.attempts.cmp(&a.attempts))
    });
    weakest_themes.truncate(MAX_THEMES);

    #[allow(clippy::cast_precision_loss)]
    let depth_history = by_day
        .into_iter()
        .map(|(day, (total, count))| ChartPoint {
            // Un giorno dal 1970 e la sua media stanno ben dentro la precisione di un f64.
            time_ms: (day * DAY_MS) as f64,
            value: f64::from(total) / f64::from(count),
        })
        .collect();

    TacticsStats {
        attempts: u32::try_from(attempts.len()).unwrap_or(u32::MAX),
        solved: u32::try_from(attempts.iter().filter(|attempt| attempt.solved).count())
            .unwrap_or(u32::MAX),
        by_depth: by_depth.into_values().collect(),
        depth_history,
        weakest_themes,
        due_reviews: input.due_reviews,
    }
}
