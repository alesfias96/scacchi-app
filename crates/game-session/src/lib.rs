//! # game-session — la partita in corso
//!
//! La partita di `chess-core` conosce le regole degli scacchi. Questa crate
//! aggiunge ciò che serve per **giocare davvero**:
//! - l'avversario: un'altra persona sulla stessa scacchiera, oppure il computer
//!   a uno dei dieci livelli;
//! - l'orologio, con il tempo che si consuma e l'incremento a ogni mossa;
//! - le regole di comportamento: chi può muovere, quando si possono ritirare
//!   le mosse, quando la partita finisce per tempo o per abbandono;
//! - le richieste da mandare al motore quando tocca al computer ([`EngineRequest`]).
//!
//! Non avvia il motore e non conosce Tauri: il ponte `src-tauri` manda le
//! richieste al motore e riporta qui le sue mosse.
//!
//! Organizzazione (un file per responsabilità):
//! - [`game_session`]: la partita in corso;
//! - [`clock`] e [`clock_view`]: l'orologio e come mostrarlo;
//! - [`computer_view`]: il computer avversario come mostrarlo;
//! - [`session_view`]: tutto ciò che l'interfaccia mostra;
//! - [`engine_request`]: la richiesta di una mossa al motore;
//! - [`move_timer`]: il tempo impiegato per ogni mossa;
//! - [`session_snapshot`]: la "fotografia" da salvare nell'archivio e da cui ripartire;
//! - [`session_id`]: il nome unico di ogni partita e la sua data;
//! - [`coin_flip`]: il sorteggio del colore;
//! - [`session_error`]: gli errori possibili.

pub mod clock;
pub mod clock_view;
pub mod coin_flip;
pub mod computer_view;
pub mod engine_request;
pub mod game_session;
pub mod move_timer;
pub mod session_error;
pub mod session_id;
pub mod session_snapshot;
pub mod session_view;

pub use clock::Clock;
pub use clock_view::ClockView;
pub use coin_flip::coin_flip;
pub use computer_view::ComputerView;
pub use engine_request::EngineRequest;
pub use game_session::GameSession;
pub use move_timer::MoveTiming;
pub use session_error::SessionError;
pub use session_id::now_unix_ms;
pub use session_snapshot::{MoveSnapshot, SessionSnapshot};
pub use session_view::SessionView;
