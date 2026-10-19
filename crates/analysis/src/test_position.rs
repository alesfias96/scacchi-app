//! Aiuti per i test: posizioni da FEN e ricerca dei fatti.

use shakmaty::fen::Fen;
use shakmaty::{CastlingMode, Chess};

use crate::fact::Fact;

/// La posizione descritta dalla FEN (che nei test è sempre valida).
pub(crate) fn position(fen: &str) -> Chess {
    fen.parse::<Fen>()
        .expect("FEN valida")
        .into_position(CastlingMode::Standard)
        .expect("posizione legale")
}

/// I testi dei fatti, per controllarli comodamente.
pub(crate) fn texts(facts: &[Fact]) -> Vec<String> {
    facts.iter().map(|fact| fact.text.clone()).collect()
}
