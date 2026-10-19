//! Le mosse della persona nelle partite analizzate, con il loro giudizio.
//!
//! Il giudizio è lo stesso dell'analisi delle partite (crate `review`): la
//! probabilità di vittoria persa decide se la mossa è buona, un'imprecisione,
//! un errore o un errore grave.

use std::collections::HashMap;

use analysis::GamePhase;
use chess_core::Side;
use review::MoveQuality;
use review::win_chance::{chance_for, move_accuracy, white_win_chance};

use crate::stats_input::{GameFacts, StatsInput};

/// Una mossa della persona, giudicata.
#[derive(Debug, Clone, PartialEq)]
pub struct JudgedMove<'a> {
    /// La partita della mossa.
    pub game: &'a GameFacts,
    /// Il colore con cui è stata giocata.
    pub side: Side,
    /// La fase di gioco.
    pub phase: Option<GamePhase>,
    /// Il giudizio.
    pub quality: MoveQuality,
    /// La precisione della mossa, da 0 a 100.
    pub accuracy: f64,
    /// Il tempo che restava dopo la mossa, se c'era l'orologio.
    pub clock_after_ms: Option<u32>,
}

/// Le mosse giocate dalla persona nelle partite analizzate, con il giudizio.
///
/// Si saltano le mosse del computer e quelle senza le due valutazioni (ad
/// esempio la mossa che dà matto, dopo la quale il motore non valuta più).
#[must_use]
pub fn judge_user_moves(input: &StatsInput) -> Vec<JudgedMove<'_>> {
    let games: HashMap<&str, &GameFacts> = input
        .games
        .iter()
        .map(|game| (game.id.as_str(), game))
        .collect();
    input
        .moves
        .iter()
        .filter_map(|facts| {
            let game = *games.get(facts.game_id.as_str())?;
            if !game.user_sides().contains(&facts.side) {
                return None;
            }
            let before = chance_for(white_win_chance(facts.evaluation_before?), facts.side);
            let after = chance_for(white_win_chance(facts.evaluation_after?), facts.side);
            let lost = (before - after).max(0.0);
            let is_best = facts.best_move.as_deref() == Some(facts.played.as_str());
            Some(JudgedMove {
                game,
                side: facts.side,
                phase: facts.phase,
                quality: MoveQuality::judge(lost, is_best),
                accuracy: move_accuracy(lost),
                clock_after_ms: facts.clock_after_ms,
            })
        })
        .collect()
}

/// La media di alcune precisioni, arrotondata (da 0 a 100); `None` se non ce ne sono.
#[must_use]
pub fn average_accuracy<'a>(accuracies: impl IntoIterator<Item = &'a f64>) -> Option<u32> {
    let (total, count) = accuracies
        .into_iter()
        .fold((0.0, 0u32), |(total, count), value| {
            (total + value, count + 1)
        });
    // La media sta tra 0 e 100: la conversione non perde nulla.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    (count > 0).then(|| (total / f64::from(count)).round() as u32)
}
