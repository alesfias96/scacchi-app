//! I principi generali: regole semplici che valgono in quasi ogni partita.
//!
//! Ogni regola guarda un aspetto verificabile della posizione (pezzi ancora a
//! casa, re al centro, colonne aperte…) e, se il principio è in gioco, dice
//! a chi riguarda e perché.

use shakmaty::{Bitboard, Chess, Color, File, Position, Role, Square};

use analysis::GamePhase;
use analysis::material::material;
use analysis::wording::{color_adjective, side_name_capitalized};

use crate::board_query::{
    file_name, is_open_file, is_passed_pawn, pieces, relative_rank, relative_square,
};
use crate::recognition::Recognition;
use crate::topic::Topic;

/// Sviluppare i pezzi minori.
pub const DEVELOPMENT: &str = "development";
/// Mettere al sicuro il re con l'arrocco.
pub const CASTLE_KING: &str = "castle-king";
/// Il controllo del centro.
pub const CENTER: &str = "center";
/// Le torri sulle colonne aperte.
pub const OPEN_FILE: &str = "open-file";
/// La coppia degli alfieri.
pub const BISHOP_PAIR: &str = "bishop-pair";
/// Il re attivo nel finale.
pub const KING_ACTIVITY: &str = "king-activity";
/// I pedoni passati vanno spinti.
pub const PASSED_PAWN: &str = "passed-pawn";
/// Chi è avanti di materiale semplifica.
pub const TRADE_WHEN_AHEAD: &str = "trade-when-ahead";

/// Da questa mossa in poi si fa notare lo sviluppo mancante.
const DEVELOPMENT_FROM_MOVE: u32 = 5;
/// Da questa mossa in poi si fa notare il re ancora al centro.
const CASTLE_FROM_MOVE: u32 = 8;
/// Il vantaggio di materiale (in punti) da cui conviene semplificare.
const AHEAD_POINTS: i32 = 2;

fn principle(
    key: &'static str,
    owner: Option<Color>,
    text: String,
    squares: Vec<Square>,
) -> Recognition {
    Recognition {
        topic: Topic::Principle,
        key,
        owner,
        text,
        squares,
    }
}

/// I pezzi minori di `color` ancora sulle case di partenza.
fn undeveloped_minors(position: &Chess, color: Color) -> Vec<Square> {
    let board = position.board();
    [
        (Role::Knight, File::B),
        (Role::Knight, File::G),
        (Role::Bishop, File::C),
        (Role::Bishop, File::F),
    ]
    .into_iter()
    .map(|(role, file)| (role, relative_square(file, 1, color)))
    .filter(|(role, square)| pieces(board, color, *role).contains(*square))
    .map(|(_, square)| square)
    .collect()
}

/// I pedoni di `color` sulle quattro case centrali (d4, e4, d5, e5).
fn central_pawns(position: &Chess, color: Color) -> Vec<Square> {
    let center = [Square::D4, Square::E4, Square::D5, Square::E5];
    let pawns = pieces(position.board(), color, Role::Pawn);
    center
        .into_iter()
        .filter(|square| pawns.contains(*square))
        .collect()
}

/// Scrive un elenco di case: "d4", "d4 e e5", "c3, d4 e e5".
fn squares_text(squares: &[Square]) -> String {
    let names: Vec<String> = squares.iter().map(ToString::to_string).collect();
    analysis::wording::join_list(&names)
}

/// I principi in gioco nell'apertura.
fn opening_principles(position: &Chess, moves: u32, result: &mut Vec<Recognition>) {
    for color in [Color::White, Color::Black] {
        let side = side_name_capitalized(color);
        let waiting = undeveloped_minors(position, color);
        if moves >= DEVELOPMENT_FROM_MOVE && waiting.len() >= 2 {
            result.push(principle(
                DEVELOPMENT,
                Some(color),
                format!(
                    "{side} ha ancora {} pezzi minori sulle case di partenza ({})",
                    waiting.len(),
                    squares_text(&waiting)
                ),
                waiting,
            ));
        }
    }
}

/// I principi in gioco finché ci sono molti pezzi (apertura e mediogioco).
fn middlegame_principles(
    position: &Chess,
    phase: GamePhase,
    moves: u32,
    result: &mut Vec<Recognition>,
) {
    let board = position.board();
    let open_files: Vec<File> = File::ALL
        .into_iter()
        .filter(|file| is_open_file(board, *file))
        .collect();

    let white_center = central_pawns(position, Color::White);
    let black_center = central_pawns(position, Color::Black);
    let center_leader = match white_center.len().cmp(&black_center.len()) {
        std::cmp::Ordering::Greater => Some((Color::White, white_center)),
        std::cmp::Ordering::Less => Some((Color::Black, black_center)),
        std::cmp::Ordering::Equal => None,
    };
    if let Some((color, squares)) = center_leader {
        result.push(principle(
            CENTER,
            Some(color),
            format!(
                "{} ha più pedoni al centro ({}): più spazio e più case per i suoi pezzi",
                side_name_capitalized(color),
                squares_text(&squares)
            ),
            squares,
        ));
    }

    for color in [Color::White, Color::Black] {
        let king_home = relative_square(File::E, 1, color);
        if moves >= CASTLE_FROM_MOVE
            && board.king_of(color) == Some(king_home)
            && position.castles().has_color(color)
        {
            result.push(principle(
                CASTLE_KING,
                Some(color),
                format!(
                    "Il re {} è ancora al centro: arroccare lo metterebbe al sicuro",
                    color_adjective(color, Role::King)
                ),
                vec![king_home],
            ));
        }

        let rooks = pieces(board, color, Role::Rook);
        if phase == GamePhase::Middlegame
            && rooks.any()
            && let Some(file) = open_files.first()
            && !rooks
                .into_iter()
                .any(|rook| open_files.contains(&rook.file()))
        {
            result.push(principle(
                OPEN_FILE,
                Some(color),
                format!(
                    "La colonna {} è aperta e nessuna torre {} la occupa ancora",
                    file_name(*file),
                    color_adjective(color, Role::Rook)
                ),
                Vec::new(),
            ));
        }
    }
}

/// I principi del finale.
fn endgame_principles(position: &Chess, result: &mut Vec<Recognition>) {
    let board = position.board();
    for color in [Color::White, Color::Black] {
        let side = side_name_capitalized(color);
        if let Some(king) = board.king_of(color)
            && board.queens().is_empty()
            && board.pawns().any()
            && relative_rank(king, color) == 1
        {
            result.push(principle(
                KING_ACTIVITY,
                Some(color),
                format!(
                    "Il re {} è ancora sulla prima traversa: nel finale va portato verso il centro",
                    color_adjective(color, Role::King)
                ),
                vec![king],
            ));
        }

        let passed = pieces(board, color, Role::Pawn)
            .into_iter()
            .filter(|pawn| is_passed_pawn(board, color, *pawn))
            .max_by_key(|pawn| relative_rank(*pawn, color));
        if let Some(pawn) = passed {
            result.push(principle(
                PASSED_PAWN,
                Some(color),
                format!("{side} ha un pedone passato in {pawn}: va spinto, scortato dai pezzi"),
                vec![pawn],
            ));
        }
    }
}

/// I principi in gioco nella posizione.
#[must_use]
pub fn recognize_principles(position: &Chess, phase: GamePhase) -> Vec<Recognition> {
    let board = position.board();
    let moves = position.fullmoves().get();
    let mut result = Vec::new();
    match phase {
        GamePhase::Opening => {
            opening_principles(position, moves, &mut result);
            middlegame_principles(position, phase, moves, &mut result);
        }
        GamePhase::Middlegame => middlegame_principles(position, phase, moves, &mut result),
        GamePhase::Endgame => endgame_principles(position, &mut result),
    }

    // La coppia degli alfieri: due alfieri (su case di colore diverso) contro meno di due.
    let pair = |color: Color| {
        let bishops = pieces(board, color, Role::Bishop);
        (bishops & Bitboard::LIGHT_SQUARES).any() && (bishops & Bitboard::DARK_SQUARES).any()
    };
    for color in [Color::White, Color::Black] {
        if pair(color) && pieces(board, color.other(), Role::Bishop).count() < 2 {
            result.push(principle(
                BISHOP_PAIR,
                Some(color),
                format!(
                    "{} ha la coppia degli alfieri",
                    side_name_capitalized(color)
                ),
                pieces(board, color, Role::Bishop).into_iter().collect(),
            ));
        }
    }

    // In vantaggio di materiale: conviene semplificare.
    let advantage = material(board).advantage;
    if phase != GamePhase::Opening && advantage.abs() >= AHEAD_POINTS {
        let color = if advantage > 0 {
            Color::White
        } else {
            Color::Black
        };
        result.push(principle(
            TRADE_WHEN_AHEAD,
            Some(color),
            format!(
                "{} è avanti di {} punti di materiale: ogni scambio di pezzi lo avvicina alla vittoria",
                side_name_capitalized(color),
                advantage.abs()
            ),
            Vec::new(),
        ));
    }
    result
}

#[cfg(test)]
mod tests {
    use shakmaty::fen::Fen;
    use shakmaty::{CastlingMode, Chess, Color};

    use super::{CASTLE_KING, DEVELOPMENT, KING_ACTIVITY, PASSED_PAWN, recognize_principles};
    use analysis::GamePhase;

    fn keys(fen: &str, phase: GamePhase) -> Vec<(&'static str, Option<Color>)> {
        let position: Chess = fen
            .parse::<Fen>()
            .expect("FEN valida")
            .into_position(CastlingMode::Standard)
            .expect("posizione legale");
        recognize_principles(&position, phase)
            .into_iter()
            .map(|found| (found.key, found.owner))
            .collect()
    }

    #[test]
    fn notices_missing_development_and_an_uncastled_king() {
        // Il Nero ha mosso solo pedoni e la donna: alla mossa 8 ha ancora tutto a casa.
        let found = keys(
            "rnb1kbnr/ppp2ppp/8/3pq3/8/2N2N2/PPPPBPPP/R1BQ1RK1 b kq - 1 8",
            GamePhase::Opening,
        );

        assert!(found.contains(&(DEVELOPMENT, Some(Color::Black))));
        assert!(found.contains(&(CASTLE_KING, Some(Color::Black))));
        assert!(!found.contains(&(CASTLE_KING, Some(Color::White))));
    }

    #[test]
    fn in_the_endgame_the_king_must_be_active_and_passed_pawns_pushed() {
        let found = keys("8/8/4k3/2P5/8/8/6p1/4K3 w - - 0 40", GamePhase::Endgame);

        assert!(found.contains(&(KING_ACTIVITY, Some(Color::White))));
        assert!(found.contains(&(PASSED_PAWN, Some(Color::White))));
        assert!(found.contains(&(PASSED_PAWN, Some(Color::Black))));
    }
}
