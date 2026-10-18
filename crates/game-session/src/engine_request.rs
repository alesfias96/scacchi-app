//! La richiesta di una mossa da mandare al motore.

use std::time::Duration;

use engine::{SearchLimits, SearchPosition};

/// Tutto ciò che serve per chiedere al motore la mossa del computer.
///
/// Il `token` identifica lo stato della partita in cui la richiesta è
/// nata: quando la mossa arriva, la partita la accetta solo se nel frattempo
/// non è cambiato nulla (nessuna mossa ritirata, nessuna nuova partita).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EngineRequest {
    /// Il contrassegno dello stato della partita.
    pub token: u64,
    /// La posizione in cui il computer deve muovere.
    pub position: SearchPosition,
    /// Per quanto tempo pensare.
    pub limits: SearchLimits,
    /// Le impostazioni del livello di gioco.
    pub options: Vec<(String, String)>,
    /// Il tempo minimo prima di rispondere, perché il computer non sembri "istantaneo".
    pub min_reply: Duration,
}
