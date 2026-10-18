# src-tauri/src/state — Le informazioni tenute in memoria

Mentre l'app è aperta, alcune informazioni devono restare disponibili tra una richiesta
e l'altra dell'interfaccia. Tauri le custodisce e le consegna ai comandi che le chiedono.

| File            | Cosa contiene                                                                                                          |
| --------------- | ---------------------------------------------------------------------------------------------------------------------- |
| `mod.rs`        | Elenca i file di questa cartella                                                                                       |
| `game_state.rs` | La partita in corso, protetta da un "lucchetto" (`Mutex`) perché due richieste contemporanee non la lascino mai a metà |

La partita in memoria si perde alla chiusura dell'app: il salvataggio delle partite
arriverà nella Fase 4.
