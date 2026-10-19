//! La posizione codificata come la vuole la rete: 112 "piani" di 64 case.
//!
//! È la codifica "classica" di Lc0 (`INPUT_CLASSICAL_112_PLANE`), con cui
//! Maia è stata addestrata. Tutto è visto **da chi deve muovere**: "nostri"
//! sono i suoi pezzi e, se muove il Nero, la scacchiera è ribaltata (la riga 8
//! diventa la riga 1), così per la rete chi muove gioca sempre "dal basso".
//!
//! - Piani 0–103: le ultime 8 posizioni (la più recente per prima), 13 piani
//!   ciascuna: i 6 tipi di pezzi nostri (pedoni, cavalli, alfieri, torri,
//!   donne, re), i 6 loro, e un piano tutto 1 se la posizione si era già vista.
//!   Se la partita è cominciata dalla posizione iniziale, le posizioni che
//!   mancano restano vuote; se è cominciata da una FEN, si ripete la prima.
//! - Piani 104–107: tutto 1 se si può arroccare: noi lungo, noi corto, loro
//!   lungo, loro corto.
//! - Piano 108: tutto 1 se muove il Nero.
//! - Piano 109: le mezze mosse dall'ultima cattura o mossa di pedone (regola delle 50 mosse).
//! - Piano 110: tutto 0 (era il numero della mossa, oggi non usato).
//! - Piano 111: tutto 1 (aiuta la rete a riconoscere i bordi della scacchiera).
//!
//! Lc0, in una partita da FEN con una presa "en passant" possibile, rimette
//! indietro il pedone appena mosso nelle posizioni ripetute: qui non lo si fa,
//! perché cambia pochissimo e capita quasi mai.

use shakmaty::zobrist::Zobrist64;
use shakmaty::{Bitboard, Board, CastlingSide, Chess, Color, EnPassantMode, Piece, Position, Role};

/// Quanti piani ha l'ingresso della rete.
pub const PLANES: usize = 112;
/// Quante posizioni passate guarda la rete.
const HISTORY: usize = 8;
/// I piani di ogni posizione passata.
const PLANES_PER_POSITION: usize = 13;
/// Il primo piano dopo le posizioni passate (gli arrocchi).
const AUX_BASE: usize = HISTORY * PLANES_PER_POSITION;
/// I tipi di pezzo, nell'ordine dei piani.
const ROLES: [Role; 6] = [
    Role::Pawn,
    Role::Knight,
    Role::Bishop,
    Role::Rook,
    Role::Queen,
    Role::King,
];

/// I 112 piani uno dopo l'altro, 64 numeri ciascuno (da a1 a h8, riga per riga).
pub type InputPlanes = Vec<f32>;

/// Riempie un piano: 1 nelle case di `squares`, 0 nelle altre.
fn set_squares(planes: &mut [f32], plane: usize, squares: Bitboard) {
    for square in squares {
        planes[plane * 64 + usize::from(square)] = 1.0;
    }
}

/// Riempie un piano intero con lo stesso numero.
fn fill(planes: &mut [f32], plane: usize, value: f32) {
    planes[plane * 64..(plane + 1) * 64].fill(value);
}

/// I 13 piani di una posizione, visti da `us`, a partire dal piano `base`.
fn encode_position(planes: &mut [f32], base: usize, board: &Board, us: Color, repeated: bool) {
    let orient = |squares: Bitboard| {
        if us == Color::Black {
            squares.flip_vertical()
        } else {
            squares
        }
    };
    for (index, role) in ROLES.into_iter().enumerate() {
        set_squares(
            planes,
            base + index,
            orient(board.by_piece(Piece { color: us, role })),
        );
        set_squares(
            planes,
            base + 6 + index,
            orient(board.by_piece(Piece {
                color: us.other(),
                role,
            })),
        );
    }
    if repeated {
        fill(planes, base + 12, 1.0);
    }
}

/// Codifica la posizione attuale con quelle che l'hanno preceduta.
///
/// `history` va dalla posizione di partenza della partita (o da una qualsiasi
/// posizione precedente) a quella in cui si deve muovere, che è l'ultima.
///
/// # Panics
///
/// Se `history` è vuota (errore di programmazione).
#[must_use]
pub fn encode(history: &[Chess]) -> InputPlanes {
    let current = history.last().expect("serve almeno la posizione attuale");
    let us = current.turn();
    let hashes: Vec<Zobrist64> = history
        .iter()
        .map(|position| position.zobrist_hash(EnPassantMode::Legal))
        .collect();
    let mut planes = vec![0.0; PLANES * 64];

    for slot in 0..HISTORY {
        let index = match (history.len() - 1).checked_sub(slot) {
            Some(index) => index,
            // Le posizioni sono finite: dalla posizione iniziale restano vuote,
            // da una FEN si ripete la prima (come `FEN_ONLY` in Lc0).
            None if history[0].board() == &Board::new() => break,
            None => 0,
        };
        let repeated = hashes[..index].contains(&hashes[index]);
        encode_position(
            &mut planes,
            slot * PLANES_PER_POSITION,
            history[index].board(),
            us,
            repeated,
        );
    }

    let castles = current.castles();
    let rights = [
        (us, CastlingSide::QueenSide),
        (us, CastlingSide::KingSide),
        (us.other(), CastlingSide::QueenSide),
        (us.other(), CastlingSide::KingSide),
    ];
    for (offset, (color, side)) in rights.into_iter().enumerate() {
        if castles.has(color, side) {
            fill(&mut planes, AUX_BASE + offset, 1.0);
        }
    }
    if us == Color::Black {
        fill(&mut planes, AUX_BASE + 4, 1.0);
    }
    #[allow(clippy::cast_precision_loss)] // Il contatore delle 50 mosse è piccolo.
    fill(&mut planes, AUX_BASE + 5, current.halfmoves() as f32);
    fill(&mut planes, AUX_BASE + 7, 1.0);
    planes
}

#[cfg(test)]
mod tests {
    use shakmaty::{Chess, Position, uci::UciMove};

    use super::{AUX_BASE, PLANES_PER_POSITION, encode};

    /// La partita dalla posizione iniziale con le mosse indicate.
    fn history(moves: &[&str]) -> Vec<Chess> {
        let mut positions = vec![Chess::default()];
        for uci in moves {
            let last = positions.last().expect("posizione").clone();
            let chess_move = UciMove::from_ascii(uci.as_bytes())
                .expect("UCI")
                .to_move(&last)
                .expect("mossa legale");
            positions.push(last.play(chess_move).expect("mossa legale"));
        }
        positions
    }

    /// Il numero nella casa `square` (0 = a1, 63 = h8) del piano `plane`.
    fn at(planes: &[f32], plane: usize, square: usize) -> f32 {
        planes[plane * 64 + square]
    }

    #[test]
    fn the_side_to_move_always_plays_from_the_bottom() {
        let planes = encode(&history(&["e2e4"]));

        // Muove il Nero: i suoi pedoni (nostri, piano 0) sono sulla riga 2, ribaltata.
        assert_eq!(
            at(&planes, 0, 8 + 4),
            1.0,
            "pedone nero in e7, visto come e2"
        );
        // Il pedone bianco in e4 diventa, ribaltato, un pedone loro in e5.
        assert_eq!(at(&planes, 6, 32 + 4), 1.0);
        // Turno del Nero e arrocchi ancora possibili.
        assert_eq!(at(&planes, AUX_BASE + 4, 0), 1.0);
        assert_eq!(at(&planes, AUX_BASE + 1, 0), 1.0);
        // La posizione precedente c'è (piani 13–25): il pedone bianco era in e2,
        // ribaltato in e7. Quella prima ancora non c'è (si partiva dall'inizio).
        assert_eq!(at(&planes, PLANES_PER_POSITION + 6, 48 + 4), 1.0);
        assert_eq!(at(&planes, 2 * PLANES_PER_POSITION, 8), 0.0);
    }

    #[test]
    fn a_repeated_position_is_marked() {
        let planes = encode(&history(&["g1f3", "g8f6", "f3g1", "f6g8"]));

        assert_eq!(
            at(&planes, 12, 0),
            1.0,
            "la posizione attuale si era già vista"
        );
        assert_eq!(at(&planes, PLANES_PER_POSITION + 12, 0), 0.0);
    }
}
