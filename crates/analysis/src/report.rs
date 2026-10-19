//! Il rapporto completo su una posizione: tutti i rilevatori insieme.

use chess_core::Side;
use shakmaty::fen::Fen;
use shakmaty::{CastlingMode, Chess, Position};

use crate::activity::activity_facts;
use crate::fact::Fact;
use crate::forks::fork_facts;
use crate::game_phase::{GamePhase, game_phase};
use crate::hanging::hanging_facts;
use crate::king_safety::king_facts;
use crate::lines::line_facts;
use crate::mate_threats::mate_facts;
use crate::material::{MaterialBalance, material, material_facts};
use crate::overload::overload_facts;
use crate::pawn_structure::pawn_facts;

/// Tutto ciò che i rilevatori hanno trovato in una posizione.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct PositionReport {
    /// La posizione analizzata, in notazione FEN (per riconoscere un rapporto vecchio).
    pub fen: String,
    /// Chi deve muovere.
    pub turn: Side,
    /// La fase di gioco.
    pub phase: GamePhase,
    /// Il materiale di ciascuno.
    pub material: MaterialBalance,
    /// I fatti trovati, ordinati per gruppo (minacce, tattiche, pedoni, re, pezzi, materiale).
    pub facts: Vec<Fact>,
}

/// Analizza la posizione descritta dalla FEN con tutti i rilevatori.
///
/// - Riceve: la posizione in notazione FEN.
/// - Restituisce: il rapporto; `None` se la FEN non descrive una posizione valida.
///
/// È veloce (qualche millesimo di secondo): si può chiamare a ogni mossa.
#[must_use]
pub fn analyze(fen: &str) -> Option<PositionReport> {
    let position: Chess = fen
        .trim()
        .parse::<Fen>()
        .ok()?
        .into_position(CastlingMode::Standard)
        .ok()?;
    Some(analyze_position(&position, fen.trim()))
}

/// Analizza una posizione già pronta.
fn analyze_position(position: &Chess, fen: &str) -> PositionReport {
    let board = position.board();
    let mut facts = mate_facts(position);
    facts.extend(hanging_facts(board));
    facts.extend(fork_facts(position));
    facts.extend(line_facts(board));
    facts.extend(overload_facts(board));
    facts.extend(pawn_facts(board));
    facts.extend(king_facts(position));
    facts.extend(activity_facts(position));
    facts.extend(material_facts(board));
    // L'ordine dei gruppi è quello della scheda; dentro un gruppo resta
    // l'ordine dei rilevatori (prima i più urgenti).
    facts.sort_by_key(|fact| fact.category);
    PositionReport {
        fen: fen.to_owned(),
        turn: Side::from(position.turn()),
        phase: game_phase(position),
        material: material(board),
        facts,
    }
}

#[cfg(test)]
mod tests {
    use super::analyze;
    use crate::fact::FactCategory;
    use crate::game_phase::GamePhase;

    #[test]
    fn the_starting_position_has_nothing_alarming() {
        let report = analyze("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
            .expect("posizione valida");

        assert_eq!(report.phase, GamePhase::Opening);
        assert!(
            report.facts.iter().all(|fact| !matches!(
                fact.category,
                FactCategory::Threats | FactCategory::Tactics
            ))
        );
    }

    #[test]
    fn puts_the_threats_first() {
        // Matto del barbiere in arrivo e altro ancora: le minacce vengono prima.
        let report = analyze("rnbqkbnr/pppp1ppp/8/4p3/6P1/5P2/PPPPP2P/RNBQKBNR w KQkq - 0 3")
            .expect("posizione valida");

        assert_eq!(report.facts[0].category, FactCategory::Threats);
        let categories: Vec<FactCategory> = report.facts.iter().map(|fact| fact.category).collect();
        let mut sorted = categories.clone();
        sorted.sort();
        assert_eq!(categories, sorted);
    }

    #[test]
    fn rejects_an_invalid_position() {
        assert_eq!(analyze("non è una FEN"), None);
    }
}
