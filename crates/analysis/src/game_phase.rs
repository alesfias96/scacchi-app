//! La fase di gioco: apertura, mediogioco o finale.

use shakmaty::{Chess, Position, Role};

/// La fase della partita.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub enum GamePhase {
    /// Le prime mosse: si sviluppano i pezzi e si mette al sicuro il re.
    Opening,
    /// La parte centrale, con molti pezzi in gioco.
    Middlegame,
    /// Pochi pezzi rimasti: il re diventa un pezzo attivo e i pedoni contano di più.
    Endgame,
}

/// Il "peso" dei pezzi per stabilire la fase: cavallo e alfiere 1, torre 2, donna 4.
/// Con tutti i pezzi in gioco la somma è 24.
const PHASE_WEIGHTS: [(Role, u32); 4] = [
    (Role::Knight, 1),
    (Role::Bishop, 1),
    (Role::Rook, 2),
    (Role::Queen, 4),
];
/// Con una somma fino a questa si è nel finale (es. due torri e un alfiere a testa = 10).
const ENDGAME_MAX_WEIGHT: u32 = 10;
/// L'apertura dura al più fino a questa mossa…
const OPENING_LAST_MOVE: u32 = 12;
/// …e solo finché quasi tutti i pezzi sono ancora in gioco.
const OPENING_MIN_WEIGHT: u32 = 20;

/// La somma dei pesi dei pezzi rimasti (di entrambi i giocatori).
fn phase_weight(position: &Chess) -> u32 {
    let board = position.board();
    PHASE_WEIGHTS
        .iter()
        .map(|(role, weight)| u32::try_from(board.by_role(*role).count()).unwrap_or(0) * weight)
        .sum()
}

/// La fase di gioco della posizione.
///
/// Si guarda soprattutto il materiale rimasto: con pochi pezzi è un finale,
/// qualunque sia il numero della mossa. L'apertura sono le prime mosse, finché
/// quasi tutti i pezzi sono ancora sulla scacchiera.
#[must_use]
pub fn game_phase(position: &Chess) -> GamePhase {
    let weight = phase_weight(position);
    if weight <= ENDGAME_MAX_WEIGHT {
        GamePhase::Endgame
    } else if position.fullmoves().get() <= OPENING_LAST_MOVE && weight >= OPENING_MIN_WEIGHT {
        GamePhase::Opening
    } else {
        GamePhase::Middlegame
    }
}

#[cfg(test)]
mod tests {
    use super::{GamePhase, game_phase};
    use crate::test_position::position;

    #[test]
    fn recognises_the_three_phases() {
        let start = position("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let later = position("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 20");
        let ending = position("4k3/pp3r2/8/8/8/8/PP3R2/4K3 w - - 0 40");

        assert_eq!(game_phase(&start), GamePhase::Opening);
        assert_eq!(game_phase(&later), GamePhase::Middlegame);
        assert_eq!(game_phase(&ending), GamePhase::Endgame);
    }
}
