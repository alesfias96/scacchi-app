//! Da una mossa legale al suo posto tra le 1858 mosse della rete.
//!
//! La rete vede sempre chi muove "dal basso" (vedi `input_planes`): per il
//! Nero le case vanno ribaltate (e7 diventa e2). Inoltre la numerazione di Lc0
//! ha due particolarità:
//! - l'**arrocco** è scritto come "il re va sulla casa della torre" (O-O del
//!   Bianco = e1h1, O-O-O = e1a1);
//! - la **promozione a cavallo** è la mossa senza lettera (a7a8); donna, torre
//!   e alfiere hanno la lettera (a7a8q, a7a8r, a7a8b).

use std::sync::OnceLock;

use shakmaty::{Color, Move, Role, Square};

use crate::policy_tables::MOVES;

/// I possibili pezzi di promozione, con la loro lettera (nessuna = cavallo o nessuna promozione).
const PROMOTIONS: [Option<char>; 4] = [None, Some('q'), Some('r'), Some('b')];

/// La chiave di una mossa nella tabella: promozione, casa di partenza, casa di arrivo.
fn key(promotion: usize, from: usize, to: usize) -> usize {
    (promotion * 64 + from) * 64 + to
}

/// Il numero (0–63) di una casa scritta come "e2".
fn square_number(name: &str) -> usize {
    let bytes = name.as_bytes();
    usize::from(bytes[1] - b'1') * 8 + usize::from(bytes[0] - b'a')
}

/// La tabella che dà, per ogni chiave, il posto della mossa in [`MOVES`]
/// (costruita una volta sola, alla prima richiesta).
fn table() -> &'static [Option<u16>] {
    static TABLE: OnceLock<Vec<Option<u16>>> = OnceLock::new();
    TABLE.get_or_init(|| {
        let mut table = vec![None; key(PROMOTIONS.len(), 0, 0)];
        for (index, uci) in MOVES.iter().enumerate() {
            let promotion = PROMOTIONS
                .iter()
                .position(|letter| *letter == uci.chars().nth(4))
                .unwrap_or(0);
            let slot = key(
                promotion,
                square_number(&uci[0..2]),
                square_number(&uci[2..4]),
            );
            table[slot] = u16::try_from(index).ok();
        }
        table
    })
}

/// Il posto della mossa tra le 1858 della rete, giocata da `us`; `None` per
/// le mosse che la rete non conosce (non succede con le mosse degli scacchi normali).
#[must_use]
pub fn nn_index(chess_move: Move, us: Color) -> Option<usize> {
    let (from, to, promotion): (Square, Square, Option<Role>) = match chess_move {
        Move::Normal {
            from,
            to,
            promotion,
            ..
        } => (from, to, promotion),
        Move::EnPassant { from, to } => (from, to, None),
        Move::Castle { king, rook } => (king, rook, None),
        Move::Put { .. } => return None,
    };
    let orient = |square: Square| {
        if us == Color::Black {
            square.flip_vertical()
        } else {
            square
        }
    };
    let letter = match promotion {
        Some(Role::Queen) => 1,
        Some(Role::Rook) => 2,
        Some(Role::Bishop) => 3,
        _ => 0,
    };
    table()[key(letter, usize::from(orient(from)), usize::from(orient(to)))].map(usize::from)
}

#[cfg(test)]
mod tests {
    use shakmaty::{Color, Move, Role, Square};

    use super::nn_index;
    use crate::policy_tables::MOVES;

    #[test]
    fn every_move_of_the_table_is_found_again() {
        assert_eq!(
            nn_index(
                Move::Normal {
                    role: Role::Pawn,
                    from: Square::E2,
                    capture: None,
                    to: Square::E4,
                    promotion: None,
                },
                Color::White
            )
            .map(|index| MOVES[index]),
            Some("e2e4")
        );
    }

    #[test]
    fn the_black_moves_are_seen_from_the_bottom() {
        let index = nn_index(
            Move::Normal {
                role: Role::Pawn,
                from: Square::E7,
                capture: None,
                to: Square::E5,
                promotion: None,
            },
            Color::Black,
        );
        assert_eq!(index.map(|index| MOVES[index]), Some("e2e4"));
    }

    #[test]
    fn castling_and_promotions_follow_the_lc0_numbering() {
        let castle = nn_index(
            Move::Castle {
                king: Square::E8,
                rook: Square::H8,
            },
            Color::Black,
        );
        assert_eq!(castle.map(|index| MOVES[index]), Some("e1h1"));

        let promote = |promotion| {
            nn_index(
                Move::Normal {
                    role: Role::Pawn,
                    from: Square::A7,
                    capture: None,
                    to: Square::A8,
                    promotion: Some(promotion),
                },
                Color::White,
            )
            .map(|index| MOVES[index])
        };
        assert_eq!(promote(Role::Queen), Some("a7a8q"));
        assert_eq!(promote(Role::Knight), Some("a7a8"));
    }
}
