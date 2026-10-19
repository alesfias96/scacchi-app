//! La scheda «Partite»: risultati, errori per fase ed errori con poco tempo.

use std::collections::BTreeMap;

use analysis::GamePhase;
use app_settings::OpponentKind;
use chess_core::Side;
use engine::levels;
use review::MoveQuality;

use crate::game_score::{add, user_score};
use crate::judged_move::{JudgedMove, average_accuracy};
use crate::stats_input::StatsInput;
use crate::stats_view::{ColorScore, GameStats, LevelScore, PhaseStats, Score, TimePressure};

/// Sotto questa frazione del tempo iniziale si è "in zeitnot" (un quinto).
const LOW_TIME_DIVISOR: u32 = 5;

/// Quante mosse di un certo giudizio.
fn count(moves: &[&JudgedMove<'_>], quality: MoveQuality) -> u32 {
    u32::try_from(
        moves
            .iter()
            .filter(|judged| judged.quality == quality)
            .count(),
    )
    .unwrap_or(u32::MAX)
}

/// Come si gioca in ogni fase della partita.
fn phases(moves: &[JudgedMove<'_>]) -> Vec<PhaseStats> {
    [
        GamePhase::Opening,
        GamePhase::Middlegame,
        GamePhase::Endgame,
    ]
    .into_iter()
    .map(|phase| {
        let own: Vec<&JudgedMove<'_>> = moves
            .iter()
            .filter(|judged| judged.phase == Some(phase))
            .collect();
        PhaseStats {
            phase,
            moves: u32::try_from(own.len()).unwrap_or(u32::MAX),
            accuracy: average_accuracy(own.iter().map(|judged| &judged.accuracy)),
            inaccuracies: count(&own, MoveQuality::Inaccuracy),
            mistakes: count(&own, MoveQuality::Mistake),
            blunders: count(&own, MoveQuality::Blunder),
        }
    })
    .collect()
}

/// Gli errori con poco tempo e gli altri, nelle partite con l'orologio.
fn time_pressure(moves: &[JudgedMove<'_>]) -> Option<TimePressure> {
    let mut result = TimePressure {
        low_time_moves: 0,
        low_time_errors: 0,
        other_moves: 0,
        other_errors: 0,
    };
    for judged in moves {
        let (Some(initial), Some(left)) = (judged.game.time_initial_ms, judged.clock_after_ms)
        else {
            continue;
        };
        let is_error = u32::from(judged.quality.is_error());
        if left < initial / LOW_TIME_DIVISOR {
            result.low_time_moves += 1;
            result.low_time_errors += is_error;
        } else {
            result.other_moves += 1;
            result.other_errors += is_error;
        }
    }
    (result.low_time_moves + result.other_moves > 0).then_some(result)
}

/// La scheda «Partite».
#[must_use]
pub fn game_stats(input: &StatsInput, moves: &[JudgedMove<'_>]) -> GameStats {
    let mut against_computer = Score::default();
    let mut by_color: BTreeMap<u8, (Side, Score)> = BTreeMap::new();
    let mut by_level: BTreeMap<u8, Score> = BTreeMap::new();
    for game in &input.games {
        let (Some((computer, level)), Some(score)) = (game.computer, user_score(game)) else {
            continue;
        };
        against_computer = add(against_computer, score);
        let user = computer.opponent();
        let key = u8::from(user == Side::Black);
        let entry = by_color.entry(key).or_insert((user, Score::default()));
        entry.1 = add(entry.1, score);
        let level_entry = by_level.entry(level).or_default();
        *level_entry = add(*level_entry, score);
    }
    let names = levels();
    GameStats {
        against_computer,
        two_player_games: u32::try_from(
            input
                .games
                .iter()
                .filter(|game| game.opponent == OpponentKind::TwoPlayers)
                .count(),
        )
        .unwrap_or(u32::MAX),
        by_color: by_color
            .into_values()
            .map(|(side, score)| ColorScore { side, score })
            .collect(),
        by_level: by_level
            .into_iter()
            .filter_map(|(level, score)| {
                let info = names.iter().find(|info| info.level == level)?;
                Some(LevelScore {
                    level,
                    name: info.name.clone(),
                    elo: info.elo,
                    score,
                })
            })
            .collect(),
        phases: phases(moves),
        time_pressure: time_pressure(moves),
    }
}
