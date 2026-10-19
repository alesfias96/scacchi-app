//! I dati di ogni mossa da salvare nel database, per le statistiche (Fase 9).

use analysis::{FactCategory, GamePhase, analyze};
use engine::Evaluation;
use shakmaty::Color;
use tutor::evaluation_words::score_for;
use tutor::what_if::CONCEDED_TITLE;

use crate::game_reviewer::scenario_facts;
use crate::replay::replay;
use crate::review_error::ReviewError;
use crate::review_input::ReviewInput;
use crate::review_view::GameReview;

/// Il massimo di centipedoni persi che si registra per una mossa (oltre, la
/// partita è comunque persa: i matti non diventano numeri enormi).
const MAX_CENTIPAWN_LOSS: i32 = 1000;

/// I dati dell'analisi di una mossa, come si salvano.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoveAnalysis {
    /// La mezza mossa.
    pub ply: u32,
    /// La valutazione prima della mossa (per il Bianco).
    pub evaluation_before: Option<Evaluation>,
    /// La valutazione dopo la mossa (per il Bianco).
    pub evaluation_after: Option<Evaluation>,
    /// La mossa migliore (UCI).
    pub best_move: Option<String>,
    /// I centipedoni persi rispetto alla mossa migliore.
    pub centipawn_loss: Option<i32>,
    /// La fase di gioco prima della mossa.
    pub phase: GamePhase,
    /// Le tattiche e le minacce a favore di chi muove, prima della mossa.
    pub tactical_themes: Vec<String>,
    /// Cosa la mossa ha concesso all'avversario, se è stata un errore.
    pub missed_threats: Vec<String>,
}

/// Il testo fisso della fase di gioco nel database (non si cambia mai).
#[must_use]
pub fn phase_code(phase: GamePhase) -> &'static str {
    match phase {
        GamePhase::Opening => "opening",
        GamePhase::Middlegame => "middlegame",
        GamePhase::Endgame => "endgame",
    }
}

/// I dati da salvare per ogni mossa.
///
/// # Errors
///
/// [`ReviewError::InvalidGame`] se la partita non si può rigiocare.
pub fn move_analysis(
    input: &ReviewInput,
    review: &GameReview,
) -> Result<Vec<MoveAnalysis>, ReviewError> {
    let replayed = replay(&input.start_fen, &input.moves)?;
    Ok(review
        .moves
        .iter()
        .enumerate()
        .map(|(index, reviewed)| {
            let mover = match reviewed.side {
                chess_core::Side::White => Color::White,
                chess_core::Side::Black => Color::Black,
            };
            let centipawn_loss = reviewed
                .evaluation_before
                .zip(reviewed.evaluation_after)
                .map(|(before, after)| {
                    (score_for(before, mover) - score_for(after, mover))
                        .clamp(0, MAX_CENTIPAWN_LOSS)
                });
            let tactical_themes = analyze(&replayed.fens[index])
                .map(|report| {
                    report
                        .facts
                        .into_iter()
                        .filter(|fact| {
                            fact.side == reviewed.side
                                && matches!(
                                    fact.category,
                                    FactCategory::Threats | FactCategory::Tactics
                                )
                        })
                        .map(|fact| fact.text)
                        .collect()
                })
                .unwrap_or_default();
            let missed_threats = if reviewed.quality.is_some_and(crate::MoveQuality::is_error) {
                scenario_facts(input, index, &reviewed.played.uci, CONCEDED_TITLE)
                    .into_iter()
                    .map(|fact| fact.text)
                    .collect()
            } else {
                Vec::new()
            };
            MoveAnalysis {
                ply: reviewed.ply,
                evaluation_before: reviewed.evaluation_before,
                evaluation_after: reviewed.evaluation_after,
                best_move: reviewed.best.as_ref().map(|best| best.uci.clone()),
                centipawn_loss,
                phase: reviewed.phase,
                tactical_themes,
                missed_threats,
            }
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use analysis::GamePhase;

    use super::move_analysis;
    use crate::game_reviewer::review_game;
    use crate::test_game::scholars_mate;

    #[test]
    fn records_losses_phases_and_conceded_threats() {
        let input = scholars_mate();
        let review = review_game(&input).expect("partita valida");

        let records = move_analysis(&input, &review).expect("partita valida");

        assert_eq!(records.len(), 7);
        assert_eq!(records[0].phase, GamePhase::Opening);
        assert_eq!(records[5].best_move.as_deref(), Some("d8e7"));
        assert_eq!(records[5].centipawn_loss, Some(1000));
        assert_eq!(records[0].missed_threats, Vec::<String>::new());
    }
}
