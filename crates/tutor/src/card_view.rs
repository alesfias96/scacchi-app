//! Una scheda della base di conoscenza adattata alla posizione.

use shakmaty::Color;

use analysis::wording::side_name;

use crate::explanation_view::{CardView, PlanView};
use crate::knowledge_card::{KnowledgeCard, PlanFor};

/// Come chiamare chi segue un piano.
///
/// - `White`/`Black`: "Il Bianco" / "Il Nero" (o l'etichetta della scheda);
/// - `Owner`/`Opponent`: l'etichetta della scheda (es. "Chi ha l'isolano") e,
///   se si sa, il colore: "Chi ha l'isolano (il Bianco)".
fn plan_label(for_side: PlanFor, label: Option<&str>, owner: Option<Color>) -> String {
    let with_color = |base: &str, color: Option<Color>| match color {
        Some(color) => format!("{base} ({})", side_name(color)),
        None => base.to_owned(),
    };
    match for_side {
        PlanFor::White => label.unwrap_or("Il Bianco").to_owned(),
        PlanFor::Black => label.unwrap_or("Il Nero").to_owned(),
        PlanFor::Both => label.unwrap_or("Per tutti e due").to_owned(),
        PlanFor::Owner => with_color(label.unwrap_or("Chi ha questa posizione"), owner),
        PlanFor::Opponent => with_color(label.unwrap_or("L'avversario"), owner.map(Color::other)),
    }
}

/// La scheda pronta da mostrare.
///
/// - Riceve: la scheda, perché vale qui (`context`) e chi "ha" la cosa
///   riconosciuta (per dare un colore ai piani).
#[must_use]
pub fn card_view(card: &KnowledgeCard, context: String, owner: Option<Color>) -> CardView {
    let written = &card.content;
    CardView {
        id: card.id.clone(),
        topic: card.topic,
        title: written.title.clone(),
        context,
        summary: written.summary.clone(),
        ideas: written.ideas.clone(),
        plans: written
            .plans
            .iter()
            .map(|plan| PlanView {
                label: plan_label(plan.for_side, plan.label.as_deref(), owner),
                steps: plan.steps.clone(),
            })
            .collect(),
        mistakes: written.mistakes.clone(),
        use_when: written.use_when.clone(),
        avoid_when: written.avoid_when.clone(),
        signs: written.signs.clone(),
        puzzle_theme: written.puzzle_theme.clone(),
    }
}

#[cfg(test)]
mod tests {
    use shakmaty::Color;

    use super::plan_label;
    use crate::knowledge_card::PlanFor;

    #[test]
    fn gives_each_plan_its_player() {
        assert_eq!(
            plan_label(PlanFor::Owner, Some("Chi ha l'isolano"), Some(Color::White)),
            "Chi ha l'isolano (il Bianco)"
        );
        assert_eq!(
            plan_label(
                PlanFor::Opponent,
                Some("Chi lo attacca"),
                Some(Color::White)
            ),
            "Chi lo attacca (il Nero)"
        );
        assert_eq!(plan_label(PlanFor::Black, None, None), "Il Nero");
    }
}
