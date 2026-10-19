//! La ricerca tra gli appunti e l'elenco delle etichette.
//!
//! Un appunto corrisponde alla ricerca se **ogni parola** cercata compare nel
//! titolo, nel testo o nelle etichette, senza badare a maiuscole e accenti
//! ("donna perche" trova "Perché la donna esce presto"). Il filtro per
//! etichetta tiene solo gli appunti con quell'etichetta.

use crate::folding::fold;
use crate::note::Note;

/// Cosa cercare. I campi vuoti non escludono nulla.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Deserialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct NoteQuery {
    /// Le parole da cercare.
    pub text: Option<String>,
    /// Solo gli appunti con questa etichetta.
    pub tag: Option<String>,
}

/// `true` se l'appunto corrisponde a tutte le parole cercate.
fn matches_words(note: &Note, words: &[&str]) -> bool {
    let haystack = fold(&format!(
        "{}\n{}\n{}",
        note.title,
        note.body,
        note.tags.join("\n")
    ));
    words.iter().all(|word| haystack.contains(word))
}

/// Gli appunti che corrispondono alla ricerca, dal più recente al più vecchio.
#[must_use]
pub fn search_notes(notes: Vec<Note>, query: &NoteQuery) -> Vec<Note> {
    let text = fold(query.text.as_deref().unwrap_or_default());
    let words: Vec<&str> = text.split(' ').filter(|word| !word.is_empty()).collect();
    let tag = query.tag.as_deref().map(fold).filter(|tag| !tag.is_empty());
    let mut found: Vec<Note> = notes
        .into_iter()
        .filter(|note| {
            tag.as_ref()
                .is_none_or(|tag| note.tags.iter().any(|own| fold(own) == *tag))
        })
        .filter(|note| matches_words(note, &words))
        .collect();
    found.sort_by_key(|note| std::cmp::Reverse(note.updated_at_ms));
    found
}

/// Tutte le etichette usate, una volta sola e in ordine alfabetico (senza
/// badare a maiuscole e accenti; vale il modo in cui sono scritte la prima volta).
#[must_use]
pub fn all_tags(notes: &[Note]) -> Vec<String> {
    let mut tags: Vec<(String, String)> = Vec::new();
    for tag in notes.iter().flat_map(|note| &note.tags) {
        let key = fold(tag);
        if !tags.iter().any(|(existing, _)| *existing == key) {
            tags.push((key, tag.clone()));
        }
    }
    tags.sort();
    tags.into_iter().map(|(_, tag)| tag).collect()
}

#[cfg(test)]
mod tests {
    use super::{NoteQuery, all_tags, search_notes};
    use crate::note::Note;

    fn note(id: &str, title: &str, body: &str, tags: &[&str], updated: i64) -> Note {
        Note {
            id: id.to_owned(),
            title: title.to_owned(),
            body: body.to_owned(),
            tags: tags.iter().map(|tag| (*tag).to_owned()).collect(),
            link: None,
            created_at_ms: 0,
            updated_at_ms: updated,
        }
    }

    fn sample() -> Vec<Note> {
        vec![
            note(
                "a",
                "Perché la donna esce presto",
                "Errore tipico",
                &["Aperture"],
                1,
            ),
            note("b", "Finale di torre", "Lucena e Philidor", &["Finali"], 3),
            note(
                "c",
                "",
                "La Siciliana Najdorf",
                &["Aperture", "Siciliana"],
                2,
            ),
        ]
    }

    fn ids(found: &[Note]) -> Vec<&str> {
        found.iter().map(|note| note.id.as_str()).collect()
    }

    #[test]
    fn every_word_must_appear_ignoring_accents() {
        let query = NoteQuery {
            text: Some("DONNA perche".to_owned()),
            tag: None,
        };
        assert_eq!(ids(&search_notes(sample(), &query)), ["a"]);
    }

    #[test]
    fn filters_by_tag_and_sorts_by_last_change() {
        let query = NoteQuery {
            text: None,
            tag: Some("aperture".to_owned()),
        };
        assert_eq!(ids(&search_notes(sample(), &query)), ["c", "a"]);
        assert_eq!(
            ids(&search_notes(sample(), &NoteQuery::default())),
            ["b", "c", "a"]
        );
    }

    #[test]
    fn lists_each_tag_once_in_alphabetical_order() {
        assert_eq!(all_tags(&sample()), ["Aperture", "Finali", "Siciliana"]);
    }
}
