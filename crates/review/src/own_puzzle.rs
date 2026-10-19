//! I puzzle nati dagli errori gravi delle proprie partite.
//!
//! Il puzzle ha la stessa forma di quelli di Lichess: comincia dalla posizione
//! prima della mossa dell'avversario, la prima mossa è quella dell'avversario e
//! poi bisogna trovare la mossa migliore che nella partita non si è giocata.

use chess_core::Side;

use crate::move_quality::MoveQuality;
use crate::replay::replay;
use crate::review_error::ReviewError;
use crate::review_input::ReviewInput;
use crate::review_view::GameReview;

/// Un puzzle da un errore grave.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnPuzzle {
    /// La mezza mossa dell'errore.
    pub ply: u32,
    /// La posizione prima della mossa dell'avversario (FEN).
    pub fen: String,
    /// La mossa dell'avversario e la mossa migliore (UCI).
    pub moves: Vec<String>,
}

/// I puzzle da creare con gli errori gravi dei giocatori indicati.
///
/// - Riceve: la partita, la sua revisione e i colori di chi ha giocato
///   davvero (contro il computer solo il proprio: i suoi errori non insegnano).
///
/// # Errors
///
/// [`ReviewError::InvalidGame`] se la partita non si può rigiocare.
pub fn own_puzzles(
    input: &ReviewInput,
    review: &GameReview,
    players: &[Side],
) -> Result<Vec<OwnPuzzle>, ReviewError> {
    let replayed = replay(&input.start_fen, &input.moves)?;
    Ok(review
        .moves
        .iter()
        .enumerate()
        .filter(|(index, reviewed)| {
            *index >= 1
                && reviewed.quality == Some(MoveQuality::Blunder)
                && players.contains(&reviewed.side)
        })
        .filter_map(|(index, reviewed)| {
            let best = reviewed.best.as_ref()?;
            Some(OwnPuzzle {
                ply: reviewed.ply,
                fen: replayed.fens[index - 1].clone(),
                moves: vec![input.moves[index - 1].clone(), best.uci.clone()],
            })
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use chess_core::Side;

    use super::own_puzzles;
    use crate::game_reviewer::review_game;
    use crate::test_game::scholars_mate;

    #[test]
    fn a_blunder_becomes_a_puzzle_for_its_player() {
        let input = scholars_mate();
        let review = review_game(&input).expect("partita valida");

        let puzzles = own_puzzles(&input, &review, &[Side::Black]).expect("partita valida");

        assert_eq!(puzzles.len(), 1);
        assert_eq!(puzzles[0].ply, 6);
        assert_eq!(puzzles[0].moves, ["f1c4", "d8e7"]);
        assert_eq!(
            own_puzzles(&input, &review, &[Side::White]).expect("partita valida"),
            []
        );
    }
}
