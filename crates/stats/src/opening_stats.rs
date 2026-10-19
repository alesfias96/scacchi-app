//! La scheda «Aperture»: le aperture più giocate, per colore.

use std::collections::HashMap;

use chess_core::Side;

use crate::game_score::{add, user_score};
use crate::judged_move::{JudgedMove, average_accuracy};
use crate::stats_input::StatsInput;
use crate::stats_view::{OpeningStats, Score};

/// Quante aperture mostrare al massimo per colore.
const MAX_PER_SIDE: usize = 8;

/// La famiglia di un'apertura di Lichess: il nome prima dei due punti
/// ("Sicilian Defense: Najdorf Variation" → "Sicilian Defense").
fn family(name: &str) -> &str {
    name.split(':').next().unwrap_or(name).trim()
}

/// Ciò che si accumula per un'apertura con un colore.
#[derive(Default)]
struct Totals {
    games: u32,
    score: Score,
    accuracies: Vec<f64>,
}

/// La scheda «Aperture».
#[must_use]
pub fn opening_stats(input: &StatsInput, moves: &[JudgedMove<'_>]) -> Vec<OpeningStats> {
    let mut totals: HashMap<(String, u8), Totals> = HashMap::new();
    let side_key = |side: Side| u8::from(side == Side::Black);
    for game in &input.games {
        let Some(name) = game.opening_name.as_deref() else {
            continue;
        };
        for side in game.user_sides() {
            let entry = totals
                .entry((family(name).to_owned(), side_key(side)))
                .or_default();
            entry.games += 1;
            if let Some(score) = user_score(game) {
                entry.score = add(entry.score, score);
            }
        }
    }
    for judged in moves {
        if let Some(name) = judged.game.opening_name.as_deref()
            && let Some(entry) = totals.get_mut(&(family(name).to_owned(), side_key(judged.side)))
        {
            entry.accuracies.push(judged.accuracy);
        }
    }

    let mut result: Vec<OpeningStats> = totals
        .into_iter()
        .map(|((name, side), totals)| OpeningStats {
            name,
            side: if side == 0 { Side::White } else { Side::Black },
            score: totals.score,
            games: totals.games,
            accuracy: average_accuracy(&totals.accuracies),
        })
        .collect();
    // Le più giocate prima; a parità, in ordine alfabetico.
    result.sort_by(|a, b| b.games.cmp(&a.games).then_with(|| a.name.cmp(&b.name)));
    let mut kept = Vec::new();
    for side in [Side::White, Side::Black] {
        kept.extend(
            result
                .iter()
                .filter(|opening| opening.side == side)
                .take(MAX_PER_SIDE)
                .cloned(),
        );
    }
    kept
}
