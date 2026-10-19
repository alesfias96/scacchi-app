//! Una partita di prova per i test di questa crate (non è usata dall'app).

use engine::Evaluation;

use crate::review_input::{PositionEval, ReviewInput};

const START: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

fn eval(cp: i32, best: &str) -> PositionEval {
    PositionEval {
        evaluation: Some(Evaluation::Centipawns(cp)),
        best_move: Some(best.to_owned()),
    }
}

/// Il matto del barbiere sbagliato dal Nero: 1.e4 e5 2.Qh5 Nc6 3.Bc4 Nf6?? 4.Qxf7#.
pub(crate) fn scholars_mate() -> ReviewInput {
    ReviewInput {
        start_fen: START.to_owned(),
        moves: ["e2e4", "e7e5", "d1h5", "b8c6", "f1c4", "g8f6", "h5f7"]
            .iter()
            .map(|uci| (*uci).to_owned())
            .collect(),
        evaluations: vec![
            eval(30, "e2e4"),
            eval(30, "e7e5"),
            eval(30, "g1f3"),
            eval(0, "b8c6"),
            eval(20, "f1c4"),
            eval(30, "d8e7"),
            PositionEval {
                evaluation: Some(Evaluation::Mate(1)),
                best_move: Some("h5f7".to_owned()),
            },
            PositionEval::default(),
        ],
    }
}
