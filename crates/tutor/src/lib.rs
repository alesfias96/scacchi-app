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
pub mod imbalances;
pub mod knowledge_card;
pub mod library;
pub mod line_walk;
pub mod middlegame_themes;
pub mod motifs;
pub mod move_lesson;
pub mod opponent_paths;
pub mod principles;
pub mod recognition;
pub mod strategy;
pub mod strategy_plans;
pub mod strategy_view;
pub mod structures;
pub mod thinking;
pub mod topic;
pub mod tutor_error;
pub mod what_if;

pub use explanation::explain;
pub use explanation_view::{
    CardView, PlanView, ThinkingStep, ThinkingView, TutorExplanation, TutorItem, TutorSection,
};
pub use idea::{move_idea, position_ideas, solution_idea};
pub use idea_view::{
    IdeaFrame, IdeaMoment, IdeaView, MotifView, MoveIdea, OpponentPaths, PositionIdeas,
};
pub use motifs::{Motif, fact_motif, motif_view};
pub use opponent_paths::{flipped_fen, opponent_paths, paths_when_in_check};
pub use strategy_view::{ImbalanceItem, StrategyPlanView, StrategyView};
pub use topic::Topic;
pub use tutor_error::TutorError;
pub use what_if::{WhatIfView, what_if};
