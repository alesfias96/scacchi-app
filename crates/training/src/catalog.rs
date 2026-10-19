//! L'elenco dei puzzle inclusi nell'app.

use std::collections::HashMap;
use std::sync::LazyLock;

use crate::puzzle::Puzzle;

/// Il file dei puzzle (circa 100.000), incluso nell'app durante la compilazione.
/// Lo prepara `npm run puzzles` (vedi `scripts/fetch-puzzles.js`).
const SOURCE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../data/puzzles/lichess-puzzles.csv"
));

/// Tutti i puzzle, letti la prima volta che servono (una frazione di secondo)
/// e poi tenuti in memoria: scegliere un puzzle è istantaneo.
static PUZZLES: LazyLock<Vec<Puzzle>> =
    LazyLock::new(|| SOURCE.lines().skip(1).filter_map(Puzzle::parse).collect());

/// Per ogni codice, la posizione del puzzle nell'elenco.
static BY_ID: LazyLock<HashMap<&'static str, usize>> = LazyLock::new(|| {
    PUZZLES
        .iter()
        .enumerate()
        .map(|(index, puzzle)| (puzzle.id.as_str(), index))
        .collect()
});

/// Tutti i puzzle.
#[must_use]
pub fn all_puzzles() -> &'static [Puzzle] {
    &PUZZLES
}

/// Il puzzle con il codice indicato, se c'è.
#[must_use]
pub fn puzzle_by_id(id: &str) -> Option<&'static Puzzle> {
    BY_ID.get(id).map(|index| &PUZZLES[*index])
}

#[cfg(test)]
mod tests {
    use super::{all_puzzles, puzzle_by_id};

    #[test]
    fn contains_about_one_hundred_thousand_puzzles() {
        assert!(all_puzzles().len() > 90_000);
    }

    #[test]
    fn finds_a_puzzle_by_its_code() {
        let first = &all_puzzles()[0];

        assert_eq!(puzzle_by_id(&first.id), Some(first));
        assert_eq!(puzzle_by_id("non-esiste"), None);
    }

    #[test]
    fn every_depth_from_one_to_six_is_present() {
        for depth in 1..=6 {
            assert!(all_puzzles().iter().any(|puzzle| puzzle.depth() == depth));
        }
    }
}
