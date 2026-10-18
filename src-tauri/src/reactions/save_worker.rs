//! Il lavoratore che salva le partite nel database, una alla volta.

use game_session::SessionSnapshot;
use tauri::{AppHandle, Manager};
use tokio::sync::mpsc;

use crate::events::storage_error::report_storage_error;
use crate::state::database_state::DatabaseState;

/// Svuota la coda dei salvataggi finché l'app è aperta.
///
/// Se nel frattempo si sono accumulate più versioni della stessa partita
/// (mosse molto veloci), salva solo l'ultima: le precedenti sono superate.
pub async fn run_save_worker(app: AppHandle, mut queue: mpsc::UnboundedReceiver<SessionSnapshot>) {
    while let Some(first) = queue.recv().await {
        let mut pending = vec![first];
        while let Ok(next) = queue.try_recv() {
            pending.push(next);
        }
        for snapshot in latest_per_game(pending) {
            let state = app.state::<DatabaseState>();
            let outcome = match state.database().await {
                Ok(database) => database
                    .save(&snapshot)
                    .await
                    .map_err(|error| error.to_string()),
                Err(message) => Err(message),
            };
            if let Err(message) = outcome {
                report_storage_error(&app, &message);
            }
        }
    }
}

/// Tiene, per ogni partita, solo l'ultima versione in coda (nell'ordine di arrivo).
fn latest_per_game(pending: Vec<SessionSnapshot>) -> Vec<SessionSnapshot> {
    let mut latest: Vec<SessionSnapshot> = Vec::new();
    for snapshot in pending {
        latest.retain(|kept| kept.id != snapshot.id);
        latest.push(snapshot);
    }
    latest
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use game_session::GameSession;

    use super::latest_per_game;

    #[test]
    fn keeps_only_the_newest_version_of_each_game() {
        let now = Instant::now();
        let mut first = GameSession::default();
        let other = GameSession::default();
        let old = first.snapshot(now);
        first.play("e2e4", now).expect("mossa legale");
        let new = first.snapshot(now);

        let kept = latest_per_game(vec![old, other.snapshot(now), new.clone()]);

        assert_eq!(kept.len(), 2);
        assert_eq!(kept[1], new);
    }
}
