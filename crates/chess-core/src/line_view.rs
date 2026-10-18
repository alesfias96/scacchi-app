//! Una sequenza di mosse a partire da una posizione, scritta come nei libri.
//!
//! Serve a mostrare la "variante principale" proposta dal motore: il motore
//! la comunica in notazione UCI (`e2e4 e7e5 g1f3`), che una persona legge a
//! fatica; qui diventa notazione SAN (`e4 e5 Nf3`), con il numero di mossa
//! da cui parte. L'interfaccia la traduce poi nella notazione scelta.

use shakmaty::fen::Fen;
use shakmaty::san::SanPlus;
use shakmaty::uci::UciMove;
use shakmaty::{Chess, EnPassantMode, Position};

use crate::game_error::GameError;
use crate::legal_move::LegalMove;
use crate::setup::parse_fen;
use crate::side::Side;

/// Una sequenza di mosse pronta da mostrare.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct LineView {
    /// La posizione da cui parte la sequenza, in notazione FEN.
    pub fen: String,
    /// Chi gioca la prima mossa della sequenza.
    pub turn: Side,
    /// Il numero (come nei libri) della prima mossa della sequenza.
    pub move_number: u32,
    /// Le mosse in notazione algebrica standard internazionale (es. `["e4", "e5", "Nf3"]`).
    pub san: Vec<String>,
    /// La prima mossa, con le sue case (per disegnare la freccia sulla scacchiera).
    pub first_move: Option<LegalMove>,
}

/// Descrive una sequenza di mosse.
///
/// - Riceve: la FEN di partenza della partita, le mosse già giocate
///   (`history`, in UCI) e la sequenza da descrivere (`line`, in UCI).
/// - Restituisce: la sequenza in SAN, a partire dalla posizione raggiunta
///   dopo `history`. Se una mossa della sequenza non è legale, la sequenza
///   si ferma prima di quella mossa (il motore a volte manda varianti tagliate).
///
/// # Errors
///
/// - [`GameError::InvalidPosition`] se la FEN di partenza non è valida;
/// - [`GameError::IllegalMove`] se una mossa di `history` non è legale.
pub fn describe_line(
    start_fen: &str,
    history: &[String],
    line: &[String],
) -> Result<LineView, GameError> {
    let mut position = parse_fen(start_fen)?;
    for uci in history {
        let chess_move =
            to_move(&position, uci).ok_or_else(|| GameError::IllegalMove(uci.clone()))?;
        position.play_unchecked(chess_move);
    }

    let fen = Fen::from_position(&position, EnPassantMode::Legal).to_string();
    let turn = Side::from(position.turn());
    let move_number = position.fullmoves().get();
    let first_move = line
        .first()
        .and_then(|uci| uci.parse::<UciMove>().ok())
        .filter(|uci| uci.to_move(&position).is_ok())
        .and_then(|uci| LegalMove::from_uci(&uci));

    let mut san = Vec::with_capacity(line.len());
    for uci in line {
        let Some(chess_move) = to_move(&position, uci) else {
            break;
        };
        san.push(SanPlus::from_move_and_play_unchecked(&mut position, chess_move).to_string());
    }

    Ok(LineView {
        fen,
        turn,
        move_number,
        san,
        first_move,
    })
}

/// La mossa UCI `uci`, se è scritta bene e legale in `position`.
fn to_move(position: &Chess, uci: &str) -> Option<shakmaty::Move> {
    uci.parse::<UciMove>().ok()?.to_move(position).ok()
}

#[cfg(test)]
mod tests {
    use super::describe_line;
    use crate::game_error::GameError;
    use crate::side::Side;

    const START: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    fn moves(text: &str) -> Vec<String> {
        text.split_whitespace().map(str::to_owned).collect()
    }

    #[test]
    fn writes_the_line_in_san_after_the_moves_played() {
        let line = describe_line(START, &moves("e2e4"), &moves("e7e5 g1f3 b8c6")).expect("valida");

        assert_eq!(line.san, ["e5", "Nf3", "Nc6"]);
        assert_eq!(line.turn, Side::Black);
        assert_eq!(line.move_number, 1);
        assert_eq!(line.first_move.expect("prima mossa").uci, "e7e5");
        assert_eq!(
            line.fen,
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1"
        );
    }

    #[test]
    fn stops_at_the_first_illegal_move_of_the_line() {
        let line = describe_line(START, &[], &moves("e2e4 e2e4 e7e5")).expect("valida");

        assert_eq!(line.san, ["e4"]);
    }

    #[test]
    fn marks_check_and_mate() {
        let line = describe_line(START, &moves("f2f3 e7e5 g2g4"), &moves("d8h4")).expect("valida");

        assert_eq!(line.san, ["Qh4#"]);
        assert_eq!(line.move_number, 2);
    }

    #[test]
    fn rejects_an_illegal_history() {
        assert_eq!(
            describe_line(START, &moves("e2e5"), &[]),
            Err(GameError::IllegalMove("e2e5".to_owned()))
        );
    }
}
