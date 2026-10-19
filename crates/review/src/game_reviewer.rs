//! La revisione di una partita: giudizio delle mosse, riepiloghi, momenti critici.

use std::collections::HashMap;

use analysis::{Fact, GamePhase, analyze};
use chess_core::{LegalMove, Side, describe_line};
use tutor::what_if::{CONCEDED_TITLE, CREATED_TITLE};
use tutor::{MotifView, fact_motif, motif_view};

use crate::move_quality::MoveQuality;
use crate::replay::{Replayed, replay};
use crate::review_error::ReviewError;
use crate::review_input::ReviewInput;
use crate::review_view::{CriticalMoment, GameReview, MoveReview, PlayerSummary};
use crate::win_chance::{chance_for, move_accuracy, white_win_chance};

/// Il motivo tattico di un momento critico: prima quello che otteneva la mossa
/// migliore (l'occasione persa), poi quello che la mossa giocata concedeva.
fn moment_motif(best_gains: &[Fact], conceded: &[Fact]) -> Option<MotifView> {
    best_gains
        .iter()
        .chain(conceded)
        .find_map(|fact| fact_motif(&fact.text))
        .map(motif_view)
}

/// Quanti momenti critici mostrare al massimo.
const MAX_CRITICAL: usize = 5;

/// Una mossa UCI con le sue case (es. `"e2e4"` → da e2 a e4).
pub(crate) fn legal_move(uci: &str) -> LegalMove {
    LegalMove {
        from: uci.get(0..2).unwrap_or_default().to_owned(),
        to: uci.get(2..4).unwrap_or_default().to_owned(),
        uci: uci.to_owned(),
    }
}

/// La probabilità di vittoria del Bianco in ogni posizione, se si conosce.
///
/// Nell'ultima posizione, se la partita è finita sulla scacchiera (il motore
/// non ha niente da valutare), vale il risultato: 100 o 0 per il matto, 50 per la patta.
fn white_chances(input: &ReviewInput, replayed: &Replayed) -> Vec<Option<f64>> {
    let last = replayed.fens.len() - 1;
    (0..=last)
        .map(|index| {
            let engine = input
                .evaluations
                .get(index)
                .and_then(|position| position.evaluation)
                .map(white_win_chance);
            match (&replayed.result, engine) {
                (Some(result), None) if index == last => Some(match result.winner {
                    Some(Side::White) => 100.0,
                    Some(Side::Black) => 0.0,
                    None => 50.0,
                }),
                _ => engine,
            }
        })
        .collect()
}

/// La mossa migliore in SAN, nella posizione dopo `history`.
fn best_san(start_fen: &str, history: &[String], best: &str) -> Option<String> {
    describe_line(start_fen, history, &[best.to_owned()])
        .ok()
        .and_then(|line| line.san.into_iter().next())
}

/// Le frasi di una parte dello scenario "e se…" di una mossa.
pub(crate) fn scenario_facts(
    input: &ReviewInput,
    index: usize,
    uci: &str,
    title: &str,
) -> Vec<Fact> {
    tutor::what_if(&input.start_fen, &input.moves[..index], uci, None, None)
        .map(|scenario| {
            scenario
                .sections
                .into_iter()
                .filter(|section| section.title == title)
                .flat_map(|section| section.items.into_iter().map(|item| item.fact))
                .collect()
        })
        .unwrap_or_default()
}

/// Il riepilogo di un giocatore.
fn summary(moves: &[MoveReview], side: Side) -> PlayerSummary {
    let own: Vec<&MoveReview> = moves.iter().filter(|review| review.side == side).collect();
    let losses: Vec<f64> = own.iter().filter_map(|review| review.chance_lost).collect();
    let accuracy = if losses.is_empty() {
        None
    } else {
        let total: f64 = losses.iter().map(|lost| move_accuracy(*lost)).sum();
        let count = f64::from(u32::try_from(losses.len()).unwrap_or(u32::MAX));
        // La precisione sta tra 0 e 100: la conversione non perde nulla.
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        Some((total / count).round() as u32)
    };
    let count = |quality: MoveQuality| {
        u32::try_from(
            own.iter()
                .filter(|review| review.quality == Some(quality))
                .count(),
        )
        .unwrap_or(u32::MAX)
    };
    let mut by_phase: HashMap<GamePhase, f64> = HashMap::new();
    for review in own
        .iter()
        .filter(|review| review.quality.is_some_and(MoveQuality::is_error))
    {
        *by_phase.entry(review.phase).or_default() += review.chance_lost.unwrap_or_default();
    }
    let worst_phase = by_phase
        .into_iter()
        .max_by(|a, b| a.1.total_cmp(&b.1))
        .map(|(phase, _)| phase);
    PlayerSummary {
        accuracy,
        inaccuracies: count(MoveQuality::Inaccuracy),
        mistakes: count(MoveQuality::Mistake),
        blunders: count(MoveQuality::Blunder),
        worst_phase,
    }
}

/// Il nome del giocatore preceduto da "del": "del Bianco", "del Nero".
fn of_side(side: Side) -> &'static str {
    match side {
        Side::White => "del Bianco",
        Side::Black => "del Nero",
    }
}

/// I momenti critici: gli errori che hanno perso più probabilità di vittoria.
fn critical_moments(
    input: &ReviewInput,
    moves: &[MoveReview],
    chances: &[Option<f64>],
) -> Vec<CriticalMoment> {
    let mut errors: Vec<(usize, &MoveReview)> = moves
        .iter()
        .enumerate()
        .filter(|(_, review)| review.quality.is_some_and(MoveQuality::is_error))
        .collect();
    errors.sort_by(|a, b| {
        b.1.chance_lost
            .unwrap_or_default()
            .total_cmp(&a.1.chance_lost.unwrap_or_default())
    });
    errors.truncate(MAX_CRITICAL);
    errors.sort_by_key(|(index, _)| *index);

    errors
        .into_iter()
        .filter_map(|(index, review)| {
            let quality = review.quality?;
            let before = chance_for(chances[index]?, review.side);
            let after = chance_for(chances[index + 1]?, review.side);
            let conceded = scenario_facts(input, index, &review.played.uci, CONCEDED_TITLE);
            let best_gains = review
                .best
                .as_ref()
                .map(|best| scenario_facts(input, index, &best.uci, CREATED_TITLE))
                .unwrap_or_default();
            Some(CriticalMoment {
                ply: review.ply,
                quality,
                text: format!(
                    "{}: la probabilità di vincere {} scende dal {before:.0}% al {after:.0}%",
                    quality.name(),
                    of_side(review.side)
                ),
                played_san: review.san.clone(),
                best_san: review.best_san.clone(),
                motif: moment_motif(&best_gains, &conceded),
                conceded,
                best_gains,
            })
        })
        .collect()
}

/// Analizza una partita: giudica ogni mossa, riassume i giocatori, trova i momenti critici.
///
/// - Riceve: la partita e il giudizio del motore su ogni posizione.
/// - Restituisce: la revisione. Le mosse senza valutazione restano senza giudizio.
///
/// # Errors
///
/// [`ReviewError::InvalidGame`] se la partita non si può rigiocare.
pub fn review_game(input: &ReviewInput) -> Result<GameReview, ReviewError> {
    let replayed = replay(&input.start_fen, &input.moves)?;
    let chances = white_chances(input, &replayed);

    let moves: Vec<MoveReview> = replayed
        .records
        .iter()
        .enumerate()
        .map(|(index, record)| {
            let chance_lost = chances[index]
                .zip(chances[index + 1])
                .map(|(before, after)| {
                    (chance_for(before, record.side) - chance_for(after, record.side)).max(0.0)
                });
            let best_uci = input
                .evaluations
                .get(index)
                .and_then(|position| position.best_move.clone());
            let is_best = best_uci.as_deref() == Some(record.uci.as_str());
            MoveReview {
                ply: u32::try_from(record.ply).unwrap_or(u32::MAX),
                move_number: record.move_number,
                side: record.side,
                san: record.san.clone(),
                played: legal_move(&record.uci),
                quality: chance_lost.map(|lost| MoveQuality::judge(lost, is_best)),
                chance_lost,
                evaluation_before: input
                    .evaluations
                    .get(index)
                    .and_then(|position| position.evaluation),
                evaluation_after: input
                    .evaluations
                    .get(index + 1)
                    .and_then(|position| position.evaluation),
                best_san: best_uci
                    .as_deref()
                    .and_then(|best| best_san(&input.start_fen, &input.moves[..index], best)),
                best: best_uci.as_deref().map(legal_move),
                phase: analyze(&replayed.fens[index])
                    .map_or(GamePhase::Opening, |report| report.phase),
            }
        })
        .collect();

    // Nel grafico le posizioni senza valutazione prendono il valore di quella precedente.
    let mut previous = 50.0;
    let white_chances = chances
        .iter()
        .map(|chance| {
            previous = chance.unwrap_or(previous);
            previous
        })
        .collect();

    Ok(GameReview {
        start_fen: input.start_fen.clone(),
        white: summary(&moves, Side::White),
        black: summary(&moves, Side::Black),
        critical: critical_moments(input, &moves, &chances),
        positions: replayed.positions,
        moves,
        white_chances,
    })
}

#[cfg(test)]
mod tests {
    use super::review_game;
    use crate::move_quality::MoveQuality;
    use crate::review_input::PositionEval;
    use crate::test_game::scholars_mate;

    #[test]
    fn judges_the_moves_and_finds_the_blunder() {
        let review = review_game(&scholars_mate()).expect("partita valida");

        assert_eq!(review.moves[0].quality, Some(MoveQuality::Best));
        assert_eq!(review.moves[5].quality, Some(MoveQuality::Blunder));
        assert_eq!(review.moves[5].best_san.as_deref(), Some("Qe7"));
        assert_eq!(review.black.blunders, 1);
        assert_eq!(review.white.blunders, 0);
        // Dopo il matto il grafico arriva al 100% per il Bianco.
        assert!((review.white_chances[7] - 100.0).abs() < 1e-9);
    }

    #[test]
    fn explains_the_critical_moment() {
        let review = review_game(&scholars_mate()).expect("partita valida");

        let moment = &review.critical[0];
        assert_eq!(moment.ply, 6);
        assert_eq!(moment.played_san, "Nf6");
        assert!(
            moment
                .text
                .starts_with("Errore grave: la probabilità di vincere del Nero")
        );
    }

    #[test]
    fn moves_without_evaluation_have_no_judgement() {
        let mut input = scholars_mate();
        input.evaluations = vec![PositionEval::default(); 8];

        let review = review_game(&input).expect("partita valida");

        assert_eq!(review.moves[0].quality, None);
        assert_eq!(review.white.accuracy, None);
        assert_eq!(review.critical, []);
    }
}
