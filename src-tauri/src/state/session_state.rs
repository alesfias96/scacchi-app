//! La partita in corso, custodita in memoria mentre l'app è aperta.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, PoisonError};

use game_session::GameSession;

/// La partita in corso (con avversario e orologio).
///
/// È protetta da un `Mutex` ("lucchetto"): se due richieste arrivano insieme
/// (ad esempio una mossa della persona e la risposta del computer), la seconda
/// aspetta che la prima abbia finito, così la partita non può mai trovarsi in
/// uno stato a metà.
#[derive(Debug, Default)]
pub struct SessionState {
    /// La partita.
    session: Mutex<GameSession>,
    /// Il numero dell'ultimo controllo dell'orologio programmato: un controllo
    /// vecchio, superato da uno più recente, non fa nulla (vedi `reactions::flag_timer`).
    flag_check: AtomicU64,
    /// Il contrassegno dell'ultima richiesta di mossa già mandata al computer:
    /// la stessa richiesta non si manda due volte (vedi [`SessionState::claim_engine_request`]).
    requested_token: AtomicU64,
}

impl SessionState {
    /// Esegue un'operazione sulla partita e ne restituisce il risultato.
    ///
    /// - Riceve: l'operazione da eseguire, che può leggere e modificare la partita.
    /// - Restituisce: ciò che restituisce l'operazione.
    ///
    /// Se un'operazione precedente si fosse interrotta a metà (caso che non
    /// dovrebbe mai accadere), la partita resta comunque utilizzabile invece
    /// di bloccare l'app.
    pub fn with_session<T>(&self, operation: impl FnOnce(&mut GameSession) -> T) -> T {
        let mut session = self.session.lock().unwrap_or_else(PoisonError::into_inner);
        operation(&mut session)
    }

    /// Prenota un nuovo controllo dell'orologio e ne restituisce il numero.
    /// Da questo momento i controlli prenotati prima non valgono più.
    pub fn next_flag_check(&self) -> u64 {
        self.flag_check.fetch_add(1, Ordering::Relaxed) + 1
    }

    /// Segna come inviata la richiesta di mossa con il contrassegno `token`.
    ///
    /// Restituisce `true` se è la prima volta (la richiesta va mandata), `false`
    /// se era già stata mandata: ad esempio quando l'utente scorre le mosse
    /// mentre il computer pensa, la partita non cambia e il computer non deve
    /// ricominciare a pensare da capo.
    pub fn claim_engine_request(&self, token: u64) -> bool {
        self.requested_token.swap(token, Ordering::Relaxed) != token
    }

    /// `true` se `check` è ancora l'ultimo controllo dell'orologio prenotato.
    pub fn is_current_flag_check(&self, check: u64) -> bool {
        self.flag_check.load(Ordering::Relaxed) == check
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::SessionState;

    #[test]
    fn keeps_the_game_between_operations() {
        let state = SessionState::default();

        state
            .with_session(|session| session.play("e2e4", Instant::now()))
            .expect("mossa legale");

        let moves = state.with_session(|session| session.view(Instant::now()).game.moves.len());
        assert_eq!(moves, 1);
    }

    #[test]
    fn the_same_engine_request_is_sent_only_once() {
        let state = SessionState::default();

        assert!(state.claim_engine_request(7));
        assert!(!state.claim_engine_request(7));
        assert!(state.claim_engine_request(8));
    }

    #[test]
    fn only_the_latest_clock_check_is_valid() {
        let state = SessionState::default();

        let first = state.next_flag_check();
        let second = state.next_flag_check();

        assert!(!state.is_current_flag_check(first));
        assert!(state.is_current_flag_check(second));
    }
}
