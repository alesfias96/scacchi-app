//! Il "telecomando" di un motore in funzione.

use std::time::Duration;

use tokio::sync::{mpsc, oneshot};
use tokio::time::{Instant, sleep_until};

use crate::engine_error::EngineError;
use crate::engine_job::{EngineJob, MoveRequest};
use crate::search_limits::SearchLimits;
use crate::search_position::SearchPosition;

/// Manda richieste a un motore che lavora in un compito separato
/// (vedi [`crate::EngineWorker`]).
///
/// Si può copiare liberamente: tutte le copie parlano allo stesso motore.
/// Le richieste non aspettano mai il motore: vengono messe in coda e il
/// motore le esegue appena può, interrompendo quella in corso.
#[derive(Debug, Clone)]
pub struct EngineHandle {
    /// La coda delle richieste.
    jobs: mpsc::UnboundedSender<EngineJob>,
}

impl EngineHandle {
    /// Crea il telecomando e la coda da cui il motore leggerà le richieste.
    pub(crate) fn channel() -> (Self, mpsc::UnboundedReceiver<EngineJob>) {
        let (jobs, receiver) = mpsc::unbounded_channel();
        (Self { jobs }, receiver)
    }

    /// Comincia ad analizzare una posizione, senza limite di tempo.
    /// Gli aggiornamenti arrivano sul canale indicato all'avvio del motore.
    pub fn analyze(&self, position: SearchPosition) {
        // Se il motore si è chiuso la richiesta va persa: non c'è nulla da fare.
        let _ = self.jobs.send(EngineJob::Analyze(position));
    }

    /// Ferma la ricerca in corso.
    pub fn stop(&self) {
        let _ = self.jobs.send(EngineJob::Stop);
    }

    /// Chiede al motore la mossa da giocare e la aspetta.
    ///
    /// - Riceve: la posizione, i limiti di tempo, le impostazioni del livello
    ///   e il tempo minimo di risposta. Quest'ultimo evita che il computer
    ///   risponda "all'istante", cosa che a chi gioca sembra innaturale.
    /// - Restituisce: la mossa in notazione UCI.
    ///
    /// # Errors
    ///
    /// - [`EngineError::Cancelled`] se nel frattempo è arrivata un'altra richiesta;
    /// - [`EngineError::NoMove`] se nella posizione non ci sono mosse;
    /// - [`EngineError::Closed`] se il motore si è chiuso.
    pub async fn find_move(
        &self,
        position: SearchPosition,
        limits: SearchLimits,
        options: Vec<(String, String)>,
        min_reply: Duration,
    ) -> Result<String, EngineError> {
        let earliest = Instant::now() + min_reply;
        let (reply, answer) = oneshot::channel();
        self.jobs
            .send(EngineJob::FindMove(MoveRequest {
                position,
                limits,
                options,
                reply,
            }))
            .map_err(|_| EngineError::Closed)?;
        let found = answer.await.map_err(|_| EngineError::Closed)??;
        sleep_until(earliest).await;
        Ok(found)
    }
}
