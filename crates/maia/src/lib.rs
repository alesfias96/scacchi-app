//! # maia — l'avversario "umano"
//!
//! Maia è una rete neurale addestrata su milioni di partite di persone vere:
//! data una posizione, dice quanto è probabile ogni mossa per una persona di un
//! certo livello (da 1100 a 1900 Elo). Gioca come una persona di quel livello,
//! errori tipici compresi, invece di indebolirsi a caso come Stockfish ai livelli bassi.
//!
//! Le reti sono quelle originali del progetto Maia (formato di Lc0, licenza
//! GPL-3.0). Il loro calcolo lo fa questa crate, in Rust semplice: la rete è
//! piccola (6 blocchi da 64 canali) e per scegliere una mossa basta un solo
//! calcolo, senza ricerca.
//!
//! I punti d'ingresso sono [`choose_move`] e [`move_probabilities`].
//!
//! Organizzazione (un file per responsabilità):
//! - [`proto_reader`]: la lettura del formato dei file delle reti;
//! - [`net_weights`]: i pesi della rete;
//! - [`network`]: il calcolo della rete;
//! - [`input_planes`]: la posizione codificata come la vuole la rete;
//! - [`move_index`]: il posto di ogni mossa tra quelle che la rete conosce;
//! - [`policy_tables`]: le tabelle delle mosse di Lc0;
//! - [`move_choice`]: dalle probabilità alla mossa da giocare;
//! - [`maia_level`]: i nove livelli;
//! - [`maia_player`]: tutto insieme;
//! - [`maia_error`]: gli errori possibili.

pub mod input_planes;
pub mod maia_error;
pub mod maia_level;
pub mod maia_player;
pub mod move_choice;
pub mod move_index;
pub mod net_weights;
pub mod network;
pub mod policy_tables;
pub mod proto_reader;

pub use maia_error::MaiaError;
pub use maia_level::{DEFAULT_LEVEL, MAX_LEVEL, MIN_LEVEL, level_elo, level_info, levels};
pub use maia_player::{choose_move, move_probabilities};
pub use move_choice::{MoveProbability, random_roll};
