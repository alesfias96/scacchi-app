//! Piccole domande sulla scacchiera, "dal punto di vista" di un colore.
//!
//! Le traverse si contano dal lato di chi gioca: la traversa 1 è quella di
//! partenza dei suoi pezzi (la prima per il Bianco, l'ottava per il Nero).
//! Così una regola come "pedone in d4" vale per il Bianco e, a specchio,
//! "pedone in d5" per il Nero, senza scriverla due volte.

use shakmaty::{Bitboard, Board, Color, File, Rank, Role, Square};

/// La casa sulla colonna `file` e sulla traversa `rank` (da 1 a 8) vista da `color`.
#[must_use]
pub fn relative_square(file: File, rank: u32, color: Color) -> Square {
    let index = rank.clamp(1, 8) - 1;
    let rank = match color {
        Color::White => Rank::new(index),
        Color::Black => Rank::new(7 - index),
    };
    Square::from_coords(file, rank)
}

/// La traversa di una casa (da 1 a 8) vista da `color`.
#[must_use]
pub fn relative_rank(square: Square, color: Color) -> u32 {
    let rank = u32::from(square.rank()) + 1;
    match color {
        Color::White => rank,
        Color::Black => 9 - rank,
    }
}

/// I pezzi di un tipo e di un colore.
#[must_use]
pub fn pieces(board: &Board, color: Color, role: Role) -> Bitboard {
    board.by_role(role) & board.by_color(color)
}

/// `true` se c'è un pedone di `color` nella casa (vista da `color`).
#[must_use]
pub fn pawn_at(board: &Board, color: Color, file: File, rank: u32) -> bool {
    pieces(board, color, Role::Pawn).contains(relative_square(file, rank, color))
}

/// `true` se `color` ha almeno un pedone sulla colonna.
#[must_use]
pub fn has_pawn_on_file(board: &Board, color: Color, file: File) -> bool {
    (pieces(board, color, Role::Pawn) & Bitboard::from_file(file)).any()
}

/// `true` se sulla colonna non c'è nessun pedone (di nessuno dei due colori).
#[must_use]
pub fn is_open_file(board: &Board, file: File) -> bool {
    (board.pawns() & Bitboard::from_file(file)).is_empty()
}

/// Le colonne vicine (una o due).
#[must_use]
pub fn neighbour_files(file: File) -> Vec<File> {
    [file.offset(-1), file.offset(1)]
        .into_iter()
        .flatten()
        .collect()
}

/// `true` se il pedone di `color` in `square` è passato: nessun pedone nemico
/// davanti a lui, sulla sua colonna o su quelle vicine.
#[must_use]
pub fn is_passed_pawn(board: &Board, color: Color, square: Square) -> bool {
    let rank = relative_rank(square, color);
    let mut files = neighbour_files(square.file());
    files.push(square.file());
    !pieces(board, color.other(), Role::Pawn)
        .into_iter()
        .any(|enemy| files.contains(&enemy.file()) && relative_rank(enemy, color) > rank)
}

/// Il nome di una colonna ("a"…"h").
#[must_use]
pub fn file_name(file: File) -> char {
    file.char()
}

/// `true` se `square` è un avamposto per `color`: nella metà avversaria
/// (traverse 4–6 viste da `color`), difeso da un proprio pedone e senza pedoni
/// avversari che, avanzando, possano attaccarlo.
#[must_use]
pub fn is_outpost(board: &Board, square: Square, color: Color) -> bool {
    let rank = relative_rank(square, color);
    if !(4..=6).contains(&rank) {
        return false;
    }
    let own = pieces(board, color, Role::Pawn);
    let defended = (shakmaty::attacks::pawn_attacks(color.other(), square) & own).any();
    let files = neighbour_files(square.file());
    let chasers = pieces(board, color.other(), Role::Pawn)
        .into_iter()
        .any(|pawn| files.contains(&pawn.file()) && relative_rank(pawn, color) > rank);
    defended && !chasers
}

#[cfg(test)]
mod tests {
    use shakmaty::fen::Fen;
    use shakmaty::{CastlingMode, Chess, Color, File, Position, Square};

    use super::{is_open_file, is_passed_pawn, pawn_at, relative_rank, relative_square};

    fn board(fen: &str) -> shakmaty::Board {
        let position: Chess = fen
            .parse::<Fen>()
            .expect("FEN valida")
            .into_position(CastlingMode::Standard)
            .expect("posizione legale");
        position.board().clone()
    }

    #[test]
    fn counts_ranks_from_each_side() {
        assert_eq!(relative_square(File::D, 4, Color::White), Square::D4);
        assert_eq!(relative_square(File::D, 4, Color::Black), Square::D5);
        assert_eq!(relative_rank(Square::E7, Color::Black), 2);
    }

    #[test]
    fn finds_pawns_open_files_and_passed_pawns() {
        let board = board("4k3/8/8/2P5/8/8/5PP1/4K3 w - - 0 1");

        assert!(pawn_at(&board, Color::White, File::C, 5));
        assert!(is_open_file(&board, File::D));
        assert!(is_passed_pawn(&board, Color::White, Square::C5));
    }
}
