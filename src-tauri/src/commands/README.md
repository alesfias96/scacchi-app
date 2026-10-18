# src-tauri/src/commands — Le richieste dell'interfaccia

I "comandi" sono le richieste che l'interfaccia può fare al nucleo Rust, chiamandole per nome.
Ogni file raccoglie i comandi di un argomento. I comandi non contengono logica:
passano la richiesta alla crate giusta in `crates/` e restituiscono la risposta.

## Contenuto

| File          | Cosa fa                                                                                                                                                                                                                                                                                                                |
| ------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `mod.rs`      | Elenca i file dei comandi                                                                                                                                                                                                                                                                                              |
| `position.rs` | Comandi sulla posizione. Per ora `initial_position`: restituisce la posizione iniziale                                                                                                                                                                                                                                 |
| `game.rs`     | Comandi sulla partita in corso: `get_game`, `new_game` (con avversario, colore, livello e tempo), `play_move`, `undo_move`, `go_to_ply`, `resign_game`, `import_game_text` (incolla FEN o PGN), `export_pgn`. Ognuno restituisce la partita aggiornata e poi avvia le reazioni (mossa del computer, orologio, analisi) |
| `engine.rs`   | Comandi sul motore: `get_engine_levels` (i dieci livelli) e `set_analysis` (accende o spegne l'analisi in tempo reale)                                                                                                                                                                                                 |
| `settings.rs` | Comandi sulle impostazioni: `get_settings` (le legge) e `save_settings` (le salva), nella cartella di configurazione dell'app                                                                                                                                                                                          |

## Come si aggiunge un comando

1. Si scrive la funzione nel file dell'argomento, con `#[tauri::command]`.
2. Si aggiunge il suo nome all'elenco `generate_handler!` in `src-tauri/src/lib.rs`.
3. Si aggiunge la funzione che lo chiama in `src/services/` (lato interfaccia).
