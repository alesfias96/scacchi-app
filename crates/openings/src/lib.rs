//! # openings — il riconoscimento delle aperture
//!
//! Le prime mosse di una partita hanno un nome, raccolto nei libri: "Difesa
//! Siciliana, variante Najdorf", codice ECO `B90`. Questa crate contiene
//! l'elenco delle aperture di Lichess (circa 3.900, licenza CC0, nella cartella
//! `data/openings/`) e riconosce quale è stata giocata in una partita
//! ([`identify`]), anche se ci si è arrivati con un ordine di mosse diverso.
//!
//! Serve all'archivio delle partite (filtri per apertura) e alle statistiche
//! sulle aperture giocate col Bianco e col Nero.
//!
//! Organizzazione (un file per responsabilità):
//! - [`opening`]: l'apertura (codice ECO e nome);
//! - [`catalog`]: l'elenco, pronto per la ricerca per posizione;
//! - [`identify`]: il riconoscimento dell'apertura di una partita.

pub mod catalog;
pub mod identify;
pub mod opening;

pub use identify::identify;
pub use opening::Opening;
