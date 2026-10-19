//! Ciò che succede da solo dopo ogni cambiamento della partita.
//!
//! Dopo una mossa, una mossa ritirata, una nuova partita, ... il ponte deve:
//! chiedere la mossa al computer se tocca a lui, programmare il controllo
//! dell'orologio e far seguire all'analisi la posizione mostrata. Le decisioni
//! ("tocca al computer?", "quando scade il tempo?") le prende la crate
//! `game-session`: qui si eseguono soltanto, senza mai bloccare l'interfaccia.
//! Qui stanno anche il salvataggio delle partite nel database e la ripresa
//! della partita lasciata a metà all'avvio, e l'analisi delle partite in sottofondo.

pub mod after_change;
pub mod computer_move;
pub mod flag_timer;
pub mod follow_analysis;
pub mod maia_move;
pub mod resume_game;
pub mod review_queue;
pub mod review_worker;
pub mod save_worker;
