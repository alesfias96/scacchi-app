//! Il puzzle in corso, custodito in memoria mentre l'app è aperta.

use std::sync::{Mutex, PoisonError};
use std::time::Instant;

use training::PuzzleAttempt;

/// Il puzzle in corso e l'istante in cui è cominciato (per misurare il tempo impiegato).
#[derive(Debug)]
pub struct ActivePuzzle {
    /// Il puzzle e le mosse trovate finora.
    pub attempt: PuzzleAttempt,
    /// Quando è cominciato.
    pub started: Instant,
}

/// Il puzzle in corso (nessuno finché non si apre la sezione "Allenamento").
///
/// Come la partita, è protetto da un `Mutex` ("lucchetto"): due richieste
/// contemporanee non possono lasciarlo a metà.
#[derive(Debug, Default)]
pub struct TrainingState {
    active: Mutex<Option<ActivePuzzle>>,
}

impl TrainingState {
    /// Esegue un'operazione sul puzzle in corso e ne restituisce il risultato.
    pub fn with_active<T>(&self, operation: impl FnOnce(&mut Option<ActivePuzzle>) -> T) -> T {
        let mut active = self.active.lock().unwrap_or_else(PoisonError::into_inner);
        operation(&mut active)
    }
}
