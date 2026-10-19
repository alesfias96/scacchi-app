//! I temi del mediogioco: arrocchi opposti, avamposti, torre in settima.

use shakmaty::attacks::pawn_attacks;
use shakmaty::{Chess, Color, File, Position, Role};

use analysis::GamePhase;
use analysis::wording::{piece_phrase_capitalized, side_name_capitalized};

use crate::board_query::{neighbour_files, pieces, relative_rank};
use crate::recognition::Recognition;
use crate::topic::Topic;

/// I re arroccati su lati opposti.
pub const OPPOSITE_CASTLING: &str = "opposite-castling";
/// Un cavallo su un avamposto.
pub const OUTPOST: &str = "outpost";
/// Una torre in settima traversa.
pub const SEVENTH_RANK: &str = "seventh-rank";

fn theme(
    key: &'static str,
    owner: Option<Color>,
    text: String,
    squares: Vec<shakmaty::Square>,
) -> Recognition {
    Recognition {
        topic: Topic::Middlegame,
        key,
        owner,
        text,
        squares,
    }
}

/// `true` se il re sta sull'ala di re (colonne f, g, h).
fn on_kingside(file: File) -> bool {
    file >= File::F
}

/// `true` se il re sta sull'ala di donna (colonne a, b, c).
fn on_queenside(file: File) -> bool {
    file <= File::C
}

/// I temi di mediogioco riconosciuti nella posizione.
#[must_use]
pub fn recognize_themes(position: &Chess, phase: GamePhase) -> Vec<Recognition> {
    let board = position.board();
    let mut result = Vec::new();
    if phase == GamePhase::Endgame {
        return result;
    }

    // Arrocchi opposti: un re sull'ala di re, l'altro sull'ala di donna, con le donne in gioco.
    if let (Some(white), Some(black)) = (board.king_of(Color::White), board.king_of(Color::Black))
        && board.queens().any()
        && ((on_kingside(white.file()) && on_queenside(black.file()))
            || (on_queenside(white.file()) && on_kingside(black.file())))
    {
        result.push(theme(
            OPPOSITE_CASTLING,
            None,
            "I re stanno su lati opposti: vince chi apre per primo le linee verso il re avversario"
                .to_owned(),
            vec![white, black],
        ));
    }

    for color in [Color::White, Color::Black] {
        let enemy_pawns = pieces(board, color.other(), Role::Pawn);
        let own_pawns = pieces(board, color, Role::Pawn);

        // Avamposto: cavallo nella metà campo avversaria, difeso da un pedone,
        // che nessun pedone nemico potrà mai cacciare.
        for knight in pieces(board, color, Role::Knight) {
            let rank = relative_rank(knight, color);
            let defended = (pawn_attacks(color.other(), knight) & own_pawns).any();
            let files = neighbour_files(knight.file());
            let can_be_chased = enemy_pawns
                .into_iter()
                .any(|pawn| files.contains(&pawn.file()) && relative_rank(pawn, color) > rank);
            if (5..=6).contains(&rank) && defended && !can_be_chased {
                let piece = shakmaty::Piece {
                    color,
                    role: Role::Knight,
                };
                result.push(theme(
                    OUTPOST,
                    Some(color),
                    format!(
                        "{} è su un avamposto: difeso da un pedone, nessun pedone nemico può cacciarlo",
                        piece_phrase_capitalized(piece, knight)
                    ),
                    vec![knight],
                ));
                break;
            }
        }

        // Torre in settima: attacca i pedoni rimasti a casa e chiude il re.
        if phase == GamePhase::Middlegame
            && let Some(rook) = pieces(board, color, Role::Rook)
                .into_iter()
                .find(|rook| relative_rank(*rook, color) == 7)
        {
            result.push(theme(
                SEVENTH_RANK,
                Some(color),
                format!(
                    "{} ha una torre in settima traversa",
                    side_name_capitalized(color)
                ),
                vec![rook],
            ));
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use shakmaty::fen::Fen;
    use shakmaty::{CastlingMode, Chess, Color};

    use super::{OPPOSITE_CASTLING, OUTPOST, recognize_themes};
    use analysis::GamePhase;

    fn keys(fen: &str) -> Vec<(&'static str, Option<Color>)> {
        let position: Chess = fen
            .parse::<Fen>()
            .expect("FEN valida")
            .into_position(CastlingMode::Standard)
            .expect("posizione legale");
        recognize_themes(&position, GamePhase::Middlegame)
            .into_iter()
            .map(|found| (found.key, found.owner))
            .collect()
    }

    #[test]
    fn finds_opposite_castling() {
        let found = keys("2kr1b1r/pppq1ppp/2n1bn2/4p3/4P3/2N1BN2/PPPQ1PPP/R4RK1 w - - 0 10");

        assert!(found.contains(&(OPPOSITE_CASTLING, None)));
    }

    #[test]
    fn finds_a_knight_outpost() {
        let found = keys("r2q1rk1/pp2bppp/3p1n2/3Np3/4P3/8/PPP2PPP/R2QKB1R w KQ - 0 10");

        assert!(found.contains(&(OUTPOST, Some(Color::White))));
    }
}
