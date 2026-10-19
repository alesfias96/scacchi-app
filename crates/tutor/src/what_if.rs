//! Gli scenari "e se…": cosa cambia se si gioca una certa mossa.
//!
//! Si confrontano i fatti dei rilevatori prima e dopo la mossa (minacce
//! nuove, minacce risolte, tattiche create) e, se c'è, la valutazione di
//! Stockfish prima e dopo.

use std::collections::HashSet;

use analysis::{Fact, FactCategory, PositionReport, analyze};
use chess_core::{GameEndReason, LegalMove, LineView, PositionView, Side};
use engine::{AnalysisUpdate, Evaluation};
use shakmaty::Color;

use crate::evaluation_words::move_verdict;
use crate::explanation::replay;
use crate::explanation_view::{TutorItem, TutorSection};
use crate::tutor_error::TutorError;

/// Il titolo della parte con ciò che la mossa concede all'avversario.
pub const CONCEDED_TITLE: &str = "Cosa concedi all'avversario";
/// Il titolo della parte con ciò che la mossa crea.
pub const CREATED_TITLE: &str = "Cosa crei";
/// Il titolo della parte con le minacce che la mossa risolve.
pub const SOLVED_TITLE: &str = "Minacce che risolvi";

/// Il risultato di una mossa provata.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct WhatIfView {
    /// La mossa provata, in notazione SAN (es. `"Nxe5"`).
    pub san: String,
    /// Chi l'ha giocata.
    pub mover: Side,
    /// La posizione dopo la mossa (con le mosse legali, per continuare a provare).
    pub position: PositionView,
    /// La mossa provata, da evidenziare.
    pub last_move: Option<LegalMove>,
    /// Il giudizio sulla mossa, se c'è la valutazione di Stockfish.
    pub verdict: Option<String>,
    /// La valutazione prima della mossa (con il gioco migliore).
    pub evaluation_before: Option<Evaluation>,
    /// La valutazione dopo la mossa.
    pub evaluation_after: Option<Evaluation>,
    /// La risposta migliore dell'avversario, secondo Stockfish.
    pub best_reply: Option<LineView>,
    /// Cosa cambia: cosa si concede, cosa si crea, cosa si risolve.
    pub sections: Vec<TutorSection>,
}

/// `true` per i fatti che contano in uno scenario: minacce e tattiche.
fn is_tactical(fact: &Fact) -> bool {
    matches!(fact.category, FactCategory::Threats | FactCategory::Tactics)
}

/// I fatti tattici di `side` presenti in `after` ma non in `before`.
fn new_facts(before: &PositionReport, after: &PositionReport, side: Side) -> Vec<TutorItem> {
    let known: HashSet<&str> = before
        .facts
        .iter()
        .filter(|fact| fact.side == side)
        .map(|fact| fact.text.as_str())
        .collect();
    after
        .facts
        .iter()
        .filter(|fact| {
            fact.side == side && is_tactical(fact) && !known.contains(fact.text.as_str())
        })
        .map(|fact| TutorItem {
            fact: fact.clone(),
            card_id: None,
        })
        .collect()
}

/// Prova una mossa e dice cosa cambia.
///
/// - Riceve: la posizione di partenza, le mosse fino alla posizione in cui si
///   prova, la mossa da provare (UCI) e, se ci sono, le valutazioni di
///   Stockfish prima e dopo la mossa.
/// - Restituisce: lo scenario, con la nuova posizione per continuare a provare.
///
/// # Errors
///
/// [`TutorError`] se la posizione o una mossa non sono valide.
pub fn what_if(
    start_fen: &str,
    history: &[String],
    uci: &str,
    before: Option<&AnalysisUpdate>,
    after: Option<&AnalysisUpdate>,
) -> Result<WhatIfView, TutorError> {
    let mut game = replay(start_fen, history)?;
    let report_before =
        analyze(&game.fen()).ok_or_else(|| TutorError::InvalidPosition(game.fen()))?;
    let mover = game.turn();
    game.play_uci(uci)
        .map_err(|_| TutorError::IllegalMove(uci.to_owned()))?;
    let view = game.view();
    let report_after = analyze(&view.position.fen)
        .ok_or_else(|| TutorError::InvalidPosition(view.position.fen.clone()))?;
    let opponent = mover.opponent();

    let mut sections = Vec::new();
    if let Some(result) = &view.result {
        let text = match result.reason {
            GameEndReason::Checkmate => "Scacco matto: la partita finisce qui!",
            GameEndReason::Stalemate => "Stallo: la partita finisce patta",
            _ => "La partita finisce patta",
        };
        let color = if mover == Side::White {
            Color::White
        } else {
            Color::Black
        };
        sections.push(TutorSection {
            title: "Fine della partita".to_owned(),
            items: vec![TutorItem {
                fact: Fact::new(FactCategory::Threats, color, text.to_owned()),
                card_id: None,
            }],
        });
    }
    let solved: Vec<TutorItem> = new_facts(&report_after, &report_before, opponent);
    for (title, items) in [
        (
            CONCEDED_TITLE,
            new_facts(&report_before, &report_after, opponent),
        ),
        (
            CREATED_TITLE,
            new_facts(&report_before, &report_after, mover),
        ),
        (SOLVED_TITLE, solved),
    ] {
        if !items.is_empty() {
            sections.push(TutorSection {
                title: title.to_owned(),
                items,
            });
        }
    }

    let mover_color = if mover == Side::White {
        Color::White
    } else {
        Color::Black
    };
    let verdict = before
        .zip(after)
        .map(|(before, after)| move_verdict(before.evaluation, after.evaluation, mover_color));
    Ok(WhatIfView {
        san: view
            .moves
            .last()
            .map(|record| record.san.clone())
            .unwrap_or_default(),
        mover,
        last_move: view.last_move.clone(),
        position: view.position,
        verdict,
        evaluation_before: before.map(|update| update.evaluation),
        evaluation_after: after.map(|update| update.evaluation),
        best_reply: after
            .filter(|update| !update.line.san.is_empty())
            .map(|update| update.line.clone()),
        sections,
    })
}

#[cfg(test)]
mod tests {
    use super::what_if;

    const START: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    fn moves(list: &[&str]) -> Vec<String> {
        list.iter().map(|uci| (*uci).to_owned()).collect()
    }

    #[test]
    fn plays_the_move_and_gives_the_new_position() {
        // 1.e4 e5 2.Nf3 Nc6 3.Bc4, poi si prova 3...Nh6: senza Stockfish non c'è giudizio.
        let scenario = what_if(
            START,
            &moves(&["e2e4", "e7e5", "g1f3", "b8c6", "f1c4"]),
            "g8h6",
            None,
            None,
        )
        .expect("mossa legale");

        assert_eq!(scenario.san, "Nh6");
        assert_eq!(scenario.verdict, None);
        assert!(
            scenario
                .position
                .legal_moves
                .iter()
                .any(|legal| legal.uci == "e1g1")
        );
    }

    #[test]
    fn announces_checkmate() {
        let scenario = what_if(START, &moves(&["f2f3", "e7e5", "g2g4"]), "d8h4", None, None)
            .expect("mossa legale");

        assert_eq!(scenario.san, "Qh4#");
        assert_eq!(scenario.sections[0].title, "Fine della partita");
    }

    #[test]
    fn rejects_an_illegal_move() {
        assert!(what_if(START, &[], "e2e5", None, None).is_err());
    }
}
