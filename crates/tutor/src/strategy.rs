//! La strategia della posizione: il bilancio "chi ha cosa" e i piani consigliati.
//!
//! Mette insieme i pezzi:
//! 1. il bilancio degli squilibri (vedi `imbalances`), scritto voce per voce;
//! 2. i piani delle regole (vedi `strategy_plans`), per chi muove e per
//!    l'avversario; se l'avversario ha un piano forte, il primo piano di chi
//!    muove diventa la **profilassi** (impedirlo);
//! 3. per ogni piano, dalla sua scheda: il titolo, il "come" (i passi), il
//!    "quando usarlo", il "quando no" e i segnali che funziona.

use analysis::GamePhase;
use analysis::wording::{piece_phrase, side_name};
use chess_core::Side;
use shakmaty::{Chess, Color, Piece, Position, Square};

use crate::card_view::card_view;
use crate::explanation_view::CardView;
use crate::imbalances::{CenterKind, Imbalances, SideImbalances, imbalances};
use crate::knowledge_card::{KnowledgeCard, PlanFor};
use crate::library::card_for;
use crate::recognition::Recognition;
use crate::strategy_plans::{PlanCandidate, lowercase_first, plan_candidates, prophylaxis};
use crate::strategy_view::{ImbalanceItem, StrategyPlanView, StrategyView};

/// Quanti piani consigliare a chi muove.
const MAX_PLANS: usize = 2;
/// Da quale differenza di spazio (case) lo spazio è un vantaggio.
const SPACE_GAP: u32 = 3;

/// Le case scritte come testo.
fn names(squares: &[Square]) -> Vec<String> {
    squares.iter().map(ToString::to_string).collect()
}

/// Una voce del bilancio.
fn item(color: Color, text: String, squares: &[Square]) -> ImbalanceItem {
    ImbalanceItem {
        side: Side::from(color),
        text,
        squares: names(squares),
    }
}

/// Le voci del bilancio di un lato legate a materiale e pedoni.
fn pawn_items(us: &SideImbalances, them: &SideImbalances) -> Vec<ImbalanceItem> {
    let color = us.color;
    let join = |squares: &[Square]| analysis::wording::join_list(&names(squares));
    let mut items = Vec::new();
    if us.material >= 1 {
        items.push(item(
            color,
            format!(
                "avanti di {} {} di materiale",
                us.material,
                if us.material == 1 { "punto" } else { "punti" }
            ),
            &[],
        ));
    }
    if us.bishop_pair {
        items.push(item(color, "la coppia degli alfieri".to_owned(), &[]));
    }
    if !us.passed.is_empty() {
        items.push(item(
            color,
            format!("pedone passato in {}", join(&us.passed)),
            &us.passed,
        ));
    }
    if !us.isolated.is_empty() {
        items.push(item(
            color,
            format!("pedone isolato in {}", join(&us.isolated)),
            &us.isolated,
        ));
    }
    if !us.backward.is_empty() {
        items.push(item(
            color,
            format!("pedone arretrato in {}", join(&us.backward)),
            &us.backward,
        ));
    }
    for file in &us.doubled {
        items.push(item(
            color,
            format!("pedoni doppiati sulla colonna {}", file.char()),
            &[],
        ));
    }
    for (wing, own, other) in [
        ("di donna", us.queenside_pawns, them.queenside_pawns),
        ("di re", us.kingside_pawns, them.kingside_pawns),
    ] {
        if own > other && own >= 2 {
            items.push(item(
                color,
                format!("maggioranza sull'ala {wing} ({own} pedoni contro {other})"),
                &[],
            ));
        }
    }
    if us.space >= them.space + SPACE_GAP {
        items.push(item(
            color,
            format!("più spazio ({} case contro {})", us.space, them.space),
            &[],
        ));
    }
    items
}

/// Le voci del bilancio di un lato legate a pezzi e re.
fn piece_items(us: &SideImbalances, phase: GamePhase) -> Vec<ImbalanceItem> {
    let color = us.color;
    let join = |squares: &[Square]| analysis::wording::join_list(&names(squares));
    let mut items = Vec::new();
    if let Some(worst) = us.worst_piece.filter(|_| phase != GamePhase::Opening) {
        let piece = Piece {
            color,
            role: worst.role,
        };
        items.push(item(
            color,
            format!(
                "pezzo peggiore: {} ({} {})",
                piece_phrase(piece, worst.square),
                worst.mobility,
                if worst.mobility == 1 {
                    "mossa"
                } else {
                    "mosse"
                }
            ),
            &[worst.square],
        ));
    }
    if !us.bad_bishops.is_empty() {
        items.push(item(
            color,
            format!("alfiere cattivo in {}", join(&us.bad_bishops)),
            &us.bad_bishops,
        ));
    }
    for file in &us.files_with_rooks {
        items.push(item(
            color,
            format!(
                "torre o donna sulla colonna {} senza propri pedoni",
                file.char()
            ),
            &[],
        ));
    }
    if let Some(outpost) = us.outposts.first().filter(|_| us.minors.0 > 0) {
        items.push(item(
            color,
            format!("avamposto libero per un cavallo in {outpost}"),
            &[*outpost],
        ));
    }
    if us.holes_near_enemy_king.len() >= 2 {
        items.push(item(
            color,
            format!(
                "case deboli intorno al re avversario: {}",
                join(&us.holes_near_enemy_king)
            ),
            &us.holes_near_enemy_king,
        ));
    }
    if phase != GamePhase::Endgame {
        if us.king_in_center {
            items.push(item(color, "re ancora al centro".to_owned(), &[]));
        } else if us.castled && us.king_shield <= 1 {
            items.push(item(
                color,
                "re arroccato ma poco protetto dai pedoni".to_owned(),
                &[],
            ));
        }
        if us.undeveloped >= 2 {
            items.push(item(
                color,
                format!("{} pezzi minori ancora da sviluppare", us.undeveloped),
                &[],
            ));
        }
    }
    items
}

/// Le voci del bilancio di un lato (solo quelle che contano).
fn side_items(us: &SideImbalances, them: &SideImbalances, phase: GamePhase) -> Vec<ImbalanceItem> {
    let mut items = pawn_items(us, them);
    items.extend(piece_items(us, phase));
    items
}

/// Com'è il centro, in una frase.
fn center_text(center: CenterKind) -> String {
    match center {
        CenterKind::Closed => "Centro chiuso: i pedoni centrali sono bloccati, si gioca sulle ali.",
        CenterKind::Open => "Centro aperto: quasi senza pedoni centrali, contano i pezzi attivi.",
        CenterKind::Mobile => {
            "Centro mobile: i pedoni centrali possono ancora muoversi o cambiarsi."
        }
    }
    .to_owned()
}

/// I passi del piano della scheda per chi segue il piano (o le idee, se la scheda non ha piani).
fn steps_for(card: &KnowledgeCard, color: Color) -> Vec<String> {
    let content = &card.content;
    content
        .plans
        .iter()
        .find(|plan| match plan.for_side {
            PlanFor::Owner | PlanFor::Both => true,
            PlanFor::White => color == Color::White,
            PlanFor::Black => color == Color::Black,
            PlanFor::Opponent => false,
        })
        .map_or_else(
            || content.ideas.iter().take(3).cloned().collect(),
            |plan| plan.steps.clone(),
        )
}

/// La frase con la prima lettera maiuscola.
fn uppercase_first(text: &str) -> String {
    let mut chars = text.chars();
    chars.next().map_or_else(String::new, |first| {
        first.to_uppercase().collect::<String>() + chars.as_str()
    })
}

/// Un piano con il contenuto della sua scheda.
fn plan_view(candidate: &PlanCandidate) -> Option<(StrategyPlanView, CardView)> {
    let card = card_for(candidate.topic, candidate.key)?;
    let content = &card.content;
    let view = StrategyPlanView {
        key: card.id.clone(),
        side: Side::from(candidate.side),
        title: content.title.clone(),
        why: uppercase_first(&candidate.why),
        how: steps_for(card, candidate.side),
        use_when: content.use_when.clone(),
        avoid_when: content.avoid_when.clone(),
        signs: content.signs.clone(),
        squares: names(&candidate.squares),
    };
    let card_view = card_view(card, uppercase_first(&candidate.why), Some(candidate.side));
    Some((view, card_view))
}

/// Il bilancio della posizione e i piani di chi muove (e dell'avversario).
#[must_use]
pub fn describe_strategy(
    position: &Chess,
    phase: GamePhase,
    recognized: &[Recognition],
) -> StrategyView {
    let found: Imbalances = imbalances(position, phase);
    let turn = position.turn();
    let mut imbalances_items = side_items(found.of(turn), found.of(turn.other()), phase);
    imbalances_items.extend(side_items(found.of(turn.other()), found.of(turn), phase));

    let opponent_candidates = plan_candidates(position, &found, recognized, turn.other());
    let opponent = opponent_candidates.iter().find_map(plan_view);
    let mut own = plan_candidates(position, &found, recognized, turn);
    if let (Some(best), Some((view, _))) = (opponent_candidates.first(), opponent.as_ref())
        && let Some(defence) = prophylaxis(best, &view.title, turn)
    {
        own.insert(0, defence);
        own.sort_by_key(|candidate| std::cmp::Reverse(candidate.score));
    }
    let chosen: Vec<(StrategyPlanView, CardView)> =
        own.iter().filter_map(plan_view).take(MAX_PLANS).collect();
    // Se il piano dell'avversario è uguale a uno dei nostri (es. il centro
    // chiuso vale per tutti e due), si mostra il suo piano successivo.
    let is_ours = |view: &StrategyPlanView| chosen.iter().any(|(own, _)| own.key == view.key);
    let opponent = opponent.filter(|(view, _)| !is_ours(view)).or_else(|| {
        opponent_candidates
            .iter()
            .filter_map(plan_view)
            .find(|(view, _)| !is_ours(view))
    });

    let mut cards: Vec<CardView> = Vec::new();
    for (_, card) in chosen.iter().chain(opponent.iter()) {
        if !cards.iter().any(|known| known.id == card.id) {
            cards.push(card.clone());
        }
    }
    StrategyView {
        center: format!("{} {}", center_text(found.center), whose_turn(turn)),
        imbalances: imbalances_items,
        plans: chosen.into_iter().map(|(view, _)| view).collect(),
        // Il "perché" è scritto per chi segue il piano («hai…»): si dice per chi è.
        opponent_plan: opponent.map(|(view, _)| StrategyPlanView {
            why: format!(
                "Per {}: {}",
                side_name(turn.other()),
                lowercase_first(&view.why)
            ),
            ..view
        }),
        cards,
    }
}

/// Chi muove, per la frase del centro.
fn whose_turn(turn: Color) -> String {
    format!(
        "I piani qui sotto sono per {}, che deve muovere.",
        side_name(turn)
    )
}

#[cfg(test)]
mod tests {
    use analysis::game_phase::game_phase;
    use shakmaty::fen::Fen;
    use shakmaty::{CastlingMode, Chess};

    use super::describe_strategy;
    use crate::recognition::recognize;

    #[test]
    fn gives_imbalances_plans_and_their_cards() {
        let position: Chess = "r1bq1rk1/pp2bppp/2p2n2/3p4/3P4/2NBPN2/PP3PPP/R2QK2R w KQ - 0 10"
            .parse::<Fen>()
            .expect("FEN")
            .into_position(CastlingMode::Standard)
            .expect("posizione");
        let phase = game_phase(&position);
        let strategy = describe_strategy(&position, phase, &recognize(&position, phase));

        assert!(
            (1..=2).contains(&strategy.plans.len()),
            "{:?}",
            strategy.plans
        );
        let plan = &strategy.plans[0];
        assert!(!plan.why.is_empty() && !plan.how.is_empty(), "{plan:?}");
        assert!(strategy.cards.iter().any(|card| card.id == plan.key));
        assert!(strategy.center.contains("Bianco"));
    }
}
