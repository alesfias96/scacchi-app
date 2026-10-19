//! # tutor — le spiegazioni delle posizioni
//!
//! Il tutor a tre livelli di `PROGETTO.md` §4.2:
//! 1. **i fatti verificati**: i rilevatori (crate `analysis`) e, se c'è, la
//!    valutazione di Stockfish;
//! 2. **la conoscenza**: le schede della cartella `knowledge/` (principi,
//!    strutture pedonali, aperture, mediogioco, finali), collegate alla
//!    posizione da regole verificabili;
//! 3. **il linguaggio**: frasi in italiano composte da regole. Il linguaggio
//!    non decide mai nulla: racconta solo ciò che i primi due livelli hanno verificato.
//!
//! I punti d'ingresso sono [`explain`] (spiega una posizione) e [`what_if`]
//! (cosa cambia se si gioca una mossa).
//!
//! Organizzazione (un file per responsabilità):
//! - [`topic`]: gli argomenti e i "quando vale" delle schede;
//! - [`knowledge_card`]: una scheda, com'è scritta nel suo file;
//! - [`library`]: tutte le schede, incluse nell'app durante la compilazione (`build.rs`);
//! - [`board_query`]: piccole domande sulla scacchiera, viste da un colore;
//! - [`recognition`]: ciò che il tutor riconosce e rimanda a una scheda;
//! - [`structures`], [`endgame_kind`], [`middlegame_themes`], [`principles`]: i riconoscitori;
//! - [`evaluation_words`]: la valutazione di Stockfish a parole;
//! - [`explanation`] e [`explanation_view`]: la spiegazione e la sua forma;
//! - [`card_view`]: una scheda adattata alla posizione (i piani con il colore giusto);
//! - [`what_if`]: gli scenari "e se…";
//! - [`tutor_error`]: gli errori possibili.

pub mod board_query;
pub mod card_view;
pub mod endgame_kind;
pub mod evaluation_words;
pub mod explanation;
pub mod explanation_view;
pub mod idea;
pub mod idea_events;
pub mod idea_view;
pub mod knowledge_card;
pub mod library;
pub mod line_walk;
pub mod middlegame_themes;
pub mod principles;
pub mod recognition;
pub mod structures;
pub mod topic;
pub mod tutor_error;
pub mod what_if;

pub use explanation::explain;
pub use explanation_view::{CardView, PlanView, TutorExplanation, TutorItem, TutorSection};
pub use idea::{move_idea, position_ideas};
pub use idea_view::{IdeaFrame, IdeaMoment, IdeaView, MoveIdea, PositionIdeas};
pub use topic::Topic;
pub use tutor_error::TutorError;
pub use what_if::{WhatIfView, what_if};
