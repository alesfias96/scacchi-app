//! La valutazione di Stockfish detta a parole.

use engine::Evaluation;
use shakmaty::Color;

use analysis::wording::side_name_capitalized;

/// Un matto vale più di qualsiasi vantaggio: prima arriva, più vale.
const MATE_SCORE: i32 = 100_000;

/// La valutazione in centesimi di pedone dal punto di vista di `color`
/// (i matti diventano numeri enormi, più grandi quanto più sono vicini).
#[must_use]
pub fn score_for(evaluation: Evaluation, color: Color) -> i32 {
    let for_white = match evaluation {
        Evaluation::Centipawns(value) => value,
        Evaluation::Mate(moves) if moves > 0 => MATE_SCORE - moves * 100,
        Evaluation::Mate(moves) if moves < 0 => -MATE_SCORE - moves * 100,
        // "Matto in 0": chi deve muovere ha già ricevuto matto. Il segno non si
        // conosce da qui; chi chiama non passa mai questo caso.
        Evaluation::Mate(_) => 0,
    };
    match color {
        Color::White => for_white,
        Color::Black => -for_white,
    }
}

/// La valutazione come numero, come nei siti di scacchi: `+0.4`, `−1.2`, `M3`.
#[must_use]
pub fn evaluation_number(evaluation: Evaluation) -> String {
    match evaluation {
        Evaluation::Centipawns(value) => {
            let text = format!("{:+.1}", f64::from(value) / 100.0);
            text.replace('-', "−")
        }
        Evaluation::Mate(moves) => format!("M{}", moves.abs()),
    }
}

/// La valutazione a parole, ad esempio "Il Bianco sta leggermente meglio (+0.6)".
#[must_use]
pub fn describe_evaluation(evaluation: Evaluation) -> String {
    let number = evaluation_number(evaluation);
    match evaluation {
        Evaluation::Mate(0) => "Scacco matto".to_owned(),
        Evaluation::Mate(moves) => {
            let side = side_name_capitalized(if moves > 0 {
                Color::White
            } else {
                Color::Black
            });
            let count = moves.abs();
            let unit = if count == 1 { "mossa" } else { "mosse" };
            format!("{side} può dare matto in {count} {unit}")
        }
        Evaluation::Centipawns(value) => {
            let side = side_name_capitalized(if value > 0 {
                Color::White
            } else {
                Color::Black
            });
            match value.abs() {
                0..=30 => format!("La posizione è in equilibrio ({number})"),
                31..=100 => format!("{side} sta leggermente meglio ({number})"),
                101..=250 => format!("{side} sta meglio ({number})"),
                251..=600 => format!("{side} ha un grande vantaggio ({number})"),
                _ => format!("{side} sta vincendo ({number})"),
            }
        }
    }
}

/// Il giudizio su una mossa provata, confrontando la valutazione prima e dopo.
///
/// - Riceve: la valutazione prima della mossa (con il gioco migliore), quella
///   dopo la mossa e il colore di chi l'ha giocata.
/// - Restituisce: una frase come "Errore: la posizione peggiora parecchio".
#[must_use]
pub fn move_verdict(before: Evaluation, after: Evaluation, mover: Color) -> String {
    let loss = score_for(before, mover) - score_for(after, mover);
    let still_winning = score_for(after, mover) >= 500;
    match loss {
        _ if loss <= 30 => "Ottima scelta: è tra le mosse migliori".to_owned(),
        _ if still_winning && loss < 400 => "Va bene: resti in netto vantaggio".to_owned(),
        _ if loss <= 80 => "Mossa giocabile, ma c'era di meglio".to_owned(),
        _ if loss <= 200 => "Imprecisione: la posizione peggiora un po'".to_owned(),
        _ if loss <= 400 => "Errore: la posizione peggiora parecchio".to_owned(),
        _ => "Errore grave: la posizione cambia completamente".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use engine::Evaluation;
    use shakmaty::Color;

    use super::{describe_evaluation, evaluation_number, move_verdict};

    #[test]
    fn writes_the_evaluation_in_words() {
        assert_eq!(
            describe_evaluation(Evaluation::Centipawns(12)),
            "La posizione è in equilibrio (+0.1)"
        );
        assert_eq!(
            describe_evaluation(Evaluation::Centipawns(-180)),
            "Il Nero sta meglio (−1.8)"
        );
        assert_eq!(
            describe_evaluation(Evaluation::Mate(1)),
            "Il Bianco può dare matto in 1 mossa"
        );
        assert_eq!(evaluation_number(Evaluation::Mate(-3)), "M3");
    }

    #[test]
    fn judges_a_move_from_the_mover_point_of_view() {
        let before = Evaluation::Centipawns(-30);

        assert_eq!(
            move_verdict(before, Evaluation::Centipawns(-20), Color::Black),
            "Ottima scelta: è tra le mosse migliori"
        );
        assert_eq!(
            move_verdict(before, Evaluation::Centipawns(320), Color::Black),
            "Errore: la posizione peggiora parecchio"
        );
        assert_eq!(
            move_verdict(
                Evaluation::Centipawns(900),
                Evaluation::Centipawns(650),
                Color::White
            ),
            "Va bene: resti in netto vantaggio"
        );
    }
}
