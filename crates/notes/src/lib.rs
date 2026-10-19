//! # notes — gli appunti personali
//!
//! Uno spazio per scrivere e ritrovare idee, pensieri e note di studio. Ogni
//! appunto ha un titolo, un testo, delle etichette libere (es. "Siciliana") e,
//! se nasce da una posizione, il collegamento a quella posizione e alla partita.
//!
//! Questa crate contiene solo le regole: come si prepara un appunto da
//! salvare e come si cerca. Il salvataggio lo fa `storage`.
//!
//! Organizzazione (un file per responsabilità):
//! - [`note`]: l'appunto e il suo collegamento;
//! - [`prepare`]: dall'appunto scritto a quello da salvare (etichette ripulite, date);
//! - [`folding`]: il testo minuscolo e senza accenti, per la ricerca;
//! - [`search`]: la ricerca e l'elenco delle etichette.

pub mod folding;
pub mod note;
pub mod prepare;
pub mod search;

pub use folding::fold;
pub use note::{Note, NoteInput, NoteLink};
pub use prepare::{MAX_TAG_LENGTH, MAX_TAGS, clean_tags, new_note_id, prepare_note};
pub use search::{NoteQuery, all_tags, search_notes};
