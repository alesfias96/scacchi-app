//! I pezzi in presa: attaccati e non difesi abbastanza.

use shakmaty::{Board, Color, Role};

use crate::exchange::{exchange_gain, least_valuable_attacker};
use crate::fact::{Fact, FactCategory};
use crate::wording::{piece_phrase_capitalized, side_name};

/// Scrive un guadagno in pedoni: "1 pedone", "3 pedoni".
fn pawns(amount: i32) -> String {
    if amount == 1 {
        "1 pedone".to_owned()
    } else {
        format!("{amount} pedoni")
    }
}

/// Trova i pezzi in presa di entrambi i giocatori.
///
/// Un pezzo è "in presa" se l'avversario, cominciando a catturarlo, guadagna
/// materiale (secondo lo scambio statico): ad esempio un cavallo non difeso
/// attaccato da un pedone, o una torre difesa ma attaccata da un alfiere.
/// La freccia parte dal pezzo che conviene usare per catturare.
#[must_use]
pub fn hanging_facts(board: &Board) -> Vec<Fact> {
    let mut facts = Vec::new();
    for square in board.occupied() {
        let Some(piece) = board.piece_at(square) else {
            continue;
        };
        if piece.role == Role::King {
            continue;
        }
        let attacker: Color = piece.color.other();
        let gain = exchange_gain(board, square, attacker);
        if gain <= 0 {
            continue;
        }
        let Some((from, _)) = least_valuable_attacker(board, square, attacker, board.occupied())
        else {
            continue;
        };
        facts.push(
            Fact::new(
                FactCategory::Threats,
                attacker,
                format!(
                    "{} è in presa: {} può guadagnare {}",
                    piece_phrase_capitalized(piece, square),
                    side_name(attacker),
                    pawns(gain)
                ),
            )
            .with_squares([square])
            .with_arrow(from, square),
        );
    }
    facts
}

#[cfg(test)]
mod tests {
    use super::hanging_facts;
    use crate::test_position::{position, texts};
    use chess_core::Side;
    use shakmaty::Position;

    #[test]
    fn finds_an_undefended_piece() {
        let game = position("4k3/8/8/4n3/3P4/8/8/4K3 w - - 0 1");

        let facts = hanging_facts(game.board());

        assert_eq!(
            texts(&facts),
            ["Il cavallo nero in e5 è in presa: il Bianco può guadagnare 3 pedoni"]
        );
        assert_eq!(facts[0].side, Side::White);
        assert_eq!(facts[0].arrows[0].from, "d4");
    }

    #[test]
    fn a_well_defended_piece_is_not_in_danger() {
        let game = position("4k3/8/3p4/4p3/8/8/4Q3/4K3 w - - 0 1");

        assert_eq!(hanging_facts(game.board()), Vec::new());
    }
}
