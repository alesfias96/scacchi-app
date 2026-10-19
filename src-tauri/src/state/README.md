# src-tauri/src/state — Le informazioni tenute in memoria

Mentre l'app è aperta, alcune informazioni devono restare disponibili tra una richiesta
e l'altra dell'interfaccia. Tauri le custodisce e le consegna ai comandi che le chiedono.

| File                | Cosa contiene                                                                                                                                                                                                                    |
| ------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `mod.rs`            | Elenca i file di questa cartella                                                                                                                                                                                                 |
| `session_state.rs`  | La partita in corso (con avversario e orologio), protetta da un "lucchetto" (`Mutex`) perché due richieste contemporanee non la lascino mai a metà                                                                               |
| `engines_state.rs`  | I tre Stockfish: quello che gioca, quello che analizza e quello del tutor (valutazioni brevi). Ognuno si avvia la prima volta che serve e si riavvia da solo se si chiude per un errore; l'analisi si accende e si spegne da qui |
| `engine_path.rs`    | Dove si trova il programma di Stockfish (accanto all'eseguibile dell'app, sia in sviluppo sia nell'app installata)                                                                                                               |
| `database_state.rs` | Il database delle partite (aperto al primo uso, nella cartella dati dell'app) e la coda dei salvataggi                                                                                                                           |
| `training_state.rs` | Il puzzle in corso e quando è cominciato (per misurare il tempo impiegato), protetto da un "lucchetto" come la partita                                                                                                           |

Dalla Fase 4 la partita in corso si salva da sola nel database a ogni cambiamento, e all'avvio
l'ultima partita non finita viene ripresa.
