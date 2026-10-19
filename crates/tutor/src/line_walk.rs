//! Rigiocare una variante del motore, mossa per mossa.
//!
//! Per capire l'idea di una mossa si guarda dove porta la variante di
//! Stockfish: qui la si rigioca e, per ogni mossa, si tiene chi l'ha giocata,
//! il pezzo, le case, cosa ha catturato e la posizione che ne risulta.

use shakmaty::san::{San, SanPlus};
use shakmaty::uci::UciMove;
use shakmaty::{Chess, Color, Move, Position, Role, Square};

/// Una mossa della variante, con la posizione dopo la mossa.
#[derive(Debug, Clone)]
pub struct LineStep {
    /// La mossa in notazione algebrica internazionale, con + o # (es. "Nf3", "Qxf7#").
    pub san: String,
    /// Chi l'ha giocata.
    pub mover: Color,
    /// Il pezzo mosso.
    pub role: Role,
    /// La casa di partenza (per l'arrocco: quella del re).
    pub from: Square,
    /// La casa di arrivo (per l'arrocco: quella in cui arriva il re).
    pub to: Square,
    /// Il pezzo catturato, se c'è.
    pub capture: Option<Role>,
    /// Il pezzo in cui si promuove, se c'è.
    pub promotion: Option<Role>,
    /// `true` per l'arrocco.
    pub is_castle: bool,
    /// La posizione dopo la mossa.
    pub after: Chess,
}

/// Le case di partenza e di arrivo del pezzo mosso, come le vede chi gioca.
fn squares(chess_move: Move, mover: Color) -> (Square, Square) {
    match chess_move {
        Move::Castle { king, rook } => {
            // Il re arriva sulla colonna g (arrocco corto) o c (lungo).
            let file = if rook > king { 6 } else { 2 };
            let rank = if mover == Color::White { 0 } else { 7 };
            (king, Square::new(rank * 8 + file))
        }
        other => (other.from().unwrap_or(other.to()), other.to()),
    }
}

/// Una mossa scritta in UCI ("g1f3") oppure in notazione algebrica ("Nf3", "Qxf7#").
fn parse_move(text: &str, position: &Chess) -> Option<Move> {
    UciMove::from_ascii(text.as_bytes())
        .ok()
        .and_then(|parsed| parsed.to_move(position).ok())
        .or_else(|| {
            let plain = text.trim_end_matches(['+', '#']);
            San::from_ascii(plain.as_bytes())
                .ok()
                .and_then(|parsed| parsed.to_move(position).ok())
        })
}

/// Rigioca la variante a partire da `start` (mosse in UCI o in notazione
/// algebrica), fermandosi alla prima mossa non valida.
#[must_use]
pub fn walk(start: &Chess, line: &[String]) -> Vec<LineStep> {
    let mut position = start.clone();
    let mut steps = Vec::new();
    for uci in line {
        let Some(chess_move) = parse_move(uci, &position) else {
            break;
        };
        let mover = position.turn();
        let (from, to) = squares(chess_move, mover);
        let san = SanPlus::from_move_and_play_unchecked(&mut position, chess_move).to_string();
        steps.push(LineStep {
            san,
            mover,
            role: chess_move.role(),
            from,
            to,
            capture: chess_move.capture(),
            promotion: chess_move.promotion(),
            is_castle: chess_move.is_castle(),
            after: position.clone(),
        });
    }
    steps
}

#[cfg(test)]
mod tests {
    use shakmaty::{Chess, Color, Role, Square};

    use super::walk;

    fn line(moves: &[&str]) -> Vec<String> {
        moves.iter().map(|uci| (*uci).to_owned()).collect()
    }

    #[test]
    fn replays_the_line_with_movers_pieces_and_captures() {
        let steps = walk(
            &Chess::default(),
            &line(&["e2e4", "d7d5", "e4d5", "zzzz", "g1f3"]),
        );

        assert_eq!(steps.len(), 3, "si ferma alla mossa non valida");
        assert_eq!(steps[1].mover, Color::Black);
        assert_eq!(steps[2].san, "exd5");
        assert_eq!(steps[2].capture, Some(Role::Pawn));
        assert_eq!((steps[2].from, steps[2].to), (Square::E4, Square::D5));
    }

    #[test]
    fn accepts_moves_written_as_in_books() {
        let steps = walk(
            &Chess::default(),
            &line(&["e4", "e5", "Qh5", "Nc6", "Bc4", "Nf6", "Qxf7#"]),
        );

        assert_eq!(steps.len(), 7);
        assert_eq!(steps[6].san, "Qxf7#");
    }

    #[test]
    fn castling_moves_the_king_to_its_square() {
        let steps = walk(
            &Chess::default(),
            &line(&["e2e4", "e7e5", "g1f3", "b8c6", "f1c4", "f8c5", "e1g1"]),
        );

        let castle = steps.last().expect("arrocco");
        assert!(castle.is_castle);
        assert_eq!((castle.from, castle.to), (Square::E1, Square::G1));
    }
}
