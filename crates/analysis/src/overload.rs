//! Il sovraccarico: un difensore che deve proteggere troppe cose.

use std::collections::BTreeMap;

use shakmaty::{Board, Color, Role, Square};

use crate::fact::{Fact, FactCategory};
use crate::wording::{join_list, of_piece_phrase, piece_phrase};

/// Trova i pezzi sovraccarichi.
///
/// Un pezzo è sovraccarico quando è l'**unico** difensore di due o più pezzi
/// attaccati dall'avversario: se si sposta per difenderne uno (o cattura),
/// l'altro resta senza protezione. Il fatto va a vantaggio di chi attacca.
#[must_use]
pub fn overload_facts(board: &Board) -> Vec<Fact> {
    let mut facts = Vec::new();
    for attacker in [Color::White, Color::Black] {
        let defender_color = attacker.other();
        // Per ogni difensore unico: i pezzi attaccati che difende da solo.
        let mut duties: BTreeMap<Square, Vec<Square>> = BTreeMap::new();
        for target in board.by_color(defender_color) {
            if board.role_at(target) == Some(Role::King) {
                continue;
            }
            let attacked = !board
                .attacks_to(target, attacker, board.occupied())
                .is_empty();
            if !attacked {
                continue;
            }
            let defenders = board.attacks_to(target, defender_color, board.occupied());
            if let (Some(only), true) = (defenders.first(), defenders.count() == 1) {
                duties.entry(only).or_default().push(target);
            }
        }
        for (defender_square, targets) in duties {
            if targets.len() < 2 {
                continue;
            }
            let Some(defender) = board.piece_at(defender_square) else {
                continue;
            };
            let names: Vec<String> = targets
                .iter()
                .filter_map(|target| {
                    board
                        .piece_at(*target)
                        .map(|piece| of_piece_phrase(piece, *target))
                })
                .collect();
            facts.push(
                Fact::new(
                    FactCategory::Tactics,
                    attacker,
                    format!(
                        "Sovraccarico: {} è l'unico difensore {}, {} attaccati",
                        piece_phrase(defender, defender_square),
                        join_list(&names),
                        if names.len() == 2 {
                            "entrambi"
                        } else {
                            "tutti"
                        }
                    ),
                )
                .with_squares(std::iter::once(defender_square).chain(targets.iter().copied())),
            );
        }
    }
    facts
}

#[cfg(test)]
mod tests {
    use super::overload_facts;
    use crate::test_position::{position, texts};
    use chess_core::Side;
    use shakmaty::Position;

    #[test]
    fn sees_a_defender_with_two_duties() {
        // Il cavallo nero in e7 è l'unico difensore dei cavalli in c6 e g6,
        // attaccati dalle torri bianche in c1 e g1.
        let game = position("4k3/4n3/2n3n1/8/8/8/8/2R1K1R1 w - - 0 1");

        let facts = overload_facts(game.board());

        assert_eq!(
            texts(&facts),
            [
                "Sovraccarico: il cavallo nero in e7 è l'unico difensore del cavallo nero in c6 e del cavallo nero in g6, entrambi attaccati"
            ]
        );
        assert_eq!(facts[0].side, Side::White);
    }
}
