//! Un finto motore, usato solo dai test.
//!
//! Risponde ai comandi come farebbe Stockfish, ma con risposte fisse e
//! istantanee, e ricorda tutti i comandi ricevuti. Così i test verificano il
//! dialogo UCI senza dover avviare il vero motore.

use std::sync::{Arc, Mutex};

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, DuplexStream, duplex};

/// Le righe con cui il finto motore si presenta (`uci`).
const INTRODUCTION: &str = "id name Finto motore\n\
    option name UCI_Elo type spin default 1320 min 1320 max 3190\n\
    option name Skill Level type spin default 20 min 0 max 20\n\
    uciok\n";

/// Le righe inviate durante un'analisi infinita: due profondità complete
/// e, in mezzo, una riga con un punteggio solo provvisorio.
const ANALYSIS_LINES: &str = "info depth 1 score cp 25 pv e2e4 e7e5\n\
    info depth 2 score cp 30 lowerbound pv e2e4\n\
    info depth 2 score cp 30 pv d2d4 d7d5\n";

/// Il finto motore e i due capi del tubo per parlargli.
pub(crate) struct FakeEngine {
    /// Da qui si leggono le risposte del finto motore.
    pub reader: DuplexStream,
    /// Qui si scrivono i comandi per il finto motore.
    pub writer: DuplexStream,
    /// Tutti i comandi ricevuti, nell'ordine.
    pub received: Arc<Mutex<Vec<String>>>,
}

impl FakeEngine {
    /// Avvia il finto motore in un compito separato.
    pub fn start() -> Self {
        let (reader, mut engine_output) = duplex(4096);
        let (writer, engine_input) = duplex(4096);
        let received = Arc::new(Mutex::new(Vec::new()));
        let log = Arc::clone(&received);

        tokio::spawn(async move {
            let mut lines = BufReader::new(engine_input).lines();
            while let Ok(Some(line)) = lines.next_line().await {
                log.lock().expect("registro dei comandi").push(line.clone());
                let reply = match line.as_str() {
                    "uci" => INTRODUCTION.to_owned(),
                    "isready" => "readyok\n".to_owned(),
                    "go infinite" => ANALYSIS_LINES.to_owned(),
                    "stop" => "bestmove d2d4\n".to_owned(),
                    "quit" => break,
                    go if go.starts_with("go") => {
                        "info depth 1 score cp 10 pv g1f3\nbestmove g1f3\n".to_owned()
                    }
                    _ => String::new(),
                };
                if engine_output.write_all(reply.as_bytes()).await.is_err() {
                    break;
                }
            }
        });

        Self {
            reader,
            writer,
            received,
        }
    }
}
