//! Il metodo di pensiero «Prima di muovere», riempito con la posizione.
//!
//! Quattro passi, sempre nello stesso ordine (scheda `tactics/thinking-method`):
//! 1. **I tuoi scacchi**: tutte le mosse che danno scacco;
//! 2. **Le tue catture**: tutte le mosse che catturano;
//! 3. **Cosa minaccia l'avversario**: le sue minacce e tattiche, dai rilevatori;
//! 4. **Il controllo finale**: prima di toccare il pezzo, guardare cosa si lascia.
//!
//! Le mosse non si scrivono nelle frasi: stanno a parte (in notazione
//! internazionale) e l'interfaccia le mostra nella notazione scelta.

use analysis::{Fact, FactCategory};
use chess_core::Side;
use shakmaty::san::SanPlus;
use shakmaty::{Chess, Position};

use crate::card_view::card_view;
use crate::explanation_view::{ThinkingStep, ThinkingView};
use crate::library::card_for;
use crate::motifs::THINKING_METHOD;
use crate::topic::Topic;

/// Quante mosse elencare al massimo per passo.
const MAX_MOVES: usize = 8;
/// Quante minacce dell'avversario elencare al massimo.
const MAX_THREATS: usize = 2;

/// Le mosse (in SAN) che rispettano `keep`, al massimo [`MAX_MOVES`].
fn moves_where(position: &Chess, keep: impl Fn(&shakmaty::Move, &Chess) -> bool) -> Vec<String> {
    position
        .legal_moves()
        .into_iter()
        .filter_map(|chess_move| {
            let mut after = position.clone();
            let san = SanPlus::from_move_and_play_unchecked(&mut after, chess_move).to_string();
            keep(&chess_move, &after).then_some(san)
        })
        .take(MAX_MOVES)
        .collect()
}

/// Un passo con il numero di mosse trovate.
fn counted(title: &str, moves: Vec<String>, none: &str, some: &str) -> ThinkingStep {
    let text = match moves.len() {
        0 => none.to_owned(),
        1 => format!("Una mossa: {some}"),
        count => format!("{count} mosse: {some}"),
    };
    ThinkingStep {
        title: title.to_owned(),
        text,
        moves,
    }
}

/// Il metodo «Prima di muovere» per chi deve muovere in `position`.
///
/// - `facts`: i fatti dei rilevatori sulla posizione (servono le minacce dell'avversario).
#[must_use]
pub fn thinking(position: &Chess, facts: &[Fact]) -> ThinkingView {
    let turn = position.turn();
    let opponent = Side::from(turn.other());
    let checks = moves_where(position, |_, after| after.is_check());
    let captures = moves_where(position, |chess_move, _| chess_move.is_capture());
    let threats: Vec<&Fact> = facts
        .iter()
        .filter(|fact| fact.side == opponent)
        .filter(|fact| matches!(fact.category, FactCategory::Threats | FactCategory::Tactics))
        .take(MAX_THREATS)
        .collect();
    let threat_step = ThinkingStep {
        title: "Cosa minaccia l'avversario".to_owned(),
        text: if threats.is_empty() {
            "Nessuna minaccia immediata. Con «Guarda avanti» vedi anche le strade che potrebbe prendere più avanti.".to_owned()
        } else {
            threats
                .iter()
                .map(|fact| fact.text.clone())
                .collect::<Vec<_>>()
                .join(". ")
        },
        moves: threats.iter().flat_map(|fact| fact.moves.clone()).collect(),
    };
    let steps = vec![
        counted(
            "I tuoi scacchi",
            checks,
            "Nessuno scacco possibile.",
            "guardale tutte, anche quelle che sembrano impossibili.",
        ),
        counted(
            "Le tue catture",
            captures,
            "Nessuna cattura possibile.",
            "per ognuna chiediti cosa risponde l'avversario.",
        ),
        threat_step,
        ThinkingStep {
            title: "Il controllo finale".to_owned(),
            text: "Scelta la mossa, guarda la casa dove va il pezzo e i pezzi che lasci: chi li attacca, chi li difende?".to_owned(),
            moves: Vec::new(),
        },
    ];
    ThinkingView {
        steps,
        card: card_for(Topic::Tactic, THINKING_METHOD).map(|card| {
            card_view(
                card,
                "Il metodo da usare prima di ogni mossa".to_owned(),
                Some(turn),
            )
        }),
    }
}

#[cfg(test)]
mod tests {
    use shakmaty::fen::Fen;
    use shakmaty::{CastlingMode, Chess};

    use super::thinking;

    #[test]
    fn lists_checks_and_captures_in_order() {
        // La donna bianca in h5 può prendere f7 con scacco.
        let position: Chess = "rnbqkbnr/pppp1ppp/8/4p2Q/4P3/8/PPPP1PPP/RNB1KBNR w KQkq - 0 1"
            .parse::<Fen>()
            .expect("FEN")
            .into_position(CastlingMode::Standard)
            .expect("posizione");
        let view = thinking(&position, &[]);

        assert_eq!(view.steps.len(), 4);
        assert_eq!(view.steps[0].title, "I tuoi scacchi");
        assert!(view.steps[0].moves.contains(&"Qxf7+".to_owned()));
        assert!(view.steps[1].moves.contains(&"Qxe5+".to_owned()));
        assert!(view.card.is_some(), "la scheda del metodo");
    }
}
