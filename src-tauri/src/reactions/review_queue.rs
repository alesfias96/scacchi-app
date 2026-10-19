//! La coda delle partite da analizzare («Analizza le partite mancanti»).
//!
//! Le partite finite non ancora analizzate si analizzano una alla volta, dalla
//! più vecchia. Fermare l'analisi in corso ferma anche la coda.

use review::ReviewQueueProgress;
use tauri::{AppHandle, Manager};

use crate::events::review::emit_review_queue;
use crate::reactions::review_worker::run_review;
use crate::state::review_state::ReviewState;

/// Analizza in sottofondo, una alla volta, le partite indicate.
pub async fn run_review_queue(app: AppHandle, games: Vec<String>) {
    let total = u32::try_from(games.len()).unwrap_or(u32::MAX);
    let mut done = 0;
    for game_id in games {
        emit_review_queue(
            &app,
            &ReviewQueueProgress {
                done,
                total,
                finished: false,
            },
        );
        // Se nel frattempo è partita un'altra analisi, la coda si ferma.
        let Some(generation) = app.state::<ReviewState>().try_start(&game_id) else {
            break;
        };
        if !run_review(app.clone(), game_id, generation).await {
            break;
        }
        done += 1;
    }
    emit_review_queue(
        &app,
        &ReviewQueueProgress {
            done,
            total,
            finished: true,
        },
    );
}
