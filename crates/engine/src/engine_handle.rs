//! Il "telecomando" di un motore in funzione.

use std::time::Duration;

use tokio::sync::{mpsc, oneshot};
use tokio::time::{Instant, sleep_until};

use crate::analysis_update::AnalysisUpdate;
use crate::engine_error::EngineError;
use crate::engine_job::{EngineJob, EvaluateRequest, LinesRequest, MoveRequest};
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

    /// Valuta una posizione per il tempo indicato e restituisce il giudizio finale.
    ///
    /// - Riceve: la posizione e per quanto tempo pensare.
    /// - Restituisce: la valutazione e la variante migliore all'ultima
    ///   profondità completata.
    ///
    /// # Errors
    ///
    /// - [`EngineError::Cancelled`] se nel frattempo è arrivata un'altra richiesta;
    /// - [`EngineError::NoMove`] se la posizione è già finita (matto o stallo);
    /// - [`EngineError::Closed`] se il motore si è chiuso.
    pub async fn evaluate(
        &self,
        position: SearchPosition,
        think_for: Duration,
    ) -> Result<AnalysisUpdate, EngineError> {
        let (reply, answer) = oneshot::channel();
        let limits = SearchLimits {
            move_time_ms: Some(u64::try_from(think_for.as_millis()).unwrap_or(u64::MAX)),
            ..SearchLimits::default()
        };
        self.jobs
            .send(EngineJob::Evaluate(EvaluateRequest {
                position,
                limits,
                reply,
            }))
            .map_err(|_| EngineError::Closed)?;
        answer.await.map_err(|_| EngineError::Closed)?
    }

    /// Valuta una posizione con più varianti insieme (`MultiPV`).
    ///
    /// - Riceve: la posizione, per quanto tempo pensare e quante varianti volere.
    /// - Restituisce: le varianti, dalla migliore, ciascuna con la sua
    ///   valutazione (possono essere meno di quelle chieste, se le mosse sono poche).
    ///
    /// # Errors
    ///
    /// Gli stessi di [`EngineHandle::evaluate`].
    pub async fn evaluate_lines(
        &self,
        position: SearchPosition,
        think_for: Duration,
        lines: u32,
    ) -> Result<Vec<AnalysisUpdate>, EngineError> {
        let (reply, answer) = oneshot::channel();
        let limits = SearchLimits {
            move_time_ms: Some(u64::try_from(think_for.as_millis()).unwrap_or(u64::MAX)),
            ..SearchLimits::default()
        };
        self.jobs
            .send(EngineJob::EvaluateLines(LinesRequest {
                position,
                limits,
                lines,
                reply,
            }))
            .map_err(|_| EngineError::Closed)?;
        answer.await.map_err(|_| EngineError::Closed)?
    }
}
