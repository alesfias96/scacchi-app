//! # chess-core — le regole degli scacchi
//!
//! Questa crate è la fonte di verità sulle regole: sa costruire una posizione
//! (quella iniziale o una descritta in notazione FEN), calcolare le mosse
//! legali e capire se il re è sotto scacco.
//!
//! Il lavoro "pesante" sulle regole è affidato alla libreria `shakmaty`;
//! questa crate la avvolge ed espone solo tipi semplici e stabili,
//! pensati per essere mostrati dall'interfaccia ([`PositionView`]).
//!
//! Organizzazione (un file per responsabilità):
//! - [`side`]: il colore che deve muovere;
//! - [`legal_move`]: una mossa legale, con casa di partenza e di arrivo;
//! - [`position_view`]: la descrizione completa di una posizione;
//! - [`setup`]: come si crea una posizione (iniziale o da FEN);
//! - [`setup_error`]: gli errori possibili durante la creazione;
//! - [`game`]: la partita (mosse, navigazione, fine partita);
//! - [`move_record`]: una mossa giocata, con la sua notazione;
//! - [`game_view`]: la descrizione completa della partita;
//! - [`game_result`]: il risultato e il motivo della fine della partita;
//! - [`game_error`]: gli errori possibili durante la partita;
//! - [`pgn_export`] e [`pgn_import`]: scrittura e lettura del formato PGN;
//! - [`game_import`]: importazione da testo (riconosce FEN o PGN).

pub mod game;
pub mod game_error;
pub mod game_import;
pub mod game_result;
pub mod game_view;
pub mod legal_move;
pub mod move_record;
pub mod pgn_export;
pub mod pgn_import;
pub mod position_view;
pub mod setup;
pub mod setup_error;
pub mod side;

pub use game::Game;
pub use game_error::GameError;
pub use game_import::import_game;
pub use game_result::{GameEndReason, GameResult};
pub use game_view::GameView;
pub use legal_move::LegalMove;
pub use move_record::MoveRecord;
pub use pgn_export::to_pgn;
pub use pgn_import::from_pgn;
pub use position_view::PositionView;
pub use setup::{initial_position, position_from_fen};
pub use setup_error::SetupError;
pub use side::Side;
