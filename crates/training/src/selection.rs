//! La scelta del prossimo puzzle.

use std::collections::HashSet;
use std::hash::BuildHasher;

use app_settings::{Difficulty, TrainingMode, TrainingSetup};

use crate::catalog::{all_puzzles, puzzle_by_id};
use crate::puzzle::Puzzle;
use crate::random::pick;

/// Di quanto il puzzle proposto è più facile o più difficile del proprio rating.
fn rating_offset(difficulty: Difficulty) -> i32 {
    match difficulty {
        Difficulty::Easier => -150,
        Difficulty::Normal => 100,
        Difficulty::Harder => 350,
    }
}

/// Le finestre di difficoltà provate in ordine: prima vicine al livello
/// cercato, poi sempre più larghe, finché si trova almeno un puzzle.
const RATING_WINDOWS: [i32; 5] = [100, 200, 400, 800, 4000];

/// `true` se il puzzle va bene per le scelte fatte (profondità e tema).
fn matches(puzzle: &Puzzle, setup: &TrainingSetup) -> bool {
    let depth_ok = match setup.mode {
        TrainingMode::Depth | TrainingMode::Visualization => {
            puzzle.depth() == usize::from(setup.depth)
        }
        // In "cosa minaccia?" conta solo la prima mossa: va bene ogni profondità.
        TrainingMode::Threat | TrainingMode::Review => true,
    };
    let theme_ok = setup
        .theme
        .as_deref()
        .is_none_or(|theme| puzzle.has_theme(theme));
    depth_ok && theme_ok
}

/// Sceglie un puzzle nuovo, adatto alle scelte fatte e al proprio livello.
///
/// - Riceve: le scelte dell'allenamento, il proprio rating, i puzzle giocati
///   di recente (da non riproporre) e un numero di partenza per la scelta a caso.
/// - Restituisce: il puzzle, oppure `None` se nessun puzzle va bene (ad
///   esempio un tema senza puzzle di quella profondità).
#[must_use]
pub fn choose_puzzle<S: BuildHasher>(
    setup: &TrainingSetup,
    rating: i32,
    recent: &HashSet<String, S>,
    seed: u64,
) -> Option<&'static Puzzle> {
    let target = rating + rating_offset(setup.difficulty);
    let suitable: Vec<&'static Puzzle> = all_puzzles()
        .iter()
        .filter(|puzzle| matches(puzzle, setup) && !recent.contains(&puzzle.id))
        .collect();
    RATING_WINDOWS.iter().find_map(|window| {
        let near: Vec<&'static Puzzle> = suitable
            .iter()
            .copied()
            .filter(|puzzle| {
                (i32::try_from(puzzle.rating).unwrap_or(i32::MAX) - target).abs() <= *window
            })
            .collect();
        pick(&near, seed).copied()
    })
}

/// Sceglie un puzzle da ripassare tra quelli in scadenza (con il tema scelto, se c'è).
///
/// Restituisce `None` se non c'è nulla da ripassare.
#[must_use]
pub fn choose_review(setup: &TrainingSetup, due: &[String], seed: u64) -> Option<&'static Puzzle> {
    let candidates: Vec<&'static Puzzle> = due
        .iter()
        .filter_map(|id| puzzle_by_id(id))
        .filter(|puzzle| matches(puzzle, setup))
        .collect();
    pick(&candidates, seed).copied()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use app_settings::{Difficulty, TrainingMode, TrainingSetup};

    use super::{choose_puzzle, choose_review};
    use crate::catalog::all_puzzles;

    fn setup(mode: TrainingMode, depth: u8, theme: Option<&str>) -> TrainingSetup {
        TrainingSetup {
            mode,
            depth,
            theme: theme.map(str::to_owned),
            difficulty: Difficulty::Normal,
            visualization_seconds: 5,
        }
    }

    #[test]
    fn respects_depth_theme_and_level() {
        let chosen = choose_puzzle(
            &setup(TrainingMode::Depth, 3, Some("fork")),
            1500,
            &HashSet::new(),
            1,
        )
        .expect("puzzle trovato");

        assert_eq!(chosen.depth(), 3);
        assert!(chosen.has_theme("fork"));
        assert!(chosen.rating.abs_diff(1600) <= 100);
    }

    #[test]
    fn does_not_repeat_recent_puzzles() {
        let first = choose_puzzle(
            &setup(TrainingMode::Depth, 1, None),
            1500,
            &HashSet::new(),
            9,
        )
        .expect("puzzle trovato");
        let recent = HashSet::from([first.id.clone()]);

        let second = choose_puzzle(&setup(TrainingMode::Depth, 1, None), 1500, &recent, 9)
            .expect("puzzle trovato");

        assert_ne!(first.id, second.id);
    }

    #[test]
    fn widens_the_search_when_needed() {
        // Nessun puzzle sta a 3400: si allarga la finestra finché se ne trova uno.
        let chosen = choose_puzzle(
            &setup(TrainingMode::Threat, 1, None),
            3400,
            &HashSet::new(),
            3,
        );

        assert!(chosen.is_some());
    }

    #[test]
    fn reviews_only_the_due_puzzles() {
        let due = vec![all_puzzles()[10].id.clone()];

        let chosen =
            choose_review(&setup(TrainingMode::Review, 1, None), &due, 5).expect("in ripasso");

        assert_eq!(chosen.id, due[0]);
        assert_eq!(
            choose_review(&setup(TrainingMode::Review, 1, None), &[], 5),
            None
        );
    }
}
