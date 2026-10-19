//! Il "lavoratore" che tiene in funzione un motore ed esegue le richieste.

use std::collections::{BTreeMap, HashMap};

use tokio::io::{AsyncRead, AsyncWrite};
use tokio::sync::{mpsc, oneshot};

use crate::analysis_update::AnalysisUpdate;
use crate::engine_error::EngineError;
use crate::engine_info::EngineInfo;
use crate::engine_job::EngineJob;
use crate::engine_message::EngineMessage;
use crate::search_limits::SearchLimits;
use crate::search_position::SearchPosition;
use crate::uci_command::UciCommand;
use crate::uci_engine::UciEngine;

/// Esegue, una alla volta, le richieste mandate con [`crate::EngineHandle`].
///
/// Lavora in un compito separato (chi lo crea lo avvia con `spawn`) e aspetta
/// contemporaneamente due cose: nuove richieste e righe del motore. Quando
/// arriva una richiesta mentre il motore sta pensando, gli dice `stop`, aspetta
/// la sua risposta (che serve a chiudere la ricerca vecchia) e poi esegue la
/// nuova. Se nel frattempo arrivano altre richieste, conta solo l'ultima.
pub struct EngineWorker<R, W> {
    /// La connessione con il motore.
    engine: UciEngine<R, W>,
    /// Le richieste in arrivo.
    jobs: mpsc::UnboundedReceiver<EngineJob>,
    /// Dove si mandano gli aggiornamenti dell'analisi.
    updates: mpsc::UnboundedSender<AnalysisUpdate>,
    /// Le impostazioni già mandate al motore, per non ripeterle a ogni mossa.
    applied: HashMap<String, String>,
}

/// La ricerca che il motore sta facendo.
struct Search {
    /// Cosa sta cercando.
    purpose: Purpose,
    /// `true` se gli si è già detto `stop` e si aspetta solo la sua risposta finale.
    stopping: bool,
}

/// Lo scopo di una ricerca.
enum Purpose {
    /// Analisi della posizione: gli aggiornamenti vanno all'interfaccia.
    Analysis(SearchPosition),
    /// La mossa da giocare: va consegnata a chi l'ha chiesta.
    Move(oneshot::Sender<Result<String, EngineError>>),
    /// Una valutazione breve: si tiene l'ultimo giudizio e lo si consegna alla fine.
    Evaluate {
        /// La posizione valutata (serve a scrivere la variante).
        position: SearchPosition,
        /// L'ultimo giudizio completo ricevuto.
        latest: Option<AnalysisUpdate>,
        /// Dove consegnarlo.
        reply: oneshot::Sender<Result<AnalysisUpdate, EngineError>>,
    },
    /// Più varianti insieme: si tiene l'ultima di ciascuna e le si consegna alla fine.
    Lines {
        /// La posizione valutata.
        position: SearchPosition,
        /// L'ultima versione di ogni variante, per numero (1 = la migliore).
        latest: BTreeMap<u32, AnalysisUpdate>,
        /// Dove consegnarle.
        reply: oneshot::Sender<Result<Vec<AnalysisUpdate>, EngineError>>,
    },
}

/// L'impostazione di Stockfish che dice quante varianti calcolare insieme.
const MULTI_PV: &str = "MultiPV";

impl<R, W> EngineWorker<R, W>
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    /// Prepara il lavoratore (usato da [`crate::connect`]).
    pub(crate) fn new(
        engine: UciEngine<R, W>,
        jobs: mpsc::UnboundedReceiver<EngineJob>,
        updates: mpsc::UnboundedSender<AnalysisUpdate>,
    ) -> Self {
        Self {
            engine,
            jobs,
            updates,
            applied: HashMap::new(),
        }
    }

    /// Esegue le richieste finché esiste almeno un telecomando; poi chiude il motore.
    ///
    /// # Errors
    ///
    /// [`EngineError::Closed`] o [`EngineError::Io`] se il motore si chiude o
    /// smette di comunicare. Chi aspettava una mossa riceve lo stesso errore.
    pub async fn run(mut self) -> Result<(), EngineError> {
        let mut current: Option<Search> = None;
        let mut pending: Option<EngineJob> = None;
        loop {
            tokio::select! {
                job = self.jobs.recv() => {
                    let Some(job) = job else {
                        // Nessuno può più mandare richieste: si chiude il motore.
                        let _ = self.engine.send(&UciCommand::Quit).await;
                        return Ok(());
                    };
                    match current.as_mut() {
                        None => current = self.start(job).await?,
                        Some(search) => {
                            if !search.stopping {
                                self.engine.send(&UciCommand::Stop).await?;
                                search.stopping = true;
                            }
                            // Una richiesta scavalcata da una più recente non avrà risposta.
                            match pending.take() {
                                Some(EngineJob::FindMove(old)) => {
                                    let _ = old.reply.send(Err(EngineError::Cancelled));
                                }
                                Some(EngineJob::Evaluate(old)) => {
                                    let _ = old.reply.send(Err(EngineError::Cancelled));
                                }
                                Some(EngineJob::EvaluateLines(old)) => {
                                    let _ = old.reply.send(Err(EngineError::Cancelled));
                                }
                                _ => {}
                            }
                            pending = match job {
                                EngineJob::Stop => None,
                                other => Some(other),
                            };
                        }
                    }
                }
                message = self.engine.next_message(), if current.is_some() => {
                    match message? {
                        EngineMessage::Info(info) => self.forward_analysis(current.as_mut(), &info),
                        EngineMessage::BestMove(best) => {
                            if let Some(search) = current.take() {
                                Self::finish(search, best);
                            }
                            if let Some(job) = pending.take() {
                                current = self.start(job).await?;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    /// Manda un'impostazione al motore, solo se è cambiata rispetto all'ultima volta.
    async fn apply(&mut self, name: &str, value: &str) -> Result<(), EngineError> {
        if self.applied.get(name).map(String::as_str) != Some(value) {
            self.engine.set_option(name, value).await?;
            self.applied.insert(name.to_owned(), value.to_owned());
        }
        Ok(())
    }

    /// Comincia a eseguire una richiesta.
    ///
    /// Restituisce la ricerca avviata, oppure `None` per la richiesta `Stop`.
    /// Solo la richiesta di più varianti ne chiede più d'una al motore: tutte
    /// le altre lo riportano a una sola, la migliore.
    async fn start(&mut self, job: EngineJob) -> Result<Option<Search>, EngineError> {
        let wanted_lines = match &job {
            EngineJob::Stop => return Ok(None),
            EngineJob::EvaluateLines(request) => request.lines.max(1),
            _ => 1,
        };
        // Il motore parte con una variante: si cambia solo se serve.
        if wanted_lines != 1 || self.applied.contains_key(MULTI_PV) {
            self.apply(MULTI_PV, &wanted_lines.to_string()).await?;
        }
        let (position, limits, purpose) = match job {
            EngineJob::Stop => return Ok(None),
            EngineJob::Analyze(position) => {
                let purpose = Purpose::Analysis(position.clone());
                (position, SearchLimits::infinite(), purpose)
            }
            EngineJob::FindMove(request) => {
                for (name, value) in &request.options {
                    self.apply(name, value).await?;
                }
                (
                    request.position,
                    request.limits,
                    Purpose::Move(request.reply),
                )
            }
            EngineJob::Evaluate(request) => {
                let purpose = Purpose::Evaluate {
                    position: request.position.clone(),
                    latest: None,
                    reply: request.reply,
                };
                (request.position, request.limits, purpose)
            }
            EngineJob::EvaluateLines(request) => {
                let purpose = Purpose::Lines {
                    position: request.position.clone(),
                    latest: BTreeMap::new(),
                    reply: request.reply,
                };
                (request.position, request.limits, purpose)
            }
        };
        self.engine.send(&UciCommand::Position(position)).await?;
        self.engine.send(&UciCommand::Go(limits)).await?;
        Ok(Some(Search {
            purpose,
            stopping: false,
        }))
    }

    /// Usa una riga di analisi del motore: la manda all'interfaccia (analisi
    /// in tempo reale) oppure la ricorda (valutazione breve).
    fn forward_analysis(&self, current: Option<&mut Search>, info: &EngineInfo) {
        let Some(search) = current else {
            return;
        };
        if search.stopping {
            return;
        }
        match &mut search.purpose {
            Purpose::Analysis(position) => {
                if let Some(update) = AnalysisUpdate::from_info(position, info) {
                    let _ = self.updates.send(update);
                }
            }
            Purpose::Evaluate {
                position, latest, ..
            } => {
                if let Some(update) = AnalysisUpdate::from_info(position, info) {
                    *latest = Some(update);
                }
            }
            Purpose::Lines {
                position, latest, ..
            } => {
                if let Some((number, update)) = AnalysisUpdate::from_any_line(position, info) {
                    latest.insert(number, update);
                }
            }
            Purpose::Move(_) => {}
        }
    }

    /// Chiude una ricerca finita: consegna la mossa o il giudizio a chi l'aveva chiesto.
    fn finish(search: Search, best: Option<String>) {
        match search.purpose {
            Purpose::Move(reply) => {
                let result = if search.stopping {
                    Err(EngineError::Cancelled)
                } else {
                    best.ok_or(EngineError::NoMove)
                };
                let _ = reply.send(result);
            }
            Purpose::Evaluate { latest, reply, .. } => {
                let result = if search.stopping {
                    Err(EngineError::Cancelled)
                } else {
                    latest.ok_or(EngineError::NoMove)
                };
                let _ = reply.send(result);
            }
            Purpose::Lines { latest, reply, .. } => {
                let result = if search.stopping {
                    Err(EngineError::Cancelled)
                } else if latest.is_empty() {
                    Err(EngineError::NoMove)
                } else {
                    Ok(latest.into_values().collect())
                };
                let _ = reply.send(result);
            }
            Purpose::Analysis(_) => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use tokio::sync::mpsc;

    use crate::connect::connect;
    use crate::engine_handle::EngineHandle;
    use crate::fake_engine::FakeEngine;
    use crate::search_limits::SearchLimits;
    use crate::search_position::SearchPosition;

    const START: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    fn start_position() -> SearchPosition {
        SearchPosition {
            start_fen: START.to_owned(),
            moves: Vec::new(),
        }
    }

    /// Avvia il finto motore con il suo lavoratore; restituisce il telecomando,
    /// gli aggiornamenti dell'analisi e il registro dei comandi ricevuti.
    async fn running_engine() -> (
        EngineHandle,
        mpsc::UnboundedReceiver<crate::AnalysisUpdate>,
        std::sync::Arc<std::sync::Mutex<Vec<String>>>,
    ) {
        let fake = FakeEngine::start();
        let (updates, receiver) = mpsc::unbounded_channel();
        let (handle, worker) = connect(fake.reader, fake.writer, &[], updates)
            .await
            .expect("avvio riuscito");
        tokio::spawn(worker.run());
        (handle, receiver, fake.received)
    }

    fn received(log: &std::sync::Arc<std::sync::Mutex<Vec<String>>>) -> Vec<String> {
        log.lock().expect("registro dei comandi").clone()
    }

    #[tokio::test]
    async fn sends_one_update_per_completed_depth() {
        let (handle, mut updates, _log) = running_engine().await;

        handle.analyze(start_position());

        let first = updates.recv().await.expect("primo aggiornamento");
        let second = updates.recv().await.expect("secondo aggiornamento");
        assert_eq!(
            (first.depth, first.line.san),
            (1, vec!["e4".to_owned(), "e5".to_owned()])
        );
        // La riga con il punteggio provvisorio (lowerbound) è stata saltata.
        assert_eq!(
            (second.depth, second.line.san),
            (2, vec!["d4".to_owned(), "d5".to_owned()])
        );
    }

    #[tokio::test]
    async fn finds_a_move_and_sends_the_level_settings_only_once() {
        let (handle, _updates, log) = running_engine().await;
        let options = vec![("Skill Level".to_owned(), "5".to_owned())];

        for _ in 0..2 {
            let found = handle
                .find_move(
                    start_position(),
                    SearchLimits::default(),
                    options.clone(),
                    Duration::ZERO,
                )
                .await
                .expect("mossa trovata");
            assert_eq!(found, "g1f3");
        }

        let settings = received(&log)
            .into_iter()
            .filter(|line| line.starts_with("setoption"))
            .count();
        assert_eq!(settings, 1);
    }

    #[tokio::test]
    async fn a_new_request_stops_the_analysis_first() {
        let (handle, mut updates, log) = running_engine().await;
        handle.analyze(start_position());
        updates.recv().await.expect("l'analisi è partita");

        let found = handle
            .find_move(
                start_position(),
                SearchLimits::default(),
                Vec::new(),
                Duration::ZERO,
            )
            .await
            .expect("mossa trovata");

        assert_eq!(found, "g1f3");
        let commands = received(&log);
        let stop = commands
            .iter()
            .position(|line| line == "stop")
            .expect("stop inviato");
        let go = commands
            .iter()
            .rposition(|line| line == "go")
            .expect("nuova ricerca");
        assert!(stop < go);
    }

    #[tokio::test]
    async fn evaluates_a_position_for_a_short_time() {
        let (handle, _updates, log) = running_engine().await;

        let update = handle
            .evaluate(start_position(), Duration::from_millis(300))
            .await
            .expect("valutazione riuscita");

        assert_eq!(update.evaluation, crate::Evaluation::Centipawns(10));
        assert_eq!(update.line.san, ["Nf3"]);
        assert!(received(&log).contains(&"go movetime 300".to_owned()));
    }

    #[tokio::test]
    async fn evaluates_several_lines_and_then_goes_back_to_one() {
        let (handle, _updates, log) = running_engine().await;

        let lines = handle
            .evaluate_lines(start_position(), Duration::from_millis(300), 3)
            .await
            .expect("varianti");
        handle
            .evaluate(start_position(), Duration::from_millis(300))
            .await
            .expect("valutazione");

        assert_eq!(lines.len(), 2, "il finto motore ne calcola due");
        assert_eq!(lines[0].line.san, ["Nf3"]);
        assert_eq!(lines[1].line.san, ["e4"]);
        let options: Vec<String> = received(&log)
            .into_iter()
            .filter(|line| line.starts_with("setoption name MultiPV"))
            .collect();
        assert_eq!(
            options,
            [
                "setoption name MultiPV value 3",
                "setoption name MultiPV value 1"
            ]
        );
    }

    #[tokio::test]
    async fn waits_at_least_the_minimum_reply_time() {
        let (handle, _updates, _log) = running_engine().await;
        let started = std::time::Instant::now();

        handle
            .find_move(
                start_position(),
                SearchLimits::default(),
                Vec::new(),
                Duration::from_millis(80),
            )
            .await
            .expect("mossa trovata");

        assert!(started.elapsed() >= Duration::from_millis(80));
    }

    #[tokio::test]
    async fn closes_the_engine_when_nobody_can_send_requests() {
        let (handle, _updates, log) = running_engine().await;

        drop(handle);
        tokio::time::sleep(Duration::from_millis(50)).await;

        assert_eq!(received(&log).last().map(String::as_str), Some("quit"));
    }
}
