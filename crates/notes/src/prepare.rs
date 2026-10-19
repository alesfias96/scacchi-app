//! Come si prepara un appunto da salvare, partendo da ciò che si è scritto.

use std::sync::atomic::{AtomicU32, Ordering};

use crate::folding::fold;
use crate::note::{Note, NoteInput};

/// Al massimo quante etichette per appunto.
pub const MAX_TAGS: usize = 12;
/// Al massimo quanti caratteri per etichetta.
pub const MAX_TAG_LENGTH: usize = 40;

/// Un contatore che rende diversi due nomi creati nello stesso istante.
static SEQUENCE: AtomicU32 = AtomicU32::new(0);

/// Un nome unico per un appunto nuovo (es. `"nota-19a3f2c4b10-0"`).
#[must_use]
pub fn new_note_id(now_ms: i64) -> String {
    let sequence = SEQUENCE.fetch_add(1, Ordering::Relaxed);
    format!("nota-{now_ms:x}-{sequence:x}")
}

/// Le etichette ripulite: senza spazi in più, senza quelle vuote, senza
/// doppioni (anche se scritti con maiuscole o accenti diversi: vale il primo),
/// accorciate a [`MAX_TAG_LENGTH`] caratteri e al massimo [`MAX_TAGS`].
#[must_use]
pub fn clean_tags(tags: &[String]) -> Vec<String> {
    let mut seen: Vec<String> = Vec::new();
    let mut cleaned = Vec::new();
    for tag in tags {
        let tag: String = tag
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .chars()
            .take(MAX_TAG_LENGTH)
            .collect();
        let key = fold(&tag);
        if key.is_empty() || seen.contains(&key) {
            continue;
        }
        seen.push(key);
        cleaned.push(tag);
        if cleaned.len() == MAX_TAGS {
            break;
        }
    }
    cleaned
}

/// L'appunto da salvare.
///
/// - `existing`: l'appunto già salvato con lo stesso nome, se c'è (ne
///   conserva la data di creazione);
/// - `now_ms`: l'istante della modifica.
#[must_use]
pub fn prepare_note(input: NoteInput, existing: Option<&Note>, now_ms: i64) -> Note {
    let id = input.id.unwrap_or_else(|| new_note_id(now_ms));
    Note {
        title: input.title.trim().to_owned(),
        body: input.body,
        tags: clean_tags(&input.tags),
        link: input.link,
        created_at_ms: existing.map_or(now_ms, |note| note.created_at_ms),
        updated_at_ms: now_ms,
        id,
    }
}

#[cfg(test)]
mod tests {
    use super::{clean_tags, prepare_note};
    use crate::note::{NoteInput, NoteLink};

    fn tags(list: &[&str]) -> Vec<String> {
        list.iter().map(|tag| (*tag).to_owned()).collect()
    }

    #[test]
    fn tags_are_cleaned_and_deduplicated() {
        assert_eq!(
            clean_tags(&tags(&[
                "  Siciliana ",
                "siciliana",
                "",
                "Finali  di torre",
                "SICILIANA"
            ])),
            ["Siciliana", "Finali di torre"]
        );
    }

    #[test]
    fn a_new_note_gets_an_id_and_an_edited_one_keeps_its_creation_date() {
        let input = NoteInput {
            id: None,
            title: "  Idea  ".to_owned(),
            body: "Spingere f5".to_owned(),
            tags: tags(&["Attacco"]),
            link: Some(NoteLink {
                fen: "8/8/8/8/8/8/8/K6k w - - 0 1".to_owned(),
                game_id: None,
                ply: None,
            }),
        };
        let created = prepare_note(input.clone(), None, 1_000);
        assert!(created.id.starts_with("nota-"));
        assert_eq!(created.title, "Idea");
        assert_eq!(created.created_at_ms, 1_000);

        let edited = prepare_note(
            NoteInput {
                id: Some(created.id.clone()),
                ..input
            },
            Some(&created),
            2_000,
        );
        assert_eq!(edited.id, created.id);
        assert_eq!((edited.created_at_ms, edited.updated_at_ms), (1_000, 2_000));
    }
}
