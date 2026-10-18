//! Il controllo dell'orologio nell'istante in cui scadrebbe il tempo.

use std::time::{Duration, Instant};

use tauri::{AppHandle, Manager};

use crate::events::game_updated::emit_game_updated;
use crate::reactions::after_change::after_change;
use crate::state::session_state::SessionState;

/// Un piccolo margine dopo la scadenza, perché il controllo trovi il tempo
/// sicuramente finito.
const MARGIN: Duration = Duration::from_millis(20);

/// Programma il controllo dell'orologio fra `delay` (se l'orologio è in moto).
///
/// Ogni nuovo controllo annulla quelli programmati prima: dopo una mossa
/// conta solo la nuova scadenza. Se al momento del controllo il tempo è
/// davvero finito, la partita finisce e l'interfaccia viene avvisata.
pub fn schedule_flag_check(app: &AppHandle, delay: Option<Duration>) {
    let check = app.state::<SessionState>().next_flag_check();
    let Some(delay) = delay else {
        return;
    };
    let app = app.clone();
    tauri::async_runtime::spawn(async move {
        tokio::time::sleep(delay + MARGIN).await;
        let state = app.state::<SessionState>();
        if !state.is_current_flag_check(check) {
            return;
        }
        let now = Instant::now();
        let ended =
            state.with_session(|session| session.check_time(now).then(|| session.view(now)));
        if let Some(view) = ended {
            emit_game_updated(&app, &view);
            after_change(&app);
        }
    });
}
