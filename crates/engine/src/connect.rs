//! Avvio di un motore: il programma vero oppure un "tubo" qualsiasi.

use std::future::Future;
use std::path::Path;
use std::process::Stdio;

use tokio::io::{AsyncRead, AsyncWrite};
use tokio::process::Command;
use tokio::sync::mpsc;

use crate::analysis_update::AnalysisUpdate;
use crate::engine_error::EngineError;
use crate::engine_handle::EngineHandle;
use crate::engine_worker::EngineWorker;
use crate::uci_engine::UciEngine;

/// Su Windows impedisce che, avviando il motore, si apra una finestra nera
/// del terminale (`CREATE_NO_WINDOW`).
#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

/// Avvia il programma del motore e lo prepara.
///
/// - Riceve: il percorso del programma (es. Stockfish), le impostazioni da
///   applicare subito e il canale su cui mandare gli aggiornamenti dell'analisi.
/// - Restituisce: il telecomando del motore e il lavoro da avviare in un
///   compito separato (con `spawn`). Il programma del motore si chiude da
///   solo quando quel lavoro finisce.
///
/// # Errors
///
/// - [`EngineError::Launch`] se il programma non si avvia;
/// - gli errori di [`UciEngine::start`] se il motore non risponde come dovrebbe.
pub async fn launch(
    path: &Path,
    settings: &[(String, String)],
    updates: mpsc::UnboundedSender<AnalysisUpdate>,
) -> Result<
    (
        EngineHandle,
        impl Future<Output = Result<(), EngineError>> + Send + 'static,
    ),
    EngineError,
> {
    let mut command = Command::new(path);
    command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .kill_on_drop(true);
    #[cfg(windows)]
    command.creation_flags(CREATE_NO_WINDOW);

    let mut child = command
        .spawn()
        .map_err(|error| EngineError::Launch(error.to_string()))?;
    let (Some(output), Some(input)) = (child.stdout.take(), child.stdin.take()) else {
        return Err(EngineError::Launch(
            "canali del programma non disponibili".to_owned(),
        ));
    };
    let (handle, worker) = connect(output, input, settings, updates).await?;
    let work = async move {
        // Il programma resta aperto finché esiste `child`, cioè finché il lavoratore lavora.
        let _child = child;
        worker.run().await
    };
    Ok((handle, work))
}

/// Collega un motore attraverso un tubo qualsiasi (usato da [`launch`] e dai test).
///
/// # Errors
///
/// Gli stessi di [`UciEngine::start`].
pub async fn connect<R, W>(
    reader: R,
    writer: W,
    settings: &[(String, String)],
    updates: mpsc::UnboundedSender<AnalysisUpdate>,
) -> Result<(EngineHandle, EngineWorker<R, W>), EngineError>
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    let engine = UciEngine::start(reader, writer, settings).await?;
    let (handle, jobs) = EngineHandle::channel();
    Ok((handle, EngineWorker::new(engine, jobs, updates)))
}
