//! Il materiale: quanti pezzi ha ciascuno e chi è avanti.

use shakmaty::{Board, Color, Role};

use crate::fact::{Fact, FactCategory};
use crate::piece_value::piece_value;
use crate::wording::side_name_capitalized;

/// I pezzi di un giocatore (il re non si conta: ce n'è sempre uno).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct MaterialCount {
    /// I pedoni.
    pub pawns: u32,
    /// I cavalli.
    pub knights: u32,
    /// Gli alfieri.
    pub bishops: u32,
    /// Le torri.
    pub rooks: u32,
    /// Le donne.
    pub queens: u32,
    /// Il valore totale, in pedoni.
    pub points: u32,
}

/// Il materiale dei due giocatori.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct MaterialBalance {
    /// I pezzi del Bianco.
    pub white: MaterialCount,
    /// I pezzi del Nero.
    pub black: MaterialCount,
    /// Il vantaggio del Bianco in pedoni (negativo se è avanti il Nero).
    pub advantage: i32,
}

/// Conta i pezzi di un colore.
fn count(board: &Board, color: Color) -> MaterialCount {
    let of = |role| (board.by_role(role) & board.by_color(color)).count();
    let pieces = MaterialCount {
        pawns: u32::try_from(of(Role::Pawn)).unwrap_or(0),
        knights: u32::try_from(of(Role::Knight)).unwrap_or(0),
        bishops: u32::try_from(of(Role::Bishop)).unwrap_or(0),
        rooks: u32::try_from(of(Role::Rook)).unwrap_or(0),
        queens: u32::try_from(of(Role::Queen)).unwrap_or(0),
        points: 0,
    };
    let points = [
        (Role::Pawn, pieces.pawns),
        (Role::Knight, pieces.knights),
        (Role::Bishop, pieces.bishops),
        (Role::Rook, pieces.rooks),
        (Role::Queen, pieces.queens),
    ]
    .iter()
    .map(|(role, number)| piece_value(*role).unsigned_abs() * number)
    .sum();
    MaterialCount { points, ..pieces }
}

/// Il materiale della posizione.
#[must_use]
pub fn material(board: &Board) -> MaterialBalance {
    let white = count(board, Color::White);
    let black = count(board, Color::Black);
    let advantage =
        i32::try_from(white.points).unwrap_or(0) - i32::try_from(black.points).unwrap_or(0);
    MaterialBalance {
        white,
        black,
        advantage,
    }
}

/// I fatti sul materiale: chi è avanti e la coppia degli alfieri.
#[must_use]
pub fn material_facts(board: &Board) -> Vec<Fact> {
    let balance = material(board);
    let mut facts = Vec::new();
    if balance.advantage != 0 {
        let leader = if balance.advantage > 0 {
            Color::White
        } else {
            Color::Black
        };
        let amount = balance.advantage.unsigned_abs();
        let unit = if amount == 1 { "pedone" } else { "pedoni" };
        facts.push(Fact::new(
            FactCategory::Material,
            leader,
            format!(
                "{} ha più materiale: {amount} {unit} di vantaggio",
                side_name_capitalized(leader)
            ),
        ));
    }
    for (color, mine, theirs) in [
        (Color::White, balance.white, balance.black),
        (Color::Black, balance.black, balance.white),
    ] {
        if mine.bishops >= 2 && theirs.bishops < 2 {
            facts.push(Fact::new(
                FactCategory::Material,
                color,
                format!(
                    "{} ha la coppia degli alfieri: un piccolo vantaggio, soprattutto in posizioni aperte",
                    side_name_capitalized(color)
                ),
            ));
        }
    }
    facts
}

#[cfg(test)]
mod tests {
    use super::{material, material_facts};
    use crate::test_position::{position, texts};
    use shakmaty::Position;

    #[test]
    fn counts_the_starting_position_as_equal() {
        let start = position("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

        let balance = material(start.board());

        assert_eq!(balance.white.points, 39);
        assert_eq!(balance.advantage, 0);
        assert_eq!(material_facts(start.board()), Vec::new());
    }

    #[test]
    fn reports_who_is_ahead_and_the_bishop_pair() {
        // Il Nero ha perso un cavallo; il Bianco ha ancora i due alfieri.
        let game = position("r1bqkb1r/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

        let found = texts(&material_facts(game.board()));

        assert_eq!(
            found[0],
            "Il Bianco ha più materiale: 6 pedoni di vantaggio"
        );
        assert_eq!(found.len(), 1);
    }
}
