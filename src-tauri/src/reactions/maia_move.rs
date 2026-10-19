//! La mossa del computer quando il computer è Maia (il computer "umano").

use std::time::Instant;

use game_session::{MaiaRequest, SessionError};
use tauri::{AppHandle, Manager};

use crate::events::engine_error::report_engine_error;
use crate::events::game_updated::emit_game_updated;
use crate::reactions::after_change::after_change;
use crate::state::session_state::SessionState;

/// Chiede la mossa a Maia, la gioca nella partita e avvisa l'interfaccia.
///
/// Il calcolo della rete dura qualche decina di millesimi di secondo: lo si
/// fa su un filo a parte (`spawn_blocking`), così l'app non si blocca. Poi
/// si aspetta il resto della pausa minima, perché la risposta non sembri
/// istantanea. Come per Stockfish, se nel frattempo la partita è cambiata la
/// mossa viene scartata in silenzio.
pub async fn play_maia_move(app: AppHandle, request: MaiaRequest) {
    let started = Instant::now();
    let MaiaRequest {
        token,
        position,
        level,
        min_reply,
    } = request;
    let chosen = tauri::async_runtime::spawn_blocking(move || {
        maia::choose_move(
            level,
            &position.start_fen,
            &position.moves,
            maia::random_roll(),
        )
    })
    .await;
    let uci = match chosen {
        Ok(Ok(uci)) => uci,
        Ok(Err(error)) => {
            report_engine_error(&app, &error.to_string());
            return;
        }
        Err(error) => {
            report_engine_error(&app, &error.to_string());
            return;
        }
    };
    if let Some(rest) = min_reply.checked_sub(started.elapsed()) {
        tokio::time::sleep(rest).await;
    }

    let now = Instant::now();
    let outcome = app.state::<SessionState>().with_session(|session| {
        session
            .play_engine(token, &uci, now)
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
