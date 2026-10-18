# src-tauri/src/state — Le informazioni tenute in memoria

Mentre l'app è aperta, alcune informazioni devono restare disponibili tra una richiesta
e l'altra dell'interfaccia. Tauri le custodisce e le consegna ai comandi che le chiedono.

| File               | Cosa contiene                                                                                                                                                                              |
| ------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `mod.rs`           | Elenca i file di questa cartella                                                                                                                                                           |
| `session_state.rs` | La partita in corso (con avversario e orologio), protetta da un "lucchetto" (`Mutex`) perché due richieste contemporanee non la lascino mai a metà                                         |
| `engines_state.rs` | I due Stockfish: quello che gioca e quello che analizza. Ognuno si avvia la prima volta che serve e si riavvia da solo se si chiude per un errore; l'analisi si accende e si spegne da qui |
| `engine_path.rs`   | Dove si trova il programma di Stockfish (accanto all'eseguibile dell'app, sia in sviluppo sia nell'app installata)                                                                         |

La partita in memoria si perde alla chiusura dell'app: il salvataggio delle partite
arriverà nella Fase 4.
