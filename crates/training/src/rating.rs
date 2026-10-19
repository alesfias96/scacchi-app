//! Il rating dei puzzle: un numero che misura il proprio livello nei puzzle.

/// Il rating di chi non ha ancora risolto nessun puzzle.
pub const START_RATING: i32 = 1500;
/// Quanto può cambiare al massimo il rating con un solo puzzle.
const K_FACTOR: f64 = 32.0;
/// I limiti del rating.
const MIN_RATING: i32 = 100;
const MAX_RATING: i32 = 3500;

/// Il nuovo rating dopo un puzzle (formula Elo, la stessa dei tornei).
///
/// Si vince "contro il puzzle" risolvendolo. Risolvere un puzzle difficile fa
/// salire molto, sbagliarne uno facile fa scendere molto; il contrario
/// conta poco.
///
/// - Riceve: il rating attuale, il rating del puzzle, se è stato risolto.
/// - Restituisce: il rating aggiornato.
#[must_use]
pub fn updated_rating(player: i32, puzzle: u32, solved: bool) -> i32 {
    let difference = f64::from(i32::try_from(puzzle).unwrap_or(MAX_RATING) - player);
    let expected = 1.0 / (1.0 + 10f64.powf(difference / 400.0));
    let score = if solved { 1.0 } else { 0.0 };
    let change = (K_FACTOR * (score - expected)).round();
    // Il cambiamento è al più ±32: la conversione non perde nulla.
    #[allow(clippy::cast_possible_truncation)]
    let change = change as i32;
    (player + change).clamp(MIN_RATING, MAX_RATING)
}

#[cfg(test)]
mod tests {
    use super::{START_RATING, updated_rating};

    #[test]
    fn an_even_puzzle_moves_the_rating_by_half_the_factor() {
        assert_eq!(updated_rating(1500, 1500, true), 1516);
        assert_eq!(updated_rating(1500, 1500, false), 1484);
    }

    #[test]
    fn a_hard_puzzle_solved_counts_more_than_an_easy_one() {
        let hard = updated_rating(START_RATING, 2000, true) - START_RATING;
        let easy = updated_rating(START_RATING, 1000, true) - START_RATING;

        assert!(hard > easy);
        assert!(easy >= 0);
    }
}
