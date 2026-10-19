//! La spiegazione di una posizione: fatti, piano e principi, in italiano.
//!
//! Il tutor non decide nulla da solo: mette in ordine ciò che hanno verificato
//! i rilevatori (crate `analysis`), il motore (se c'è la sua valutazione) e i
//! riconoscitori di questa crate, e ci aggiunge le schede della base di conoscenza.

use std::collections::HashSet;

use analysis::{Fact, FactCategory, GamePhase, analyze};
use chess_core::{Game, Side};
use engine::AnalysisUpdate;
use shakmaty::fen::Fen;
use shakmaty::{CastlingMode, Chess, Color};

use analysis::wording::side_name_capitalized;

use crate::card_view::card_view;
use crate::evaluation_words::describe_evaluation;
use crate::explanation_view::{CardView, TutorExplanation, TutorItem, TutorSection};
use crate::library::{card_for, opening_card};
use crate::recognition::{Recognition, recognize};
use crate::strategy::describe_strategy;
use crate::topic::Topic;
use crate::tutor_error::TutorError;

/// Quante minacce e tattiche mostrare al massimo.
const MAX_TACTICS: usize = 6;
/// Quanti principi mostrare al massimo.
const MAX_PRINCIPLES: usize = 5;

/// Rigioca la partita: dalla posizione di partenza, con le mosse indicate.
///
/// # Errors
///
/// [`TutorError`] se la posizione o una mossa non sono valide.
pub(crate) fn replay(start_fen: &str, moves: &[String]) -> Result<Game, TutorError> {
    let mut game = Game::from_fen(start_fen)
        .map_err(|error| TutorError::InvalidPosition(error.to_string()))?;
    for uci in moves {
        game.play_uci(uci)
            .map_err(|_| TutorError::IllegalMove(uci.clone()))?;
    }
    Ok(game)
}

/// La posizione di `shakmaty` per una FEN già controllata.
pub(crate) fn chess_position(fen: &str) -> Result<Chess, TutorError> {
    fen.parse::<Fen>()
        .ok()
        .and_then(|fen| fen.into_position(CastlingMode::Standard).ok())
        .ok_or_else(|| TutorError::InvalidPosition(fen.to_owned()))
}

/// Una frase semplice, riferita a `color`.
fn item(color: Color, text: String, card_id: Option<String>) -> TutorItem {
    TutorItem {
        fact: Fact::new(FactCategory::Activity, color, text),
        card_id,
    }
}

/// Una cosa riconosciuta diventa una frase (con la sua scheda, se c'è).
fn recognition_item(found: &Recognition, turn: Color) -> TutorItem {
    TutorItem {
        fact: Fact::new(
            FactCategory::Activity,
            found.owner.unwrap_or(turn),
            found.text.clone(),
        )
        .with_squares(found.squares.iter().copied()),
        card_id: card_for(found.topic, found.key).map(|card| card.id.clone()),
    }
}

/// La frase sulla fase di gioco.
fn phase_text(phase: GamePhase) -> &'static str {
    match phase {
        GamePhase::Opening => {
            "Siamo in apertura: contano lo sviluppo dei pezzi, il centro e la sicurezza del re"
        }
        GamePhase::Middlegame => {
            "Siamo nel mediogioco: è il momento dei piani, degli attacchi e delle tattiche"
        }
        GamePhase::Endgame => {
            "Siamo nel finale: il re diventa un pezzo attivo e i pedoni passati contano moltissimo"
        }
    }
}

/// La frase sul materiale.
fn material_text(advantage: i32) -> String {
    if advantage == 0 {
        return "Il materiale è pari".to_owned();
    }
    let side = side_name_capitalized(if advantage > 0 {
        Color::White
    } else {
        Color::Black
    });
    let points = advantage.abs();
    let unit = if points == 1 { "punto" } else { "punti" };
    format!(
        "{side} è avanti di {points} {unit} di materiale (pedone 1, pezzo minore 3, torre 5, donna 9)"
    )
}

/// Spiega una posizione.
///
/// - Riceve: la posizione di partenza, le mosse giocate fino alla posizione da
///   spiegare (in UCI) e, se c'è, la valutazione di Stockfish di quella posizione.
/// - Restituisce: la spiegazione, divisa in parti, con le schede che la approfondiscono.
///
/// # Errors
///
/// [`TutorError`] se la posizione o una mossa non sono valide.
pub fn explain(
    start_fen: &str,
    moves: &[String],
    evaluation: Option<&AnalysisUpdate>,
) -> Result<TutorExplanation, TutorError> {
    let game = replay(start_fen, moves)?;
    let view = game.view();
    let fen = view.position.fen.clone();
    let position = chess_position(&fen)?;
    let report = analyze(&fen).ok_or_else(|| TutorError::InvalidPosition(fen.clone()))?;
    let side_to_move = view.position.turn;
    let turn = match side_to_move {
        Side::White => Color::White,
        Side::Black => Color::Black,
    };
    let recognized = recognize(&position, report.phase);
    let opening = openings::identify(start_fen, moves);

    // La situazione: chi muove, fase, apertura, materiale, valutazione, tipo di finale.
    let to_move = match turn {
        Color::White => "Tocca al Bianco",
        Color::Black => "Tocca al Nero",
    };
    let mut situation = vec![
        item(turn, to_move.to_owned(), None),
        item(turn, phase_text(report.phase).to_owned(), None),
    ];
    if let Some(opening) = &opening {
        situation.push(item(
            turn,
            format!("Apertura: {} ({})", opening.name, opening.eco),
            opening_card(&opening.name).map(|card| card.id.clone()),
        ));
    }
    situation.push(item(turn, material_text(report.material.advantage), None));
    if let Some(update) = evaluation {
        situation.push(item(turn, describe_evaluation(update.evaluation), None));
        if let Some(best) = update.line.san.first() {
            situation.push(TutorItem {
                fact: Fact::new(
                    FactCategory::Activity,
                    turn,
                    "Secondo Stockfish la mossa migliore è".to_owned(),
                )
                .with_move(best.clone()),
                card_id: None,
            });
        }
    }
    situation.extend(
        recognized
            .iter()
            .filter(|found| found.topic == Topic::Endgame)
            .map(|found| recognition_item(found, turn)),
    );

    // Cosa succede adesso: minacce e tattiche dei rilevatori.
    let mut now: Vec<TutorItem> = report
        .facts
        .iter()
        .filter(|fact| matches!(fact.category, FactCategory::Threats | FactCategory::Tactics))
        .take(MAX_TACTICS)
        .map(|fact| TutorItem {
            fact: fact.clone(),
            card_id: None,
        })
        .collect();
    if now.is_empty() {
        now.push(item(
            turn,
            "Nessuna minaccia immediata: è il momento di pensare a un piano".to_owned(),
            None,
        ));
    }

    // Il piano: strutture pedonali e temi di mediogioco.
    let plan: Vec<TutorItem> = recognized
        .iter()
        .filter(|found| matches!(found.topic, Topic::Structure | Topic::Middlegame))
        .map(|found| recognition_item(found, turn))
        .collect();

    // Da ricordare: i principi in gioco.
    let principles: Vec<TutorItem> = recognized
        .iter()
        .filter(|found| found.topic == Topic::Principle)
        .take(MAX_PRINCIPLES)
        .map(|found| recognition_item(found, turn))
        .collect();

    let sections = [
        ("La situazione", situation),
        ("Cosa succede adesso", now),
        ("Il piano", plan),
        ("Da ricordare", principles),
    ]
    .into_iter()
    .filter(|(_, items)| !items.is_empty())
    .map(|(title, items)| TutorSection {
        title: title.to_owned(),
        items,
    })
    .collect();

    Ok(TutorExplanation {
        fen,
        position: view.position,
        turn: side_to_move,
        sections,
        cards: cards(opening.as_ref(), &recognized),
        strategy: describe_strategy(&position, report.phase, &recognized),
    })
}

/// Le schede che riguardano la posizione, senza doppioni, nell'ordine degli argomenti.
fn cards(opening: Option<&openings::Opening>, recognized: &[Recognition]) -> Vec<CardView> {
    let mut seen = HashSet::new();
    let mut result = Vec::new();
    if let Some(opening) = opening
        && let Some(card) = opening_card(&opening.name)
    {
        seen.insert(card.id.clone());
        result.push(card_view(
            card,
            format!("Apertura riconosciuta: {}", opening.name),
            None,
        ));
    }
    let mut ordered: Vec<&Recognition> = recognized.iter().collect();
    ordered.sort_by_key(|found| found.topic);
    for found in ordered {
        if let Some(card) = card_for(found.topic, found.key)
            && seen.insert(card.id.clone())
        {
            result.push(card_view(card, found.text.clone(), found.owner));
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use chess_core::LineView;
    use engine::{AnalysisUpdate, Evaluation};

    use super::explain;

    const START: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    fn moves(list: &[&str]) -> Vec<String> {
        list.iter().map(|uci| (*uci).to_owned()).collect()
    }

    fn titles(sections: &[crate::TutorSection]) -> Vec<&str> {
        sections
            .iter()
            .map(|section| section.title.as_str())
            .collect()
    }

    #[test]
    fn explains_the_opening_with_its_card() {
        let explanation = explain(
            START,
            &moves(&["e2e4", "e7e5", "g1f3", "b8c6", "f1c4"]),
            None,
        )
        .expect("posizione valida");

        assert_eq!(explanation.cards[0].id, "openings/italian-game");
        assert_eq!(explanation.cards[0].plans[0].label, "Il Bianco");
        let situation = &explanation.sections[0];
        assert_eq!(situation.items[0].fact.text, "Tocca al Nero");
        assert!(
            situation
                .items
                .iter()
                .any(|item| item.fact.text.starts_with("Apertura: Italian Game"))
        );
    }

    #[test]
    fn adds_the_engine_judgement_when_available() {
        let update = AnalysisUpdate {
            depth: 18,
            evaluation: Evaluation::Centipawns(35),
            line: LineView {
                fen: START.to_owned(),
                turn: chess_core::Side::White,
                move_number: 1,
                san: vec!["e4".to_owned()],
                first_move: None,
            },
        };

        let explanation = explain(START, &[], Some(&update)).expect("posizione valida");

        let texts: Vec<&str> = explanation.sections[0]
            .items
            .iter()
            .map(|item| item.fact.text.as_str())
            .collect();
        assert!(texts.contains(&"Il Bianco sta leggermente meglio (+0.3)"));
        assert!(texts.contains(&"Secondo Stockfish la mossa migliore è"));
    }

    #[test]
    fn links_a_pawn_structure_to_its_plans() {
        let explanation = explain(
            "r1bq1rk1/pp2bppp/2n1pn2/8/3P4/2NB1N2/PP3PPP/R1BQ1RK1 w - - 0 9",
            &[],
            None,
        )
        .expect("posizione valida");

        assert!(titles(&explanation.sections).contains(&"Il piano"));
        let card = explanation
            .cards
            .iter()
            .find(|card| card.id == "structures/isolated-queen-pawn")
            .expect("scheda dell'isolano");
        assert_eq!(card.plans[0].label, "Chi ha l'isolano (il Bianco)");
        assert_eq!(card.plans[1].label, "Chi gioca contro l'isolano (il Nero)");
    }

    #[test]
    fn rejects_an_illegal_move() {
        assert!(explain(START, &moves(&["e2e5"]), None).is_err());
    }
}
