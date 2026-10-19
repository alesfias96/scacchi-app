//! Gli scacchi matti in una mossa: quelli possibili e quelli minacciati.

use shakmaty::san::SanPlus;
use shakmaty::{Chess, Move, Position};

use crate::fact::{Fact, FactCategory};
use crate::wording::side_name_capitalized;

/// Quante mosse di matto mostrare al massimo per ogni fatto.
const MAX_MOVES: usize = 3;

/// Le mosse che danno scacco matto nella posizione, in notazione SAN.
fn mating_moves(position: &Chess) -> Vec<(Move, String)> {
    position
        .legal_moves()
        .into_iter()
        .filter_map(|chess_move| {
            let mut after = position.clone();
            after.play_unchecked(chess_move);
            after.is_checkmate().then(|| {
                let san = SanPlus::from_move(position.clone(), chess_move).to_string();
                (chess_move, san)
            })
        })
        .take(MAX_MOVES)
        .collect()
}

/// Costruisce il fatto "matto in una mossa".
fn mate_fact(position: &Chess, moves: &[(Move, String)], text: String) -> Fact {
    let mut fact = Fact::new(FactCategory::Threats, position.turn(), text);
    for (chess_move, san) in moves {
        if let Some(from) = chess_move.from() {
            fact = fact.with_arrow(from, chess_move.to());
        }
        fact.moves.push(san.clone());
    }
    fact
}

/// I matti in una mossa: quelli che può dare chi muove e quelli che
/// l'avversario minaccia (cioè che darebbe se fosse di nuovo il suo turno).
///
/// La minaccia si calcola "passando il turno" all'avversario. Se chi muove è
/// sotto scacco questo non si può fare (sarebbe una posizione impossibile):
/// in quel caso c'è solo da rispondere allo scacco.
#[must_use]
pub fn mate_facts(position: &Chess) -> Vec<Fact> {
    let mut facts = Vec::new();
    let available = mating_moves(position);
    if !available.is_empty() {
        let text = format!(
            "{} può dare scacco matto in una mossa con",
            side_name_capitalized(position.turn())
        );
        facts.push(mate_fact(position, &available, text));
    }
    if let Ok(swapped) = position.clone().swap_turn() {
        let threatened = mating_moves(&swapped);
        if !threatened.is_empty() {
            let text = format!(
                "{} minaccia scacco matto: se ne avesse l'occasione giocherebbe",
                side_name_capitalized(swapped.turn())
            );
            facts.push(mate_fact(&swapped, &threatened, text));
        }
    }
    facts
}

#[cfg(test)]
mod tests {
    use super::mate_facts;
    use crate::test_position::{position, texts};
    use chess_core::Side;

    #[test]
    fn finds_a_mate_in_one() {
        // Il Bianco matta con Dg7.
        let game = position("7k/8/6QK/8/8/8/8/8 w - - 0 1");

        let facts = mate_facts(&game);

        assert_eq!(
            texts(&facts)[0],
            "Il Bianco può dare scacco matto in una mossa con"
        );
        assert!(facts[0].moves.contains(&"Qg7#".to_owned()));
        assert_eq!(facts[0].side, Side::White);
    }

    #[test]
    fn warns_about_the_mate_threatened_by_the_opponent() {
        // La posizione del "matto del barbiere" con il Bianco al tratto: se fosse
        // il turno del Nero, Dh4 sarebbe matto. Il Bianco deve vederlo.
        let game = position("rnbqkbnr/pppp1ppp/8/4p3/6P1/5P2/PPPPP2P/RNBQKBNR w KQkq - 0 3");

        let facts = mate_facts(&game);

        assert_eq!(facts.len(), 1);
        assert_eq!(facts[0].side, Side::Black);
        assert_eq!(facts[0].moves, ["Qh4#"]);
    }
}
