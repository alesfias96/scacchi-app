//! Il tipo di finale, deciso dai pezzi rimasti (i pedoni non contano).

use shakmaty::{Bitboard, Board, Color, Role};

use analysis::wording::side_name_capitalized;

use crate::board_query::pieces;
use crate::recognition::Recognition;
use crate::topic::Topic;

/// Finale di soli pedoni.
pub const PAWN_ENDGAME: &str = "pawn-endgame";
/// Finale di torre (una torre a testa).
pub const ROOK_ENDGAME: &str = "rook-endgame";
/// Finale di alfieri di colore contrario.
pub const OPPOSITE_BISHOPS: &str = "opposite-bishops";
/// Finale di alfieri dello stesso colore.
pub const SAME_COLOR_BISHOPS: &str = "same-color-bishops";
/// Finale di cavalli.
pub const KNIGHT_ENDGAME: &str = "knight-endgame";
/// Alfiere contro cavallo.
pub const BISHOP_VS_KNIGHT: &str = "bishop-vs-knight";
/// Finale di donne.
pub const QUEEN_ENDGAME: &str = "queen-endgame";
/// Torre contro pezzo minore.
pub const ROOK_VS_MINOR: &str = "rook-vs-minor";
/// Il matto con re e donna contro re solo.
pub const MATE_WITH_QUEEN: &str = "mate-with-queen";
/// Il matto con re e torre contro re solo.
pub const MATE_WITH_ROOK: &str = "mate-with-rook";

/// I pezzi (non pedoni, non re) di un colore: quanti per tipo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Army {
    knights: usize,
    bishops: usize,
    rooks: usize,
    queens: usize,
    pawns: usize,
}

impl Army {
    fn of(board: &Board, color: Color) -> Self {
        let count = |role| pieces(board, color, role).count();
        Self {
            knights: count(Role::Knight),
            bishops: count(Role::Bishop),
            rooks: count(Role::Rook),
            queens: count(Role::Queen),
            pawns: count(Role::Pawn),
        }
    }

    /// Quanti pezzi (escluso il re e i pedoni).
    fn pieces(self) -> usize {
        self.knights + self.bishops + self.rooks + self.queens
    }

    /// `true` se ha esattamente un pezzo, di quel tipo.
    fn only(self, role: Role) -> bool {
        let number = match role {
            Role::Knight => self.knights,
            Role::Bishop => self.bishops,
            Role::Rook => self.rooks,
            Role::Queen => self.queens,
            Role::Pawn | Role::King => 0,
        };
        number == 1 && self.pieces() == 1
    }

    /// `true` se ha un solo pezzo e questo è un cavallo o un alfiere.
    fn only_minor(self) -> bool {
        self.only(Role::Knight) || self.only(Role::Bishop)
    }
}

/// Chi ha più pedoni (nei finali "simmetrici" è chi può giocare per vincere).
fn pawn_leader(white: Army, black: Army) -> Option<Color> {
    match white.pawns.cmp(&black.pawns) {
        std::cmp::Ordering::Greater => Some(Color::White),
        std::cmp::Ordering::Less => Some(Color::Black),
        std::cmp::Ordering::Equal => None,
    }
}

/// `true` se tutti gli alfieri della scacchiera stanno su case dello stesso colore.
fn bishops_on_same_color(board: &Board) -> bool {
    let bishops = board.by_role(Role::Bishop);
    (bishops & Bitboard::LIGHT_SQUARES).is_empty() || (bishops & Bitboard::DARK_SQUARES).is_empty()
}

/// Il tipo di finale e la frase che lo descrive, oppure `None` se non è un finale riconosciuto.
fn classify(board: &Board) -> Option<(&'static str, Option<Color>, String)> {
    let white = Army::of(board, Color::White);
    let black = Army::of(board, Color::Black);
    let leader = pawn_leader(white, black);
    let no_pawns = white.pawns + black.pawns == 0;

    for (strong, weak, color) in [(white, black, Color::White), (black, white, Color::Black)] {
        let side = side_name_capitalized(color);
        if no_pawns && weak.pieces() == 0 && strong.only(Role::Queen) {
            return Some((
                MATE_WITH_QUEEN,
                Some(color),
                format!("{side} ha re e donna contro il re solo: può dare matto"),
            ));
        }
        if no_pawns && weak.pieces() == 0 && strong.only(Role::Rook) {
            return Some((
                MATE_WITH_ROOK,
                Some(color),
                format!("{side} ha re e torre contro il re solo: può dare matto"),
            ));
        }
        if strong.only(Role::Bishop) && weak.only(Role::Knight) {
            return Some((
                BISHOP_VS_KNIGHT,
                Some(color),
                format!(
                    "Finale di alfiere contro cavallo: l'alfiere è di {}",
                    analysis::wording::side_name(color)
                ),
            ));
        }
        if strong.only(Role::Rook) && weak.only_minor() {
            return Some((
                ROOK_VS_MINOR,
                Some(color),
                format!("{side} ha la torre contro un pezzo minore: è in vantaggio di materiale"),
            ));
        }
    }

    let both = |test: fn(Army) -> bool| test(white) && test(black);
    let found = if both(|army| army.pieces() == 0) && white.pawns + black.pawns > 0 {
        (
            PAWN_ENDGAME,
            "Finale di soli pedoni: ogni tempo e ogni casa del re contano",
        )
    } else if both(|army| army.only(Role::Rook)) {
        (
            ROOK_ENDGAME,
            "Finale di torre: il tipo di finale più frequente",
        )
    } else if both(|army| army.only(Role::Bishop)) && bishops_on_same_color(board) {
        (SAME_COLOR_BISHOPS, "Finale di alfieri dello stesso colore")
    } else if both(|army| army.only(Role::Bishop)) {
        (
            OPPOSITE_BISHOPS,
            "Finale di alfieri di colore contrario: spesso finisce patta",
        )
    } else if both(|army| army.only(Role::Knight)) {
        (
            KNIGHT_ENDGAME,
            "Finale di cavalli: assomiglia a un finale di pedoni",
        )
    } else if both(|army| army.only(Role::Queen)) {
        (
            QUEEN_ENDGAME,
            "Finale di donne: attenzione agli scacchi perpetui",
        )
    } else {
        return None;
    };
    Some((found.0, leader, found.1.to_owned()))
}

/// Il tipo di finale della posizione, se è uno di quelli riconosciuti.
#[must_use]
pub fn recognize_endgame(board: &Board) -> Option<Recognition> {
    classify(board).map(|(key, owner, text)| Recognition {
        topic: Topic::Endgame,
        key,
        owner,
        text,
        squares: Vec::new(),
    })
}

#[cfg(test)]
mod tests {
    use shakmaty::fen::Fen;
    use shakmaty::{CastlingMode, Chess, Color, Position};

    use super::{
        BISHOP_VS_KNIGHT, MATE_WITH_ROOK, OPPOSITE_BISHOPS, PAWN_ENDGAME, ROOK_ENDGAME,
        recognize_endgame,
    };

    fn kind(fen: &str) -> Option<(&'static str, Option<Color>)> {
        let position: Chess = fen
            .parse::<Fen>()
            .expect("FEN valida")
            .into_position(CastlingMode::Standard)
            .expect("posizione legale");
        recognize_endgame(position.board()).map(|found| (found.key, found.owner))
    }

    #[test]
    fn recognizes_the_common_endgames() {
        assert_eq!(
            kind("8/5k2/8/3p4/3P4/8/5K2/8 w - - 0 1"),
            Some((PAWN_ENDGAME, None))
        );
        assert_eq!(
            kind("8/5k2/r7/3p4/3P4/2P5/5K2/R7 w - - 0 1"),
            Some((ROOK_ENDGAME, Some(Color::White)))
        );
        assert_eq!(
            kind("8/5k2/2b5/8/8/8/3B1K2/8 w - - 0 1"),
            Some((OPPOSITE_BISHOPS, None))
        );
        assert_eq!(
            kind("8/5k2/2n5/8/8/8/2B2K2/8 w - - 0 1"),
            Some((BISHOP_VS_KNIGHT, Some(Color::White)))
        );
        assert_eq!(
            kind("8/5k2/8/8/8/8/5K2/R7 w - - 0 1"),
            Some((MATE_WITH_ROOK, Some(Color::White)))
        );
    }

    #[test]
    fn the_middlegame_is_not_an_endgame() {
        assert_eq!(
            kind("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
            None
        );
    }
}
