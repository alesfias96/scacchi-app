//! Le strade dell'avversario: cosa potrebbe fare se toccasse a lui.
//!
//! Non si dice «l'avversario vuole fare questo, punto»: si mostrano le strade
//! **possibili**. Si "passa la mano" (la stessa posizione, ma con l'avversario
//! che muove), Stockfish calcola le sue varianti migliori e ognuna diventa una
//! strada, spiegata come un'idea (vedi `idea`) ma al condizionale: «una strada
//! possibile: … — si vedrebbe tra 3 mosse». È il passo «cosa minaccia
//! l'avversario?» del metodo di pensiero, fatto guardando lontano.

use analysis::wording::side_name;
use chess_core::Side;
use engine::Evaluation;
use shakmaty::fen::Fen;
use shakmaty::{Chess, EnPassantMode, Position};

use crate::evaluation_words::score_for;
use crate::explanation::chess_position;
use crate::idea::{Voice, describe_idea_as, reached};
use crate::idea_view::{IdeaView, OpponentPaths};
use crate::tutor_error::TutorError;

/// Da quale vantaggio (centesimi di pedone, per l'avversario) una strada è pericolosa.
const DANGEROUS: i32 = 150;
/// Da quale punteggio la valutazione è un matto (vedi `score_for`).
const MATE_THRESHOLD: i32 = 50_000;

/// La posizione raggiunta dalla partita con la mano passata all'avversario
/// (in FEN), oppure `None` se chi muove è sotto scacco (non si può passare la mano).
///
/// # Errors
///
/// [`TutorError`] se la posizione o una mossa della partita non sono valide.
pub fn flipped_fen(start_fen: &str, moves: &[String]) -> Result<Option<String>, TutorError> {
    let position = reached(start_fen, moves)?;
    if position.is_check() {
        return Ok(None);
    }
    Ok(position
        .swap_turn()
        .ok()
        .map(|flipped| Fen::from_position(&flipped, EnPassantMode::Legal).to_string()))
}

/// Le strade quando chi muove è sotto scacco: prima va parato lo scacco.
///
/// # Errors
///
/// [`TutorError`] se la posizione o una mossa della partita non sono valide.
pub fn paths_when_in_check(start_fen: &str, moves: &[String]) -> Result<OpponentPaths, TutorError> {
    let position = reached(start_fen, moves)?;
    Ok(OpponentPaths {
        side: Side::from(position.turn().other()),
        intro: "Sei sotto scacco: prima di tutto devi parare lo scacco, le strade dell'avversario vengono dopo.".to_owned(),
        paths: Vec::new(),
    })
}

/// La frase d'introduzione, secondo quanto è pericolosa la strada migliore.
fn intro(position: &Chess, best: Option<Evaluation>) -> String {
    let side = position.turn();
    let who = side_name(side);
    match best.map(|evaluation| score_for(evaluation, side)) {
        Some(score) if score >= MATE_THRESHOLD => format!(
            "Attenzione: se toccasse a {who}, potrebbe dare matto. Prima di muovere, cerca come impedirlo. Le strade che avrebbe:"
        ),
        Some(score) if score >= DANGEROUS => format!(
            "Attenzione: se toccasse a {who}, avrebbe una strada pericolosa. Prima di muovere, chiediti come fermarla. Le strade che potrebbe prendere:"
        ),
        Some(_) => format!(
            "Se toccasse a {who}, queste sono le strade che potrebbe prendere: nessuna è una minaccia seria, ma tienile d'occhio."
        ),
        None => format!("Non ci sono strade da mostrare per {who}."),
    }
}

/// Le strade della posizione `position` (già con la mano all'avversario), dalle varianti di Stockfish.
#[must_use]
pub fn describe_opponent_paths(
    position: &Chess,
    lines: &[(Vec<String>, Evaluation)],
) -> OpponentPaths {
    let side = position.turn();
    let paths: Vec<IdeaView> = lines
        .iter()
        .map(|(line, evaluation)| {
            describe_idea_as(position, line, side, Some(*evaluation), Voice::Possibility)
        })
        .collect();
    OpponentPaths {
        side: Side::from(side),
        intro: intro(position, lines.first().map(|(_, evaluation)| *evaluation)),
        paths,
    }
}

/// Le strade dell'avversario, dalla posizione con la mano passata (vedi [`flipped_fen`]).
///
/// # Errors
///
/// [`TutorError::InvalidPosition`] se la FEN non è valida.
pub fn opponent_paths(
    flipped: &str,
    lines: &[(Vec<String>, Evaluation)],
) -> Result<OpponentPaths, TutorError> {
    Ok(describe_opponent_paths(&chess_position(flipped)?, lines))
}

#[cfg(test)]
mod tests {
    use engine::Evaluation;
    use shakmaty::fen::Fen;
    use shakmaty::{CastlingMode, Chess};

    use super::{describe_opponent_paths, flipped_fen};

    #[test]
    fn passes_the_move_unless_in_check() {
        let start = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let flipped = flipped_fen(start, &[])
            .expect("posizione")
            .expect("si può passare");
        assert!(flipped.contains(" b "), "{flipped}");

        let check = ["e2e4", "f7f6", "d1h5"].map(str::to_owned);
        assert_eq!(flipped_fen(start, &check).expect("posizione"), None);
    }

    #[test]
    fn a_dangerous_path_is_announced_and_written_as_a_possibility() {
        // Mano passata al Nero: la torre in a2 dà matto in a1 sull'ultima traversa.
        let position: Chess = "6k1/5ppp/8/8/8/8/r4PPP/6K1 b - - 0 1"
            .parse::<Fen>()
            .expect("FEN")
            .into_position(CastlingMode::Standard)
            .expect("posizione");
        let paths = describe_opponent_paths(
            &position,
            &[(vec!["a2a1".to_owned()], Evaluation::Mate(-1))],
        );
        assert!(paths.intro.starts_with("Attenzione"), "{}", paths.intro);
        assert_eq!(paths.paths.len(), 1);
        assert!(
            paths.paths[0].headline.starts_with("Una strada possibile"),
            "{}",
            paths.paths[0].headline
        );
    }
}
