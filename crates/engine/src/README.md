# crates/engine/src — Il codice del dialogo con il motore

Ogni file ha un solo compito. I test di ogni file stanno in fondo al file stesso.

## Ciò che si manda al motore

| File                 | Cosa fa                                                                                |
| -------------------- | -------------------------------------------------------------------------------------- |
| `uci_command.rs`     | I comandi (`uci`, `isready`, `setoption`, `position`, `go`, `stop`, `quit`) come testo |
| `search_position.rs` | La posizione: partenza della partita più le mosse giocate                              |
| `search_limits.rs`   | Per quanto pensare: all'infinito, tempo fisso, orologio, profondità massima            |

## Ciò che il motore risponde

| File                | Cosa fa                                                                                 |
| ------------------- | --------------------------------------------------------------------------------------- |
| `engine_message.rs` | Legge una riga del motore e capisce che cos'è (`uciok`, `info`, `bestmove`, …)          |
| `engine_info.rs`    | Le informazioni di una riga `info`: profondità, punteggio, variante                     |
| `engine_option.rs`  | Un'impostazione dichiarata dal motore, con i suoi limiti (per non superarli mai)        |
| `score.rs`          | Il punteggio come lo dà il motore: centipedoni o matto, dal punto di vista di chi muove |

## La connessione e le richieste

| File               | Cosa fa                                                                                                                                         |
| ------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| `uci_engine.rs`    | La connessione: presentazione iniziale, invio dei comandi, lettura delle righe                                                                  |
| `engine_job.rs`    | Le richieste possibili: analizza, trova la mossa, valuta per poco tempo, fermati                                                                |
| `engine_handle.rs` | Il "telecomando": manda le richieste senza aspettare; per la mossa e per la valutazione breve (`evaluate`, usata dal tutor) aspetta la risposta |
| `engine_worker.rs` | Il "lavoratore": esegue le richieste una alla volta, interrompendo quella vecchia quando ne arriva una nuova                                    |
| `connect.rs`       | Avvia il programma del motore (senza aprire finestre del terminale su Windows) e lo collega                                                     |

## I dati per l'interfaccia e i livelli

| File                 | Cosa fa                                                                           |
| -------------------- | --------------------------------------------------------------------------------- |
| `evaluation.rs`      | La valutazione dal punto di vista del Bianco (per la barra di valutazione)        |
| `analysis_update.rs` | Un aggiornamento dell'analisi: valutazione, profondità, variante in notazione SAN |
| `strength.rs`        | I dieci livelli di gioco: nome, Elo indicativo, impostazioni e tempi              |
| `engine_error.rs`    | Gli errori possibili, con messaggi chiari                                         |
| `fake_engine.rs`     | Un finto motore usato solo dai test                                               |
| `lib.rs`             | La "porta d'ingresso": elenca i moduli e decide cosa è visibile da fuori          |
