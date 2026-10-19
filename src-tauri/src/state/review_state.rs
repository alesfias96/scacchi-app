//! L'analisi di una partita in corso (una alla volta).

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, PoisonError};

/// Quale partita si sta analizzando, e il "numero" dell'analisi.
///
/// Fermare un'analisi cambia il numero: il lavoro in sottofondo se ne accorge
/// alla posizione successiva e si ferma (vedi `reactions::review_worker`).
#[derive(Debug, Default)]
pub struct ReviewState {
    /// La partita in analisi, se c'è.
    running: Mutex<Option<String>>,
    /// Il numero dell'analisi valida.
    generation: AtomicU64,
}

impl ReviewState {
    /// Prova a cominciare l'analisi di una partita.
    ///
    /// Restituisce il numero dell'analisi, oppure `None` se un'altra è già in corso.
    pub fn try_start(&self, game_id: &str) -> Option<u64> {
        let mut running = self.running.lock().unwrap_or_else(PoisonError::into_inner);
        if running.is_some() {
            return None;
        }
        *running = Some(game_id.to_owned());
        Some(self.generation.fetch_add(1, Ordering::Relaxed) + 1)
    }

    /// La partita in analisi, se c'è.
    pub fn running(&self) -> Option<String> {
        self.running
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }

    /// Ferma l'analisi in corso (alla posizione successiva).
    pub fn cancel(&self) {
        self.generation.fetch_add(1, Ordering::Relaxed);
    }

    /// `true` se l'analisi con questo numero deve continuare.
    pub fn is_current(&self, generation: u64) -> bool {
        self.generation.load(Ordering::Relaxed) == generation
    }

    /// L'analisi è finita (o si è fermata): se ne può cominciare un'altra.
    pub fn finish(&self) {
        *self.running.lock().unwrap_or_else(PoisonError::into_inner) = None;
    }
}

#[cfg(test)]
mod tests {
    use super::ReviewState;

    #[test]
    fn one_review_at_a_time_and_cancellable() {
        let state = ReviewState::default();

        let generation = state.try_start("partita-1").expect("nessuna in corso");
        assert_eq!(state.try_start("partita-2"), None);
        assert_eq!(state.running().as_deref(), Some("partita-1"));
        assert!(state.is_current(generation));

        state.cancel();
        assert!(!state.is_current(generation));

        state.finish();
        assert!(state.try_start("partita-2").is_some());
    }
}
