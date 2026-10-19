//! La scheda «Livello»: Elo stimato, rating dei puzzle e precisione nel tempo.

use std::collections::BTreeMap;

use crate::game_score::user_score;
use crate::judged_move::{JudgedMove, average_accuracy};
use crate::stats_input::StatsInput;
use crate::stats_view::{ChartPoint, LevelStats};

/// Da quante partite contro il computer si mostra l'Elo stimato.
const MIN_ELO_GAMES: u32 = 5;
/// Quanto può valere al massimo lo scarto dalla media degli avversari (±400, come nei tornei).
const PERFORMANCE_SPREAD: f64 = 400.0;

/// Un istante come numero per i grafici.
#[allow(clippy::cast_precision_loss)]
fn time(ms: i64) -> f64 {
    // I millesimi di secondo dal 1970 stanno ben dentro la precisione di un f64.
    ms as f64
}

/// L'Elo di rendimento contro i livelli del computer (Stockfish e Maia insieme):
/// la media dell'Elo indicativo degli avversari, più 400 × (vittorie − sconfitte) / partite.
fn estimated_elo(input: &StatsInput) -> (Option<i32>, u32) {
    let mut total_elo = 0.0;
    let mut balance = 0.0;
    let mut games = 0u32;
    for game in &input.games {
        let (Some(computer), Some(score)) = (&game.computer, user_score(game)) else {
            continue;
        };
        total_elo += f64::from(computer.level.elo);
        balance += f64::from(score.wins) - f64::from(score.losses);
        games += 1;
    }
    if games < MIN_ELO_GAMES {
        return (None, games);
    }
    let count = f64::from(games);
    let performance = total_elo / count + PERFORMANCE_SPREAD * balance / count;
    // Un Elo sta tra qualche centinaio e qualche migliaio: la conversione non perde nulla.
    #[allow(clippy::cast_possible_truncation)]
    (Some(performance.round() as i32), games)
}

/// La scheda «Livello».
#[must_use]
pub fn level_stats(input: &StatsInput, moves: &[JudgedMove<'_>]) -> LevelStats {
    let (estimated_elo, elo_games) = estimated_elo(input);

    // La precisione di ogni partita analizzata, nell'ordine in cui si è giocato.
    let mut by_game: BTreeMap<(i64, &str), Vec<f64>> = BTreeMap::new();
    for judged in moves {
        by_game
            .entry((judged.game.started_at_ms, judged.game.id.as_str()))
            .or_default()
            .push(judged.accuracy);
    }
    let accuracy_history = by_game
        .iter()
        .filter_map(|((started, _), values)| {
            average_accuracy(values).map(|accuracy| ChartPoint {
                time_ms: time(*started),
                value: f64::from(accuracy),
            })
        })
        .collect();

    LevelStats {
        estimated_elo,
        elo_games,
        puzzle_rating: input.puzzle_rating,
        puzzle_history: input
            .puzzles
            .iter()
            .filter(|attempt| !attempt.is_review)
            .map(|attempt| ChartPoint {
                time_ms: time(attempt.played_at_ms),
                value: f64::from(attempt.rating_after),
            })
            .collect(),
        accuracy: average_accuracy(moves.iter().map(|judged| &judged.accuracy)),
        accuracy_history,
    }
}
