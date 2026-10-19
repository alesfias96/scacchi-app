//! La lezione di un errore: quale passo del metodo di pensiero l'avrebbe evitato.
//!
//! Il metodo «Prima di muovere» (scheda `tactics/thinking-method`) ha quattro
//! passi: i tuoi scacchi, le tue catture, cosa minaccia l'avversario, il
//! controllo finale. Quando Stockfish preferiva un'altra mossa, si guarda:
//! - se la mossa migliore era uno **scacco** → il passo «i tuoi scacchi»;
//! - se era una **cattura** → il passo «le tue catture»;
//! - se dopo la mossa giocata l'avversario **vince materiale** → il passo
//!   «cosa minaccia l'avversario»;
//! - altrimenti la mossa migliore era tranquilla: confrontare più candidate.

use shakmaty::{Chess, Position};

use crate::idea_events::{IdeaKind, material_event};
use crate::line_walk::walk;

/// Quante mezze mosse dopo la mossa giocata si guardano per vedere se l'avversario vince materiale.
const REPLY_PLIES: usize = 4;

/// La lezione della mossa `played` (UCI) giocata in `before`, dato come
/// continua la partita secondo Stockfish (`after_moves`) e la variante della
/// mossa migliore (`best_line`).
#[must_use]
pub fn lesson(
    before: &Chess,
    played: &str,
    after_moves: &[String],
    best_line: &[String],
) -> String {
    let best = walk(before, &best_line[..best_line.len().min(1)]);
    if let Some(step) = best.first() {
        if step.after.is_check() {
            return "Il primo passo del metodo, «i tuoi scacchi», l'avrebbe trovata: la mossa migliore era uno scacco.".to_owned();
        }
        if step.capture.is_some() {
            return "Il passo «le tue catture» del metodo l'avrebbe trovata: la mossa migliore era una cattura.".to_owned();
        }
    }
    let line: Vec<String> = std::iter::once(played.to_owned())
        .chain(after_moves.iter().take(REPLY_PLIES).cloned())
        .collect();
    let steps = walk(before, &line);
    let opponent = before.turn().other();
    let loses_material = material_event(before, &steps, opponent, false)
        .is_some_and(|event| event.kind == IdeaKind::WinsMaterial);
    if loses_material {
        "Il passo «cosa minaccia l'avversario» l'avrebbe evitato: dopo la tua mossa l'avversario vince materiale.".to_owned()
    } else {
        "La mossa migliore era tranquilla: prima di scegliere, confronta due o tre mosse candidate e guarda le strade dell'avversario.".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use shakmaty::fen::Fen;
    use shakmaty::{CastlingMode, Chess};

    use super::lesson;

    fn position(fen: &str) -> Chess {
        fen.parse::<Fen>()
            .expect("FEN")
            .into_position(CastlingMode::Standard)
            .expect("posizione")
    }

    #[test]
    fn a_missed_check_points_to_the_first_step() {
        let start = position("3k4/r7/8/4N3/8/8/8/4K3 w - - 0 1");
        let text = lesson(&start, "e1d2", &[], &["e5c6".to_owned()]);
        assert!(text.contains("i tuoi scacchi"), "{text}");
    }

    #[test]
    fn a_piece_left_hanging_points_to_the_threats() {
        // Il Bianco sposta il re e lascia il cavallo in e5 alla torre nera.
        let start = position("3k4/8/8/r3N3/8/8/8/4K3 w - - 0 1");
        let text = lesson(&start, "e1d2", &["a5e5".to_owned()], &["e5c4".to_owned()]);
        assert!(text.contains("cosa minaccia"), "{text}");
    }
}
