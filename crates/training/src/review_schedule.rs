//! Il ripasso dei puzzle sbagliati, con il metodo delle "scatole" (Leitner).
//!
//! Un puzzle sbagliato entra nella prima scatola e torna il giorno dopo. Ogni
//! volta che lo si risolve passa alla scatola successiva, e torna dopo più
//! tempo: 1, 3, 7, 14, 30 giorni. Risolto anche dall'ultima scatola, il
//! puzzle è imparato ed esce dal ripasso. Un nuovo errore lo riporta alla
//! prima scatola.

/// Dopo quanti giorni torna un puzzle, per ogni scatola.
const INTERVAL_DAYS: [i64; 5] = [1, 3, 7, 14, 30];
/// Un giorno, in millesimi di secondo.
const DAY_MS: i64 = 24 * 60 * 60 * 1000;

/// A che punto è il ripasso di un puzzle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReviewState {
    /// La scatola (da 0 a 4).
    pub box_index: u8,
    /// Quando torna, in millesimi di secondo dal 1970.
    pub due_at_ms: i64,
}

/// Il ripasso di un puzzle dopo un tentativo.
///
/// - Riceve: il ripasso attuale (`None` se il puzzle non è in ripasso), se il
///   puzzle è stato risolto e l'istante attuale.
/// - Restituisce: il nuovo ripasso, oppure `None` se il puzzle non è (più) da ripassare.
#[must_use]
pub fn next_review(current: Option<ReviewState>, solved: bool, now_ms: i64) -> Option<ReviewState> {
    if !solved {
        return Some(ReviewState {
            box_index: 0,
            due_at_ms: now_ms + INTERVAL_DAYS[0] * DAY_MS,
        });
    }
    let current = current?;
    let next = usize::from(current.box_index) + 1;
    let days = INTERVAL_DAYS.get(next)?;
    Some(ReviewState {
        box_index: u8::try_from(next).unwrap_or(u8::MAX),
        due_at_ms: now_ms + days * DAY_MS,
    })
}

#[cfg(test)]
mod tests {
    use super::{DAY_MS, ReviewState, next_review};

    #[test]
    fn a_mistake_puts_the_puzzle_in_the_first_box() {
        let review = next_review(None, false, 0).expect("in ripasso");

        assert_eq!(
            review,
            ReviewState {
                box_index: 0,
                due_at_ms: DAY_MS
            }
        );
    }

    #[test]
    fn solving_moves_it_forward_and_further_in_time() {
        let first = ReviewState {
            box_index: 0,
            due_at_ms: 0,
        };

        let second = next_review(Some(first), true, 0).expect("ancora in ripasso");

        assert_eq!(
            second,
            ReviewState {
                box_index: 1,
                due_at_ms: 3 * DAY_MS
            }
        );
    }

    #[test]
    fn solved_from_the_last_box_it_is_learned() {
        let last = ReviewState {
            box_index: 4,
            due_at_ms: 0,
        };

        assert_eq!(next_review(Some(last), true, 0), None);
    }

    #[test]
    fn a_puzzle_solved_at_the_first_try_never_enters_the_review() {
        assert_eq!(next_review(None, true, 0), None);
    }

    #[test]
    fn a_new_mistake_starts_again_from_the_first_box() {
        let late = ReviewState {
            box_index: 3,
            due_at_ms: 0,
        };

        assert_eq!(
            next_review(Some(late), false, 0).map(|review| review.box_index),
            Some(0)
        );
    }
}
