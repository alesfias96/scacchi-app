//! La partita in corso, custodita in memoria mentre l'app è aperta.

use std::sync::{Mutex, PoisonError};

use chess_core::Game;

/// La partita in corso.
///
/// È protetta da un `Mutex` ("lucchetto"): se due richieste arrivano insieme,
/// la seconda aspetta che la prima abbia finito, così la partita non può mai
/// trovarsi in uno stato a metà.
#[derive(Debug, Default)]
pub struct GameState {
    game: Mutex<Game>,
}

impl GameState {
    /// Esegue un'operazione sulla partita e ne restituisce il risultato.
    ///
    /// - Riceve: l'operazione da eseguire, che può leggere e modificare la partita.
    /// - Restituisce: ciò che restituisce l'operazione.
    ///
    /// Se un'operazione precedente si fosse interrotta a metà (caso che non
    /// dovrebbe mai accadere), la partita resta comunque utilizzabile invece
    /// di bloccare l'app.
    pub fn with_game<T>(&self, operation: impl FnOnce(&mut Game) -> T) -> T {
        let mut game = self.game.lock().unwrap_or_else(PoisonError::into_inner);
        operation(&mut game)
    }
}

#[cfg(test)]
mod tests {
    use super::GameState;

    #[test]
    fn keeps_the_game_between_operations() {
        let state = GameState::default();

        state
            .with_game(|game| game.play_uci("e2e4"))
            .expect("mossa legale");

        assert_eq!(state.with_game(|game| game.moves().len()), 1);
    }
}
