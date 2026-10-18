# src-tauri/src/events — I messaggi in tempo reale per l'interfaccia

I **comandi** rispondono a una richiesta dell'interfaccia. Gli **eventi** invece partono da soli,
quando succede qualcosa che l'interfaccia non ha chiesto in quel momento. L'interfaccia li
ascolta in `src/services/`.

| File               | Evento            | Quando parte                                                                      |
| ------------------ | ----------------- | --------------------------------------------------------------------------------- |
| `mod.rs`           |                   | Elenca i file di questa cartella                                                  |
| `game_updated.rs`  | `game-updated`    | La partita è cambiata da sola: mossa del computer, tempo scaduto                  |
| `analysis.rs`      | `analysis-update` | Il motore che analizza ha completato una nuova profondità (valutazione, variante) |
| `engine_error.rs`  | `engine-error`    | Il motore ha un problema (ad esempio Stockfish non si avvia)                      |
| `storage_error.rs` | `storage-error`   | Il database ha un problema (una partita non si salva o non si legge)              |
