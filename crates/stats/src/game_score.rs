//! Il risultato di una partita contro il computer, dal punto di vista della persona.

use crate::stats_input::GameFacts;
use crate::stats_view::Score;

/// Il risultato di una partita finita contro il computer: una vittoria, una
/// patta o una sconfitta della persona. `None` per le partite non finite o a
/// due giocatori (lì vince sempre "la persona").
#[must_use]
pub fn user_score(game: &GameFacts) -> Option<Score> {
    let computer = game.computer.as_ref()?.side;
    if !game.finished {
        return None;
    }
    Some(match game.winner {
        None => Score {
            draws: 1,
            ..Score::default()
        },
        Some(winner) if winner == computer => Score {
            losses: 1,
            ..Score::default()
        },
        Some(_) => Score {
            wins: 1,
            ..Score::default()
        },
    })
}

/// Somma due risultati.
#[must_use]
pub fn add(a: Score, b: Score) -> Score {
    Score {
        wins: a.wins + b.wins,
        draws: a.draws + b.draws,
        losses: a.losses + b.losses,
    }
}
