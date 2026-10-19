//! Le forchette: un pezzo che attacca due (o più) pezzi nemici insieme.

use shakmaty::san::SanPlus;
use shakmaty::{Board, Chess, Color, Position, Role, Square};

use crate::exchange::exchange_gain;
use crate::fact::{Fact, FactCategory};
use crate::piece_value::piece_value;
use crate::wording::{join_list, piece_phrase, side_name_capitalized};

/// Quante forchette possibili (mosse) mostrare al massimo.
const MAX_OPPORTUNITIES: usize = 3;

/// I pezzi nemici che il pezzo in `square` attacca "per davvero": il re (scacco),
/// i pezzi che valgono più di lui e quelli che si possono catturare guadagnando.
fn real_targets(board: &Board, square: Square) -> Vec<Square> {
    let Some(piece) = board.piece_at(square) else {
        return Vec::new();
    };
    let enemy = piece.color.other();
    (board.attacks_from(square) & board.by_color(enemy))
        .into_iter()
        .filter(|target| {
            board.role_at(*target).is_some_and(|role| {
                role == Role::King
                    || piece_value(role) > piece_value(piece.role)
                    || exchange_gain(board, *target, piece.color) > 0
            })
        })
        .collect()
}

/// `true` se il pezzo in `square` non si può catturare guadagnando: una
/// forchetta con un pezzo che si perde subito non è una vera forchetta.
fn is_safe(board: &Board, square: Square, color: Color) -> bool {
    exchange_gain(board, square, color.other()) == 0
}

/// La descrizione dei bersagli: "il re nero in e8 e la torre nera in a8".
fn targets_phrase(board: &Board, targets: &[Square]) -> String {
    let names: Vec<String> = targets
        .iter()
        .filter_map(|target| {
            board
                .piece_at(*target)
                .map(|piece| piece_phrase(piece, *target))
        })
        .collect();
    join_list(&names)
}

/// Le forchette già presenti sulla scacchiera, di entrambi i giocatori.
fn current_forks(board: &Board) -> Vec<Fact> {
    let mut facts = Vec::new();
    for square in board.occupied() {
        let Some(piece) = board.piece_at(square) else {
            continue;
        };
        if piece.role == Role::King {
            continue;
        }
        let targets = real_targets(board, square);
        if targets.len() < 2 || !is_safe(board, square, piece.color) {
            continue;
        }
        let mut fact = Fact::new(
            FactCategory::Tactics,
            piece.color,
            format!(
                "Forchetta: {} attacca {}",
                piece_phrase(piece, square),
                targets_phrase(board, &targets)
            ),
        )
        .with_squares(targets.iter().copied());
        for target in &targets {
            fact = fact.with_arrow(square, *target);
        }
        facts.push(fact);
    }
    facts
}

/// Le mosse con cui chi muove può fare una forchetta.
fn fork_opportunities(position: &Chess) -> Vec<Fact> {
    let mover = position.turn();
    let mut facts = Vec::new();
    for chess_move in position.legal_moves() {
        if chess_move.is_castle() {
            continue;
        }
        let destination = chess_move.to();
        let mut after = position.clone();
        after.play_unchecked(chess_move);
        let board = after.board();
        let targets = real_targets(board, destination);
        if targets.len() < 2 || !is_safe(board, destination, mover) {
            continue;
        }
        let Some(piece) = board.piece_at(destination) else {
            continue;
        };
        let san = SanPlus::from_move(position.clone(), chess_move).to_string();
        let mut fact = Fact::new(
            FactCategory::Tactics,
            mover,
            format!(
                "{} può fare una forchetta: {} attaccherebbe {}. La mossa è",
                side_name_capitalized(mover),
                piece_phrase(piece, destination),
                targets_phrase(board, &targets)
            ),
        )
        .with_squares(targets.iter().copied())
        .with_move(san);
        if let Some(from) = chess_move.from() {
            fact = fact.with_arrow(from, destination);
        }
        facts.push(fact);
        if facts.len() == MAX_OPPORTUNITIES {
            break;
        }
    }
    facts
}

/// Le forchette: quelle già sulla scacchiera e quelle che chi muove può fare.
#[must_use]
pub fn fork_facts(position: &Chess) -> Vec<Fact> {
    let mut facts = current_forks(position.board());
    facts.extend(fork_opportunities(position));
    facts
}

#[cfg(test)]
mod tests {
    use super::fork_facts;
    use crate::test_position::{position, texts};
    use chess_core::Side;

    #[test]
    fn sees_a_knight_fork_on_the_board() {
        // Il cavallo bianco in c7 attacca il re in e8 e la torre in a8.
        let game = position("r3k3/2N5/8/8/8/8/8/4K3 b - - 0 1");

        let facts = fork_facts(&game);

        assert_eq!(
            texts(&facts)[0],
            "Forchetta: il cavallo bianco in c7 attacca la torre nera in a8 e il re nero in e8"
        );
        assert_eq!(facts[0].side, Side::White);
    }

    #[test]
    fn suggests_a_move_that_forks() {
        // Cc7+ attacca re e torre.
        let game = position("r3k3/8/8/1N6/8/8/8/4K3 w - - 0 1");

        let facts = fork_facts(&game);

        let opportunity = facts
            .iter()
            .find(|fact| fact.moves == ["Nc7+"])
            .expect("forchetta trovata");
        assert!(
            opportunity
                .text
                .starts_with("Il Bianco può fare una forchetta")
        );
    }

    #[test]
    fn a_piece_that_can_be_taken_is_not_a_real_fork() {
        // Il cavallo in c7 attacca re e torre ma la donna nera in d8 lo cattura.
        let game = position("r2qk3/2N5/8/8/8/8/8/4K3 b - - 0 1");

        let facts = fork_facts(&game);

        assert!(facts.iter().all(|fact| !fact.text.starts_with("Forchetta")));
    }
}
