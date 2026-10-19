# src/services — Comunicazione con il nucleo Rust

Qui stanno le funzioni che inviano richieste al nucleo Rust e ne ricevono le risposte.
È l'unico punto dell'interfaccia che conosce i nomi dei comandi e degli eventi Rust: se un comando cambia nome,
si modifica solo qui.

I tipi dei dati ricevuti arrivano da `src/bindings/`, generati automaticamente da Rust.

## Contenuto

| File                 | Cosa fa                                                                                                                                                                                                                                                                                                                            |
| -------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `positionService.ts` | Chiede a Rust la posizione iniziale (comando `initial_position`), usata dall'anteprima delle Impostazioni                                                                                                                                                                                                                          |
| `settingsService.ts` | Legge e salva le impostazioni (comandi `get_settings` e `save_settings`)                                                                                                                                                                                                                                                           |
| `gameService.ts`     | La partita in corso: leggerla, cominciarne una nuova (con avversario, colore, livello e tempo), giocare e annullare mosse, scorrerla, abbandonare, incollare FEN/PGN, copiare il PGN, salvarla o aprirla da un file `.pgn` (`save_pgn_file`, `open_pgn_file`); ascolta l'evento `game-updated` (mossa del computer, tempo scaduto) |
| `engineService.ts`   | Il motore: i livelli di gioco (`get_engine_levels`), l'analisi accesa o spenta (`set_analysis`); ascolta gli eventi `analysis-update` ed `engine-error`                                                                                                                                                                            |
| `archiveService.ts`  | L'archivio: elenco delle partite (`list_games`), apertura (`open_game`), eliminazione (`delete_game`); ascolta l'evento `storage-error`                                                                                                                                                                                            |
| `insightService.ts`  | Le osservazioni: i fatti trovati dai rilevatori nella posizione mostrata (`get_position_report`)                                                                                                                                                                                                                                   |
| `trainingService.ts` | L'allenamento: riepilogo (`get_training_overview`), nuovo puzzle (`start_puzzle`), mossa (`play_puzzle_move`), aiuto (`puzzle_hint`), soluzione (`show_puzzle_solution`)                                                                                                                                                           |
| `tutorService.ts`    | Il tutor: spiegazione della posizione (`explain_position`) e mosse provate (`try_what_if`)                                                                                                                                                                                                                                         |
