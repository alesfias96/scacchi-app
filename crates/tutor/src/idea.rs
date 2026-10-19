//! L'idea di una variante: si guarda dove porta e si spiega il perché.
//!
//! Il tutor non "indovina" le intenzioni: rigioca la variante di Stockfish
//! (vedi `line_walk`), ci cerca gli eventi strategici e tattici (vedi
//! `idea_events`), sceglie i più importanti e li racconta. La frase principale
//! dice l'idea e **quando** si vede; i momenti chiave la mostrano passo per passo.

use analysis::FactArrow;
use analysis::wording::side_name;
use chess_core::{LegalMove, Side};
use engine::Evaluation;
use shakmaty::fen::Fen;
use shakmaty::{Chess, Color, EnPassantMode, Position};

use crate::card_view::card_view;
use crate::evaluation_words::{evaluation_number, score_for};
use crate::explanation::{chess_position, replay};
use crate::explanation_view::CardView;
use crate::idea_events::{
    IdeaEvent, IdeaKind, material_event, open_file_events, pawn_break_events, quiet_plan_event,
    route_events, structure_events, tactic_events, weakness_events,
};
use crate::idea_view::{IdeaFrame, IdeaMoment, IdeaView, MoveIdea, PositionIdeas};
use crate::library::all_cards;
use crate::line_walk::{LineStep, walk};
use crate::motifs::{Motif, motif_events, motif_view};
use crate::move_lesson::lesson;
use crate::recognition::recognize;
use crate::strategy::describe_strategy;
use crate::strategy_plans::confirming_kinds;
use crate::tutor_error::TutorError;

/// Quante mezze mosse della variante si guardano al massimo.
pub const LOOK_AHEAD_PLIES: usize = 20;
/// Entro quante mezze mosse si cercano i motivi tattici.
const TACTIC_PLIES: usize = 10;
/// Quanti momenti chiave mostrare al massimo.
const MAX_MOMENTS: usize = 4;
/// Da quale vantaggio (centesimi di pedone, per chi gioca) la variante è "buona".
const GOOD_SCORE: i32 = 50;
/// Sotto questa differenza (centesimi di pedone) due idee si equivalgono quasi.
const NEARLY_EQUAL: i32 = 30;
/// Di quanto la mossa migliore deve valere di più per proporla (centesimi di pedone).
const BETTER_BY: i32 = 50;
/// Da quale punteggio la valutazione è un matto (vedi `score_for`).
const MATE_THRESHOLD: i32 = 50_000;
/// Quanti motivi raccontare al massimo in una combinazione.
const MAX_STORY: usize = 3;

/// Come si racconta un'idea.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Voice {
    /// L'idea di chi gioca la variante: «L'idea: … — si vede tra 3 mosse».
    Idea,
    /// Una strada possibile (dell'avversario): «Una strada possibile: … — si vedrebbe tra 3 mosse».
    Possibility,
}

/// Toglie da `tactics` i motivi che si vedono fino alla mezza mossa `until`
/// (al massimo [`MAX_STORY`], i più vicini): sono i passi di una combinazione.
fn take_story(tactics: &mut Vec<IdeaEvent>, until: usize) -> Vec<IdeaEvent> {
    let (story, rest): (Vec<IdeaEvent>, Vec<IdeaEvent>) =
        tactics.drain(..).partition(|event| event.ply <= until);
    *tactics = rest;
    let skip = story.len().saturating_sub(MAX_STORY);
    story.into_iter().skip(skip).collect()
}

/// "1 mossa", "3 mosse".
fn mate_moves(moves: usize) -> String {
    if moves == 1 {
        "1 mossa".to_owned()
    } else {
        format!("{moves} mosse")
    }
}

/// I passi di una combinazione in una frase: «prima …, poi …».
fn story_text(story: &[IdeaEvent]) -> String {
    match story {
        [] => String::new(),
        [only] => only.text.clone(),
        [first, rest @ ..] => std::iter::once(format!("prima {}", first.text))
            .chain(rest.iter().map(|event| format!("poi {}", event.text)))
            .collect::<Vec<_>>()
            .join(", "),
    }
}

/// Gli eventi della variante, dal più importante, e i motivi tattici in ordine di mossa.
///
/// - `assume_good`: la variante è buona per `side` anche senza valutazione
///   (le soluzioni dei puzzle): il materiale dato è un sacrificio;
/// - `previous`: la mossa giocata subito prima della variante, se si conosce
///   (vedi `motifs::motif_events`).
fn events(
    start: &Chess,
    steps: &[LineStep],
    side: Color,
    evaluation: Option<Evaluation>,
    assume_good: bool,
    previous: Option<&LineStep>,
) -> (Vec<IdeaEvent>, Vec<Motif>) {
    let score = evaluation.map(|evaluation| score_for(evaluation, side));
    // I motivi: quelli "fermi" dei rilevatori e quelli che si vedono mossa dopo mossa.
    let mut tactics = tactic_events(start, steps, side, TACTIC_PLIES);
    for event in motif_events(start, steps, side, TACTIC_PLIES, previous) {
        if !tactics
            .iter()
            .any(|known| known.card_id.is_some() && known.card_id == event.card_id)
        {
            tactics.push(event);
        }
    }
    tactics.sort_by_key(|event| event.ply);
    // Dopo il matto, le tattiche "ferme" della posizione finale (forchette,
    // inchiodature…) non contano più: la partita è finita.
    if let Some(last) = steps.len().checked_sub(1)
        && steps[last].mover == side
        && steps[last].after.is_checkmate()
    {
        tactics.retain(|event| {
            event.ply < last
                || !matches!(
                    event.card_id.as_deref().and_then(Motif::from_card_id),
                    Some(Motif::Fork | Motif::Pin | Motif::Skewer | Motif::Overload)
                )
        });
    }
    let mut motifs: Vec<Motif> = Vec::new();
    for motif in tactics
        .iter()
        .filter_map(|event| event.card_id.as_deref().and_then(Motif::from_card_id))
    {
        if !motifs.contains(&motif) {
            motifs.push(motif);
        }
    }
    let mut found = Vec::new();
    let mates = steps
        .last()
        .is_some_and(|step| step.mover == side && step.after.is_checkmate());
    if let Some(last) = steps.len().checked_sub(1)
        && (mates || score.is_some_and(|score| score >= MATE_THRESHOLD))
    {
        let moves = match evaluation {
            Some(Evaluation::Mate(moves)) => moves.unsigned_abs() as usize,
            _ => steps.iter().filter(|step| step.mover == side).count(),
        };
        let story = take_story(&mut tactics, last);
        let text = if story.is_empty() {
            format!("la variante porta al matto in {}", mate_moves(moves))
        } else {
            format!(
                "la variante porta al matto in {}: {}",
                mate_moves(moves),
                story_text(&story)
            )
        };
        found.push(IdeaEvent {
            kind: IdeaKind::Mate,
            ply: last,
            text,
            squares: story
                .last()
                .map(|event| event.squares.clone())
                .unwrap_or_default(),
            arrows: Vec::new(),
            card_id: story.first().and_then(|event| event.card_id.clone()),
        });
    }
    let good = assume_good || score.is_some_and(|score| score >= GOOD_SCORE);
    if let Some(material) = material_event(start, steps, side, good) {
        // Dei motivi tattici che portano a guadagnare materiale sono una
        // combinazione: diventano un solo evento, raccontato passo per passo.
        let story = if material.kind == IdeaKind::WinsMaterial {
            take_story(&mut tactics, material.ply)
        } else {
            Vec::new()
        };
        match story.last() {
            Some(key) => found.push(IdeaEvent {
                kind: IdeaKind::Combination,
                text: format!(
                    "una combinazione: {}, e {}",
                    story_text(&story),
                    material.text
                ),
                card_id: story.first().and_then(|event| event.card_id.clone()),
                ..key.clone()
            }),
            None => found.push(material),
        }
    }
    found.extend(tactics);
    found.extend(weakness_events(start, steps, side));
    found.extend(structure_events(start, steps));
    found.extend(route_events(steps, side));
    found.extend(pawn_break_events(steps, side));
    found.extend(open_file_events(start, steps, side));
    found.extend(quiet_plan_event(steps, side));
    // A parità di peso conta prima ciò che si vede prima.
    found.sort_by_key(|event| (std::cmp::Reverse(event.kind.weight()), event.ply));
    (found, motifs)
}

/// La frase con la prima lettera minuscola.
fn lowercase_first(text: &str) -> String {
    let mut chars = text.chars();
    chars.next().map_or_else(String::new, |first| {
        first.to_lowercase().collect::<String>() + chars.as_str()
    })
}

/// La frase con la prima lettera maiuscola.
fn uppercase_first(text: &str) -> String {
    let mut chars = text.chars();
    chars.next().map_or_else(String::new, |first| {
        first.to_uppercase().collect::<String>() + chars.as_str()
    })
}

/// Quando si vede un evento: "subito", "tra 2 mosse", … (al condizionale per le strade possibili).
fn when(ply: usize, voice: Voice) -> String {
    let moves = ply / 2 + 1;
    let verb = match voice {
        Voice::Idea => "si vede",
        Voice::Possibility => "si vedrebbe",
    };
    if moves <= 1 {
        format!("{verb} subito")
    } else {
        format!("{verb} tra {moves} mosse")
    }
}

/// La frase principale.
fn headline(top: Option<&IdeaEvent>, voice: Voice) -> String {
    let (open, calm) = match voice {
        Voice::Idea => (
            "L'idea",
            "L'idea: migliorare la posizione con calma, senza concedere nulla all'avversario",
        ),
        Voice::Possibility => (
            "Una strada possibile",
            "Una strada tranquilla: migliorare i pezzi con calma, senza un obiettivo immediato",
        ),
    };
    top.map_or_else(
        || calm.to_owned(),
        |event| {
            format!(
                "{open}: {} — {}",
                lowercase_first(&event.text),
                when(event.ply, voice)
            )
        },
    )
}

/// Le posizioni da far scorrere sulla scacchiera.
fn frames(start: &Chess, steps: &[LineStep]) -> Vec<IdeaFrame> {
    let frame = |position: &Chess, last_move: Option<LegalMove>| IdeaFrame {
        board_fen: Fen::from_position(position, EnPassantMode::Legal)
            .to_string()
            .split(' ')
            .next()
            .unwrap_or_default()
            .to_owned(),
        turn: Side::from(position.turn()),
        in_check: position.is_check(),
        last_move,
    };
    std::iter::once(frame(start, None))
        .chain(steps.iter().map(|step| {
            frame(
                &step.after,
                Some(LegalMove {
                    from: step.from.to_string(),
                    to: step.to.to_string(),
                    uci: format!("{}{}", step.from, step.to),
                }),
            )
        }))
        .collect()
}

/// Le schede citate dai momenti chiave (una volta sola ciascuna), con la
/// frase del momento come "perché vale qui".
fn cards(cited: &[(&str, &str)], side: Color) -> Vec<CardView> {
    let mut views: Vec<CardView> = Vec::new();
    for (id, why) in cited {
        if views.iter().any(|view| view.id == *id) {
            continue;
        }
        if let Some(card) = all_cards().iter().find(|card| card.id == *id) {
            views.push(card_view(card, uppercase_first(why), Some(side)));
        }
    }
    views
}

/// Un evento come momento chiave dell'interfaccia.
fn moment(event: &IdeaEvent) -> IdeaMoment {
    IdeaMoment {
        ply: u32::try_from(event.ply).unwrap_or(u32::MAX),
        text: uppercase_first(&event.text),
        squares: event.squares.iter().map(ToString::to_string).collect(),
        arrows: event
            .arrows
            .iter()
            .map(|(from, to)| FactArrow {
                from: from.to_string(),
                to: to.to_string(),
            })
            .collect(),
        card_id: event.card_id.clone(),
    }
}

/// L'idea della variante `line` (mosse UCI) a partire da `start`, per `side`.
#[must_use]
pub fn describe_idea(
    start: &Chess,
    line: &[String],
    side: Color,
    evaluation: Option<Evaluation>,
) -> IdeaView {
    describe_idea_as(start, line, side, evaluation, Voice::Idea)
}

/// Come [`describe_idea`], raccontata con la voce `voice`.
#[must_use]
pub(crate) fn describe_idea_as(
    start: &Chess,
    line: &[String],
    side: Color,
    evaluation: Option<Evaluation>,
    voice: Voice,
) -> IdeaView {
    build_idea(start, line, side, evaluation, voice, false, None)
}

/// L'idea della soluzione di un puzzle: `puzzle_fen` è la posizione del puzzle,
/// `moves` le sue mosse in UCI (la prima è dell'avversario, poi la soluzione).
/// Restituisce `None` se i dati del puzzle non sono validi.
#[must_use]
pub fn solution_idea(puzzle_fen: &str, moves: &[String]) -> Option<IdeaView> {
    let (first, solution) = moves.split_first()?;
    let puzzle = chess_position(puzzle_fen).ok()?;
    let opening = walk(&puzzle, std::slice::from_ref(first));
    let start = opening.first()?.after.clone();
    let side = start.turn();
    Some(build_idea(
        &start,
        solution,
        side,
        None,
        Voice::Idea,
        true,
        opening.first(),
    ))
}

/// L'idea di una variante (vedi [`describe_idea`]); `assume_good` e `previous` come in [`events`].
fn build_idea(
    start: &Chess,
    line: &[String],
    side: Color,
    evaluation: Option<Evaluation>,
    voice: Voice,
    assume_good: bool,
    previous: Option<&LineStep>,
) -> IdeaView {
    let limit = line.len().min(LOOK_AHEAD_PLIES);
    let steps = walk(start, &line[..limit]);
    let (found, motifs) = events(start, &steps, side, evaluation, assume_good, previous);
    let mut chosen: Vec<&IdeaEvent> = found.iter().take(MAX_MOMENTS).collect();
    chosen.sort_by_key(|event| event.ply);
    let chosen_cards: Vec<(&str, &str)> = chosen
        .iter()
        .filter_map(|event| Some((event.card_id.as_deref()?, event.text.as_str())))
        .collect();
    IdeaView {
        side: Side::from(side),
        headline: headline(found.first(), voice),
        key_ply: found
            .first()
            .map(|event| u32::try_from(event.ply).unwrap_or(u32::MAX)),
        evaluation,
        moves: steps.iter().map(|step| step.san.clone()).collect(),
        first_move_number: start.fullmoves().get(),
        first_side: Side::from(start.turn()),
        moments: chosen.into_iter().map(moment).collect(),
        cards: cards(&chosen_cards, side),
        frames: frames(start, &steps),
        motifs: motifs.into_iter().map(motif_view).collect(),
    }
}

/// La valutazione dal punto di vista di `side`: "+1.2" vuol dire che `side` sta meglio.
fn number_for(evaluation: Evaluation, side: Color) -> String {
    let own = match (evaluation, side) {
        (Evaluation::Centipawns(value), Color::Black) => Evaluation::Centipawns(-value),
        (Evaluation::Mate(moves), Color::Black) => Evaluation::Mate(-moves),
        (other, _) => other,
    };
    let number = evaluation_number(own);
    match own {
        Evaluation::Mate(moves) if moves > 0 => format!("il matto ({number})"),
        Evaluation::Mate(_) => format!("il matto per l'avversario ({number})"),
        Evaluation::Centipawns(_) => number,
    }
}

/// Perché la prima idea è preferibile alla seconda, per `side`.
fn compare(best: Evaluation, second: Evaluation, side: Color) -> String {
    let gap = score_for(best, side) - score_for(second, side);
    if gap < NEARLY_EQUAL {
        format!(
            "Le prime due idee si equivalgono quasi ({} e {} per {}): sono due buoni modi di continuare.",
            number_for(best, side),
            number_for(second, side),
            side_name(side)
        )
    } else {
        format!(
            "Stockfish preferisce la prima idea: vale {} per {}, la seconda solo {}.",
            number_for(best, side),
            side_name(side),
            number_for(second, side)
        )
    }
}

/// I piani della strategia di chi muove confermati dalla variante migliore:
/// quelli per cui la variante contiene un evento dello stesso tipo (vedi
/// `strategy_plans::confirming_kinds`).
fn confirmed_plans(position: &Chess, best: Option<&(Vec<String>, Evaluation)>) -> Vec<String> {
    let Some((line, evaluation)) = best else {
        return Vec::new();
    };
    let side = position.turn();
    let limit = line.len().min(LOOK_AHEAD_PLIES);
    let steps = walk(position, &line[..limit]);
    let (found, _) = events(position, &steps, side, Some(*evaluation), false, None);
    let phase = analysis::game_phase::game_phase(position);
    let strategy = describe_strategy(position, phase, &recognize(position, phase));
    strategy
        .plans
        .iter()
        .filter(|plan| {
            let key = plan.key.rsplit('/').next().unwrap_or_default();
            let kinds = confirming_kinds(key);
            found.iter().any(|event| kinds.contains(&event.kind))
        })
        .map(|plan| plan.key.clone())
        .collect()
}

/// Le idee della posizione: una per ogni variante di Stockfish (dalla migliore),
/// per chi deve muovere, e il confronto tra la prima e le altre.
#[must_use]
pub fn describe_position_ideas(
    position: &Chess,
    lines: &[(Vec<String>, Evaluation)],
) -> PositionIdeas {
    let side = position.turn();
    let ideas: Vec<IdeaView> = lines
        .iter()
        .map(|(line, evaluation)| describe_idea(position, line, side, Some(*evaluation)))
        .collect();
    let comparison = match (lines.first(), lines.get(1)) {
        (Some((_, best)), Some((_, second))) if ideas.len() > 1 => {
            Some(compare(*best, *second, side))
        }
        _ => None,
    };
    PositionIdeas {
        fen: Fen::from_position(position, EnPassantMode::Legal).to_string(),
        ideas,
        comparison,
        confirmed_plans: confirmed_plans(position, lines.first()),
    }
}

/// L'idea della mossa `played` (UCI) giocata in `before`.
///
/// - `after_line`: la variante di Stockfish dalla posizione dopo la mossa, con la sua valutazione;
/// - `best_before`: la variante migliore della posizione prima della mossa, se la si conosce.
#[must_use]
pub fn describe_move_idea(
    before: &Chess,
    played: &str,
    after_line: &(Vec<String>, Evaluation),
    best_before: Option<&(Vec<String>, Evaluation)>,
) -> MoveIdea {
    let mover = before.turn();
    let (after_moves, after_evaluation) = after_line;
    let line: Vec<String> = std::iter::once(played.to_owned())
        .chain(after_moves.iter().cloned())
        .collect();
    let played_idea = describe_idea(before, &line, mover, Some(*after_evaluation));
    let better = best_before.filter(|(best_line, best_evaluation)| {
        best_line.first().map(String::as_str) != Some(played)
            && score_for(*best_evaluation, mover) - score_for(*after_evaluation, mover) >= BETTER_BY
    });
    let better_idea = better.map(|(best_line, best_evaluation)| {
        describe_idea(before, best_line, mover, Some(*best_evaluation))
    });
    // La mossa migliore non si scrive qui: è la prima di `better_idea`, e
    // l'interfaccia la scrive nella notazione scelta nelle Impostazioni.
    let comparison = better.map(|(_, best_evaluation)| {
        format!(
            "Stockfish preferiva un'altra mossa: dopo la tua la posizione vale {} per {}, con la sua {}.",
            number_for(*after_evaluation, mover),
            side_name(mover),
            number_for(*best_evaluation, mover)
        )
    });
    let lesson = better.map(|(best_line, _)| lesson(before, played, after_moves, best_line));
    MoveIdea {
        played: played_idea,
        better: better_idea,
        comparison,
        lesson,
    }
}

/// La posizione raggiunta dalla partita (partenza e mosse in UCI).
pub(crate) fn reached(start_fen: &str, moves: &[String]) -> Result<Chess, TutorError> {
    let game = replay(start_fen, moves)?;
    chess_position(&game.fen())
}

/// Le idee della posizione raggiunta dalla partita, dalle varianti di Stockfish.
///
/// # Errors
///
/// [`TutorError`] se la posizione o una mossa della partita non sono valide.
pub fn position_ideas(
    start_fen: &str,
    moves: &[String],
    lines: &[(Vec<String>, Evaluation)],
) -> Result<PositionIdeas, TutorError> {
    Ok(describe_position_ideas(&reached(start_fen, moves)?, lines))
}

/// L'idea della mossa `played` (UCI), giocata dopo `moves_before`.
///
/// # Errors
///
/// [`TutorError`] se la posizione, una mossa della partita o la mossa giocata non sono valide.
pub fn move_idea(
    start_fen: &str,
    moves_before: &[String],
    played: &str,
    after_line: &(Vec<String>, Evaluation),
    best_before: Option<&(Vec<String>, Evaluation)>,
) -> Result<MoveIdea, TutorError> {
    let before = reached(start_fen, moves_before)?;
    let legal = shakmaty::uci::UciMove::from_ascii(played.as_bytes())
        .ok()
        .and_then(|uci| uci.to_move(&before).ok())
        .is_some();
    if !legal {
        return Err(TutorError::IllegalMove(played.to_owned()));
    }
    Ok(describe_move_idea(&before, played, after_line, best_before))
}

#[cfg(test)]
mod tests {
    use engine::Evaluation;
    use shakmaty::fen::Fen;
    use shakmaty::{CastlingMode, Chess};

    use super::{describe_idea, describe_move_idea, describe_position_ideas};

    fn position(fen: &str) -> Chess {
        fen.parse::<Fen>()
            .expect("FEN")
            .into_position(CastlingMode::Standard)
            .expect("posizione")
    }

    fn line(moves: &[&str]) -> Vec<String> {
        moves.iter().map(|uci| (*uci).to_owned()).collect()
    }

    #[test]
    fn a_knight_fork_that_wins_the_rook_is_a_combination() {
        // Il cavallo bianco in e5 salta in c6: scacco al re in d8 e attacco alla torre in a7.
        let start = position("3k4/r7/8/4N3/8/8/8/4K3 w - - 0 1");
        let idea = describe_idea(
            &start,
            &line(&["e5c6", "d8d7", "c6a7"]),
            shakmaty::Color::White,
            Some(Evaluation::Centipawns(500)),
        );

        assert!(idea.headline.starts_with("L'idea:"), "{}", idea.headline);
        assert!(
            idea.headline.contains("vince una torre"),
            "{}",
            idea.headline
        );
        assert_eq!(idea.frames.len(), 4, "partenza più tre mosse");
        assert_eq!(idea.moves, ["Nc6+", "Kd7", "Nxa7"]);
    }

    #[test]
    fn a_knight_manoeuvre_to_an_outpost_is_explained() {
        // Il Bianco spinge e5 e il cavallo va d2–f3–g5, vicino al re nero.
        let start = position("4k3/8/3p4/8/3PP3/8/3N4/4K3 w - - 0 1");
        let idea = describe_idea(
            &start,
            &line(&["e4e5", "e8e7", "d2f3", "e7e6", "f3g5"]),
            shakmaty::Color::White,
            Some(Evaluation::Centipawns(80)),
        );

        assert!(!idea.moments.is_empty(), "almeno un momento chiave");
    }

    #[test]
    fn several_lines_are_compared() {
        let start = Chess::default();
        let ideas = describe_position_ideas(
            &start,
            &[
                (line(&["e2e4", "e7e5"]), Evaluation::Centipawns(40)),
                (line(&["d2d4", "d7d5"]), Evaluation::Centipawns(30)),
            ],
        );

        assert_eq!(ideas.ideas.len(), 2);
        assert!(ideas.comparison.expect("confronto").contains("+0.4"));
    }

    #[test]
    fn a_much_better_move_is_proposed_with_its_idea() {
        let start = position("3k4/r7/8/4N3/8/8/8/4K3 w - - 0 1");
        let idea = describe_move_idea(
            &start,
            "e1d2",
            &(line(&["a7a1"]), Evaluation::Centipawns(-100)),
            Some(&(line(&["e5c6", "d8d7", "c6a7"]), Evaluation::Centipawns(500))),
        );

        assert_eq!(idea.played.moves.first().map(String::as_str), Some("Kd2"));
        let better = idea.better.expect("mossa migliore");
        assert_eq!(better.moves.first().map(String::as_str), Some("Nc6+"));
        assert!(idea.comparison.expect("confronto").contains("+5.0"));
    }
}
