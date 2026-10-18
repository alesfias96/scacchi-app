//! La valutazione di una posizione dal punto di vista del Bianco.

use chess_core::Side;

use crate::score::Score;

/// Quanto è buona la posizione per il **Bianco**: positivo se sta meglio il
/// Bianco, negativo se sta meglio il Nero.
///
/// Il motore ragiona dal punto di vista di chi muove; la barra di valutazione
/// invece ha sempre il Bianco da una parte e il Nero dall'altra, quindi il
/// punteggio viene "girato" quando muove il Nero.
///
/// Nel JSON per l'interfaccia diventa `{ "kind": "centipawns", "value": 35 }`
/// oppure `{ "kind": "mate", "value": -3 }`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(tag = "kind", content = "value", rename_all = "camelCase"),
    ts(export)
)]
pub enum Evaluation {
    /// Vantaggio in centesimi di pedone (+100 ≈ un pedone in più per il Bianco).
    Centipawns(i32),
    /// Scacco matto in tante mosse: positivo se lo dà il Bianco, negativo se lo dà il Nero.
    Mate(i32),
}

impl Evaluation {
    /// Converte il punteggio del motore nel punto di vista del Bianco.
    ///
    /// - Riceve: il punteggio (dal punto di vista di chi muove) e chi muove.
    /// - Restituisce: la valutazione per il Bianco.
    #[must_use]
    pub fn from_score(score: Score, side_to_move: Side) -> Self {
        let sign = match side_to_move {
            Side::White => 1,
            Side::Black => -1,
        };
        match score {
            Score::Centipawns(value) => Self::Centipawns(value * sign),
            Score::Mate(moves) => Self::Mate(moves * sign),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Evaluation;
    use crate::score::Score;
    use chess_core::Side;

    #[test]
    fn keeps_the_score_when_white_is_to_move() {
        assert_eq!(
            Evaluation::from_score(Score::Centipawns(40), Side::White),
            Evaluation::Centipawns(40)
        );
    }

    #[test]
    fn turns_the_score_around_when_black_is_to_move() {
        assert_eq!(
            Evaluation::from_score(Score::Centipawns(40), Side::Black),
            Evaluation::Centipawns(-40)
        );
        assert_eq!(
            Evaluation::from_score(Score::Mate(2), Side::Black),
            Evaluation::Mate(-2)
        );
    }
}
