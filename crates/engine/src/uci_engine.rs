//! La connessione con un motore che parla il protocollo UCI.

use std::time::Duration;

use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader, Lines};
use tokio::time::timeout;

use crate::engine_error::EngineError;
use crate::engine_message::EngineMessage;
use crate::engine_option::EngineOption;
use crate::uci_command::UciCommand;

/// Quanto aspettare al massimo che il motore si presenti e sia pronto.
/// Stockfish carica la sua rete neurale all'avvio: su un computer lento può
/// servire qualche secondo.
const STARTUP_TIMEOUT: Duration = Duration::from_secs(15);

/// La connessione con un motore: da una parte gli si scrivono comandi,
/// dall'altra si leggono le sue risposte, una riga alla volta.
///
/// È generica sul "tubo" di comunicazione (`R` per leggere, `W` per
/// scrivere): con il vero Stockfish sono l'uscita e l'ingresso del suo
/// programma; nei test sono un finto motore in memoria.
pub struct UciEngine<R, W> {
    /// Le righe che arrivano dal motore.
    lines: Lines<BufReader<R>>,
    /// Dove si scrivono i comandi per il motore.
    writer: W,
    /// Le impostazioni dichiarate dal motore all'avvio, con i loro limiti.
    options: Vec<EngineOption>,
}

impl<R, W> UciEngine<R, W>
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    /// Si presenta al motore e lo prepara.
    ///
    /// - Riceve: il tubo di lettura e quello di scrittura, e le impostazioni
    ///   da applicare subito (es. `Threads`, `Hash`).
    /// - Restituisce: la connessione pronta all'uso, dopo `uciok` e `readyok`.
    ///
    /// # Errors
    ///
    /// - [`EngineError::Timeout`] se il motore non risponde entro qualche secondo;
    /// - [`EngineError::Closed`] o [`EngineError::Io`] se la comunicazione si interrompe.
    pub async fn start(
        reader: R,
        writer: W,
        settings: &[(String, String)],
    ) -> Result<Self, EngineError> {
        let mut engine = Self {
            lines: BufReader::new(reader).lines(),
            writer,
            options: Vec::new(),
        };
        engine.send(&UciCommand::Uci).await?;
        timeout(STARTUP_TIMEOUT, engine.read_options())
            .await
            .map_err(|_| EngineError::Timeout)??;
        for (name, value) in settings {
            engine.set_option(name, value).await?;
        }
        engine.send(&UciCommand::IsReady).await?;
        timeout(STARTUP_TIMEOUT, engine.wait_ready())
            .await
            .map_err(|_| EngineError::Timeout)??;
        Ok(engine)
    }

    /// Manda un comando al motore.
    ///
    /// # Errors
    ///
    /// [`EngineError::Io`] se non si riesce a scrivere (ad esempio il motore si è chiuso).
    pub async fn send(&mut self, command: &UciCommand) -> Result<(), EngineError> {
        let line = format!("{command}\n");
        self.writer.write_all(line.as_bytes()).await?;
        self.writer.flush().await?;
        Ok(())
    }

    /// Cambia un'impostazione del motore, riportando il valore entro i limiti
    /// che il motore ha dichiarato (se li ha dichiarati).
    ///
    /// # Errors
    ///
    /// Gli stessi di [`UciEngine::send`].
    pub async fn set_option(&mut self, name: &str, value: &str) -> Result<(), EngineError> {
        let value = self
            .options
            .iter()
            .find(|option| option.name.eq_ignore_ascii_case(name))
            .map_or_else(|| value.to_owned(), |option| option.clamp(value));
        self.send(&UciCommand::SetOption {
            name: name.to_owned(),
            value,
        })
        .await
    }

    /// Aspetta la prossima riga del motore e la interpreta.
    ///
    /// Si può interrompere in qualsiasi momento senza perdere righe (è
    /// "cancel-safe"): serve per aspettare insieme il motore e nuove richieste.
    ///
    /// # Errors
    ///
    /// - [`EngineError::Closed`] se il motore si è chiuso;
    /// - [`EngineError::Io`] se la lettura non riesce.
    pub async fn next_message(&mut self) -> Result<EngineMessage, EngineError> {
        match self.lines.next_line().await? {
            // `trim_end` toglie anche il "\r" che i programmi Windows aggiungono a fine riga.
            Some(line) => Ok(EngineMessage::parse(line.trim_end())),
            None => Err(EngineError::Closed),
        }
    }

    /// Le impostazioni dichiarate dal motore.
    #[must_use]
    pub fn options(&self) -> &[EngineOption] {
        &self.options
    }

    /// Raccoglie le impostazioni dichiarate dal motore fino a `uciok`.
    async fn read_options(&mut self) -> Result<(), EngineError> {
        loop {
            match self.next_message().await? {
                EngineMessage::Option(option) => self.options.push(option),
                EngineMessage::UciOk => return Ok(()),
                _ => {}
            }
        }
    }

    /// Aspetta `readyok`.
    async fn wait_ready(&mut self) -> Result<(), EngineError> {
        while self.next_message().await? != EngineMessage::ReadyOk {}
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::UciEngine;
    use crate::engine_error::EngineError;
    use crate::fake_engine::FakeEngine;

    #[tokio::test]
    async fn introduces_itself_and_applies_the_settings_within_limits() {
        let fake = FakeEngine::start();
        let settings = [("UCI_Elo".to_owned(), "900".to_owned())];

        let engine = UciEngine::start(fake.reader, fake.writer, &settings)
            .await
            .expect("avvio riuscito");

        assert_eq!(engine.options().len(), 2);
        let received = fake.received.lock().expect("registro dei comandi").clone();
        assert_eq!(
            received,
            ["uci", "setoption name UCI_Elo value 1320", "isready"]
        );
    }

    #[tokio::test]
    async fn reports_an_engine_that_closes() {
        // Il motore chiude subito la sua uscita: la lettura trova la fine del tubo.
        let (reader, engine_side) = tokio::io::duplex(64);
        drop(engine_side);
        let (_engine_input, writer) = tokio::io::duplex(64);

        let result = UciEngine::start(reader, writer, &[]).await;

        assert!(matches!(
            result,
            Err(EngineError::Closed | EngineError::Io(_))
        ));
    }
}
