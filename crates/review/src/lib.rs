//! # review — l'analisi delle partite giocate
//!
//! Riceve una partita e il giudizio di Stockfish su ogni sua posizione (lo
//! chiede il ponte `src-tauri`) e ne ricava:
//! - il **giudizio di ogni mossa** (ottima, buona, imprecisione, errore,
//!   errore grave), dalla probabilità di vittoria persa come fa Lichess;
//! - la **precisione** e gli errori di ciascun giocatore;
//! - i **momenti critici**, spiegati con i fatti dei rilevatori (cosa
//!   concedeva la mossa giocata, cosa otteneva quella migliore);
//! - il **grafico** della probabilità di vittoria;
//! - i **dati da salvare** per le statistiche e i **puzzle** dagli errori gravi.
//!
//! Organizzazione (un file per responsabilità):
//! - [`review_input`]: cosa serve per analizzare (mosse e valutazioni);
//! - [`replay`]: rigioca la partita e ne raccoglie le posizioni;
//! - [`win_chance`]: probabilità di vittoria e precisione di una mossa;
//! - [`move_quality`]: il giudizio di una mossa;
//! - [`game_reviewer`]: la revisione completa;
//! - [`review_view`]: la forma della revisione per l'interfaccia;
//! - [`move_analysis`]: i dati di ogni mossa da salvare;
//! - [`own_puzzle`]: i puzzle dagli errori gravi;
//! - [`review_progress`]: i messaggi sull'analisi in corso;
//! - [`review_error`]: gli errori possibili.

pub mod game_reviewer;
pub mod move_analysis;
pub mod move_quality;
pub mod own_puzzle;
pub mod replay;
pub mod review_error;
pub mod review_input;
pub mod review_progress;
pub mod review_view;
pub mod win_chance;

#[cfg(test)]
pub(crate) mod test_game;

pub use game_reviewer::review_game;
pub use move_analysis::{MoveAnalysis, move_analysis, phase_code};
pub use move_quality::MoveQuality;
pub use own_puzzle::{OwnPuzzle, own_puzzles};
pub use review_error::ReviewError;
pub use review_input::{PositionEval, ReviewInput};
pub use review_progress::{ReviewFinished, ReviewProgress};
pub use review_view::{CriticalMoment, GameReview, MoveReview, PlayerSummary};
