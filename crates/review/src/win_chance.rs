//! La probabilità di vittoria e la precisione di una mossa.
//!
//! I centipedoni da soli ingannano: perdere due pedoni in una posizione già
//! vinta conta poco, perderli in una posizione pari conta moltissimo. Per
//! questo, come fa Lichess, si trasforma la valutazione in una probabilità di
//! vittoria (da 0 a 100) e si giudicano le mosse da quanta probabilità perdono.

use chess_core::Side;
use engine::Evaluation;

/// La pendenza della curva che trasforma i centipedoni in probabilità (la stessa di Lichess).
const CURVE: f64 = 0.003_682_08;

/// La probabilità di vittoria del Bianco, da 0 a 100, per una valutazione.
#[must_use]
pub fn white_win_chance(evaluation: Evaluation) -> f64 {
    match evaluation {
        Evaluation::Centipawns(value) => {
            let chances = 2.0 / (1.0 + (-CURVE * f64::from(value)).exp()) - 1.0;
            50.0 + 50.0 * chances
        }
        Evaluation::Mate(moves) if moves > 0 => 100.0,
        Evaluation::Mate(moves) if moves < 0 => 0.0,
        // "Matto in 0": il matto è già sulla scacchiera; chi chiama lo gestisce a parte.
        Evaluation::Mate(_) => 50.0,
    }
}

/// La probabilità di vittoria di `side`, data quella del Bianco.
#[must_use]
pub fn chance_for(white_chance: f64, side: Side) -> f64 {
    match side {
        Side::White => white_chance,
        Side::Black => 100.0 - white_chance,
    }
}

/// La precisione di una mossa (da 0 a 100) data la probabilità di vittoria persa
/// (la formula di Lichess: nessuna perdita = 100, una grossa perdita ≈ 0).
#[must_use]
pub fn move_accuracy(chance_lost: f64) -> f64 {
    (103.166_8 * (-0.043_54 * chance_lost).exp() - 3.166_9).clamp(0.0, 100.0)
}

#[cfg(test)]
mod tests {
    use chess_core::Side;
    use engine::Evaluation;

    use super::{chance_for, move_accuracy, white_win_chance};

    #[test]
    fn an_even_position_is_fifty_fifty() {
        assert!((white_win_chance(Evaluation::Centipawns(0)) - 50.0).abs() < 1e-9);
        assert!(white_win_chance(Evaluation::Centipawns(300)) > 75.0);
        assert!((white_win_chance(Evaluation::Mate(-2))).abs() < 1e-9);
        assert!((chance_for(70.0, Side::Black) - 30.0).abs() < 1e-9);
    }

    #[test]
    fn the_accuracy_falls_with_the_chance_lost() {
        assert!(move_accuracy(0.0) > 99.9);
        assert!(move_accuracy(10.0) > 60.0);
        assert!(move_accuracy(40.0) < 15.0);
    }
}
