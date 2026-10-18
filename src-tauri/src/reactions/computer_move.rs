//! La mossa del computer.

use std::time::Instant;

use engine::EngineError;
use game_session::{EngineRequest, SessionError};
use tauri::{AppHandle, Manager};

use crate::events::engine_error::report_engine_error;
use crate::events::game_updated::emit_game_updated;
use crate::reactions::after_change::after_change;
use crate::state::engines_state::EnginesState;
use crate::state::session_state::SessionState;

/// Chiede la mossa al motore che gioca, la gioca nella partita e avvisa l'interfaccia.
///
/// Se nel frattempo la partita è cambiata (mossa ritirata, nuova partita) la
/// mossa viene scartata in silenzio: è la partita stessa a riconoscerla come
/// vecchia, grazie al contrassegno della richiesta.
pub async fn play_computer_move(app: AppHandle, request: EngineRequest) {
    let player = match app.state::<EnginesState>().player(&app).await {
        Ok(player) => player,
        Err(message) => {
            report_engine_error(&app, &message);
            return;
        }
    };
    let found = player
        .find_move(
            request.position,
            request.limits,
            request.options,
            request.min_reply,
        )
        .await;
    let uci = match found {
        Ok(uci) => uci,
        // Annullata da una richiesta più recente: non è un errore.
        Err(EngineError::Cancelled) => return,
        Err(error) => {
            report_engine_error(&app, &error.to_string());
            return;
        }
    };

    let now = Instant::now();
    let outcome = app.state::<SessionState>().with_session(|session| {
        session
            .play_engine(request.token, &uci, now)
            .map(|()| session.view(now))
    });
    match outcome {
        Ok(view) => {
            emit_game_updated(&app, &view);
            after_change(&app);
        }
        Err(SessionError::StaleEngineMove) => {}
        Err(error) => report_engine_error(&app, &error.to_string()),
    }
}
