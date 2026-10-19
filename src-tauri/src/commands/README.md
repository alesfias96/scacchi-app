# src-tauri/src/commands — Le richieste dell'interfaccia

I "comandi" sono le richieste che l'interfaccia può fare al nucleo Rust, chiamandole per nome.
Ogni file raccoglie i comandi di un argomento. I comandi non contengono logica:
passano la richiesta alla crate giusta in `crates/` e restituiscono la risposta.

## Contenuto

| File            | Cosa fa                                                                                                                                                                                                                                                                                                                |
| --------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `mod.rs`        | Elenca i file dei comandi                                                                                                                                                                                                                                                                                              |
| `position.rs`   | Comandi sulla posizione. Per ora `initial_position`: restituisce la posizione iniziale                                                                                                                                                                                                                                 |
| `game.rs`       | Comandi sulla partita in corso: `get_game`, `new_game` (con avversario, colore, livello e tempo), `play_move`, `undo_move`, `go_to_ply`, `resign_game`, `import_game_text` (incolla FEN o PGN), `export_pgn`. Ognuno restituisce la partita aggiornata e poi avvia le reazioni (mossa del computer, orologio, analisi) |
| `engine.rs`     | Comandi sul motore: `get_engine_levels` (i dieci livelli di Stockfish), `get_maia_levels` (i nove livelli di Maia) e `set_analysis` (accende o spegne l'analisi in tempo reale)                                                                                                                                        |
| `archive.rs`    | Comandi sull'archivio: `list_games` (elenco con filtri), `open_game` (apre una partita salvata in "Gioca"), `delete_game`                                                                                                                                                                                              |
| `analysis.rs`   | Comando dei rilevatori: `get_position_report` (i fatti trovati nella posizione mostrata)                                                                                                                                                                                                                               |
| `pgn_file.rs`   | Comandi sui file: `save_pgn_file` e `open_pgn_file`, con la finestra del sistema per scegliere il file                                                                                                                                                                                                                 |
| `settings.rs`   | Comandi sulle impostazioni: `get_settings` (le legge) e `save_settings` (le salva), nella cartella di configurazione dell'app                                                                                                                                                                                          |
| `training.rs`   | Comandi dell'allenamento: `get_training_overview` (rating, statistiche, ripassi, temi), `start_puzzle`, `play_puzzle_move`, `puzzle_hint`, `show_puzzle_solution`. Quando il risultato di un puzzle si decide, lo salva nel database (tentativo, rating, ripasso)                                                      |
| `tutor.rs`      | Comandi del tutor: `explain_position` (la spiegazione della posizione) e `try_what_if` (cosa cambia con una mossa provata). Se richiesto, chiedono al terzo Stockfish una valutazione di mezzo secondo                                                                                                                 |
| `review.rs`     | Comandi dell'analisi delle partite: `start_review` (la fa partire in sottofondo), `cancel_review`, `running_review` (quale partita si sta analizzando), `get_review` (la revisione di una partita già analizzata)                                                                                                      |
| `variations.rs` | Comandi delle varianti: `go_to_node` (mostra una mossa dell'albero), `promote_variation`, `delete_variation`                                                                                                                                                                                                           |
| `clipboard.rs`  | `copy_text`: copia un testo negli appunti del sistema (Copia FEN, Copia PGN), con il plugin ufficiale di Tauri                                                                                                                                                                                                         |
| `stats.rs`      | Comandi delle statistiche: `get_stats` (legge i dati grezzi e li passa alla crate `stats`) e `analyze_missing_games` (mette in coda le partite finite non analizzate)                                                                                                                                                  |

## Come si aggiunge un comando

1. Si scrive la funzione nel file dell'argomento, con `#[tauri::command]`.
2. Si aggiunge il suo nome all'elenco `generate_handler!` in `src-tauri/src/lib.rs`.
3. Si aggiunge la funzione che lo chiama in `src/services/` (lato interfaccia).
