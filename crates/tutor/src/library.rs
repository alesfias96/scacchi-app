//! La base di conoscenza: tutte le schede della cartella `knowledge/`.

use std::sync::LazyLock;

use crate::knowledge_card::KnowledgeCard;
use crate::topic::Topic;

// L'elenco delle schede, preparato da `build.rs` durante la compilazione.
include!(concat!(env!("OUT_DIR"), "/knowledge_files.rs"));

/// Le schede, lette la prima volta che servono. Una scheda non valida viene
/// saltata (non dovrebbe mai succedere: i test le controllano tutte).
static LIBRARY: LazyLock<Vec<KnowledgeCard>> = LazyLock::new(|| {
    KNOWLEDGE_FILES
        .iter()
        .filter_map(|(id, text)| KnowledgeCard::parse(id, text).ok())
        .collect()
});

/// Tutte le schede.
#[must_use]
pub fn all_cards() -> &'static [KnowledgeCard] {
    &LIBRARY
}

/// La scheda di un argomento con la chiave indicata (es. struttura `"carlsbad"`).
#[must_use]
pub fn card_for(topic: Topic, key: &str) -> Option<&'static KnowledgeCard> {
    LIBRARY.iter().find(|card| {
        card.triggers
            .iter()
            .any(|trigger| trigger.topic == topic && trigger.key == key)
    })
}

/// `true` se il nome di un'apertura di Lichess appartiene alla famiglia `family`.
///
/// Vale sia per `"Italian Game: Giuoco Piano"` (comincia con la famiglia) sia
/// per `"Indian Defense: London System"` (la famiglia è dopo i due punti).
fn belongs_to(name: &str, family: &str) -> bool {
    name.starts_with(family) || name.contains(&format!(": {family}"))
}

/// La scheda dell'apertura giocata, dato il suo nome di Lichess.
///
/// Se più schede vanno bene si sceglie la più precisa (il nome più lungo):
/// ad esempio "Semi-Slav Defense" invece di "Slav Defense".
#[must_use]
pub fn opening_card(name: &str) -> Option<&'static KnowledgeCard> {
    LIBRARY
        .iter()
        .filter(|card| card.topic == Topic::Opening)
        .filter_map(|card| {
            card.triggers
                .iter()
                .filter(|trigger| belongs_to(name, &trigger.key))
                .map(|trigger| trigger.key.len())
                .max()
                .map(|length| (length, card))
        })
        .max_by_key(|(length, _)| *length)
        .map(|(_, card)| card)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::{KNOWLEDGE_FILES, all_cards, belongs_to, card_for, opening_card};
    use crate::knowledge_card::KnowledgeCard;
    use crate::recognition::KNOWN_KEYS;
    use crate::topic::Topic;

    /// L'elenco delle aperture di Lichess, per controllare i nomi usati nelle schede.
    const OPENINGS: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../data/openings/lichess-openings.tsv"
    ));

    #[test]
    fn every_card_is_valid() {
        for (id, text) in KNOWLEDGE_FILES {
            if let Err(error) = KnowledgeCard::parse(id, text) {
                panic!("{error}");
            }
        }
        assert_eq!(all_cards().len(), KNOWLEDGE_FILES.len());
    }

    #[test]
    fn every_card_has_text_and_every_plan_has_steps() {
        for card in all_cards() {
            assert!(
                !card.content.summary.trim().is_empty(),
                "{}: manca il riassunto",
                card.id
            );
            assert!(
                !card.content.ideas.is_empty(),
                "{}: mancano le idee",
                card.id
            );
            for plan in &card.content.plans {
                assert!(!plan.steps.is_empty(), "{}: un piano senza passi", card.id);
            }
        }
    }

    #[test]
    fn every_trigger_is_recognized_by_the_tutor() {
        let names: Vec<&str> = OPENINGS
            .lines()
            .skip(1)
            .filter_map(|line| line.split('\t').nth(1))
            .collect();
        for card in all_cards() {
            for trigger in &card.triggers {
                if trigger.topic == Topic::Opening {
                    assert!(
                        names.iter().any(|name| belongs_to(name, &trigger.key)),
                        "{}: nessuna apertura si chiama «{}»",
                        card.id,
                        trigger.key
                    );
                } else {
                    assert!(
                        KNOWN_KEYS.contains(&(trigger.topic, trigger.key.as_str())),
                        "{}: il tutor non riconosce «{}»",
                        card.id,
                        trigger.key
                    );
                }
            }
        }
    }

    #[test]
    fn everything_the_tutor_recognizes_has_a_card() {
        let missing: HashSet<_> = KNOWN_KEYS
            .iter()
            .filter(|(topic, key)| card_for(*topic, key).is_none())
            .collect();

        assert_eq!(missing, HashSet::new());
    }

    #[test]
    fn picks_the_most_precise_opening_card() {
        let semi_slav = opening_card("Semi-Slav Defense: Meran Variation").expect("scheda");
        let london = opening_card("Indian Defense: London System").expect("scheda");

        assert!(
            semi_slav
                .triggers
                .iter()
                .any(|trigger| trigger.key == "Semi-Slav Defense")
        );
        assert!(
            london
                .triggers
                .iter()
                .any(|trigger| trigger.key == "London System")
        );
        assert_eq!(opening_card("Amar Opening"), None);
    }
}
