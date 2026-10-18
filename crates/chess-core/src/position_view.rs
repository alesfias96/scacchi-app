//! La descrizione completa di una posizione, pronta per l'interfaccia.

use shakmaty::fen::Fen;
use shakmaty::uci::UciMove;
use shakmaty::{Chess, EnPassantMode, Position};

use crate::legal_move::LegalMove;
use crate::side::Side;

/// Tutto ciò che l'interfaccia deve sapere su una posizione.
///
/// È una "fotografia": contiene solo dati già calcolati, così l'interfaccia
/// non deve conoscere le regole degli scacchi.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct PositionView {
    /// La posizione completa in notazione FEN (pezzi, turno, arrocchi, en passant, contatori).
    pub fen: String,
    /// Solo la disposizione dei pezzi (prima parte della FEN), usata dalla scacchiera grafica.
    pub board_fen: String,
    /// Il colore che deve muovere.
    pub turn: Side,
    /// `true` se il re del giocatore di turno è sotto scacco.
    pub in_check: bool,
    /// Tutte le mosse legali del giocatore di turno.
    pub legal_moves: Vec<LegalMove>,
}

impl PositionView {
    /// Crea la descrizione di una posizione di `shakmaty`.
    ///
    /// - Riceve: una posizione valida.
    /// - Restituisce: la [`PositionView`] con FEN, turno, scacco e mosse legali.
    #[must_use]
    pub fn from_position(position: &Chess) -> Self {
        let fen = Fen::from_position(position, EnPassantMode::Legal).to_string();

        // La FEN è formata da parti separate da spazi: la prima è la scacchiera.
        let board_fen = fen.split(' ').next().unwrap_or_default().to_owned();

        let legal_moves = position
            .legal_moves()
            .into_iter()
            .map(UciMove::from_standard)
            .filter_map(|uci_move| LegalMove::from_uci(&uci_move))
            .collect();

        Self {
            fen,
            board_fen,
            turn: Side::from(position.turn()),
            in_check: position.is_check(),
            legal_moves,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PositionView;
    use crate::side::Side;
    use shakmaty::Chess;

    #[test]
    fn describes_the_starting_position() {
        let view = PositionView::from_position(&Chess::default());

        assert_eq!(
            view.fen,
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        );
        assert_eq!(
            view.board_fen,
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"
        );
        assert_eq!(view.turn, Side::White);
        assert!(!view.in_check);
        assert_eq!(view.legal_moves.len(), 20);
    }
}
