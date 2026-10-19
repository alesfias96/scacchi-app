//! La richiesta di una mossa a Maia, quando il computer è "umano".

use std::time::Duration;

use engine::SearchPosition;

/// Tutto ciò che serve per chiedere a Maia la mossa del computer.
///
/// Come per Stockfish (vedi [`crate::engine_request::EngineRequest`]), il
/// `token` dice in quale stato della partita è nata la richiesta: una mossa
/// arrivata quando la partita è già cambiata viene scartata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaiaRequest {
    /// Il contrassegno dello stato della partita.
    pub token: u64,
    /// La partita fino alla posizione in cui il computer deve muovere
    /// (a Maia servono anche le posizioni precedenti, non solo l'ultima).
    pub position: SearchPosition,
    /// Il livello di Maia, da 1 (1100 Elo) a 9 (1900 Elo).
    pub level: u8,
    /// Il tempo minimo prima di rispondere: Maia sceglie in pochi millesimi
    /// di secondo, ma una risposta istantanea sembrerebbe innaturale.
    pub min_reply: Duration,
}
