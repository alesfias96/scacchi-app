//! # analysis — i rilevatori
//!
//! Guarda una posizione e trova i **fatti verificabili**: cose vere che un
//! buon giocatore vede guardando la scacchiera. Sono il "livello 1" del tutor
//! (`PROGETTO.md` §4.2): non danno giudizi su chi sta meglio (quello lo fa
//! Stockfish), ma dicono cosa c'è.
//!
//! Il punto d'ingresso è [`analyze`], che restituisce un [`PositionReport`].
//!
//! Organizzazione (un file per responsabilità):
//! - [`report`]: il rapporto completo e la funzione che chiama tutti i rilevatori;
//! - [`fact`]: un fatto trovato, con le case da evidenziare;
//! - [`piece_value`], [`exchange`]: valore dei pezzi e scambio statico;
//! - [`wording`]: le parole italiane per pezzi e colori;
//! - i rilevatori: [`material`], [`game_phase`], [`hanging`], [`mate_threats`],
//!   [`forks`], [`lines`] (inchiodature, infilate, scoperte), [`overload`],
//!   [`pawn_structure`], [`king_safety`], [`activity`].

pub mod activity;
pub mod exchange;
pub mod fact;
pub mod forks;
pub mod game_phase;
pub mod hanging;
pub mod king_safety;
pub mod lines;
pub mod mate_threats;
pub mod material;
pub mod overload;
pub mod pawn_structure;
pub mod piece_value;
pub mod report;
pub mod wording;

#[cfg(test)]
pub(crate) mod test_position;

pub use fact::{Fact, FactArrow, FactCategory};
pub use game_phase::GamePhase;
pub use material::MaterialBalance;
pub use report::{PositionReport, analyze};
