# src/services — Comunicazione con il nucleo Rust

Qui stanno le funzioni che inviano richieste al nucleo Rust e ne ricevono le risposte.
È l'unico punto dell'interfaccia che conosce i nomi dei comandi Rust: se un comando cambia nome,
si modifica solo qui.

I tipi dei dati ricevuti arrivano da `src/bindings/`, generati automaticamente da Rust.

## Contenuto

| File                 | Cosa fa                                                                                                                                                                                                                                  |
| -------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `positionService.ts` | Chiede a Rust la posizione iniziale (comando `initial_position`), usata dall'anteprima delle Impostazioni                                                                                                                                |
| `settingsService.ts` | Legge e salva le impostazioni (comandi `get_settings` e `save_settings`)                                                                                                                                                                 |
| `gameService.ts`     | La partita in corso: leggerla, giocare e annullare mosse, scorrerla, cominciarne una nuova, incollare FEN/PGN e copiare il PGN (comandi `get_game`, `play_move`, `undo_move`, `go_to_ply`, `new_game`, `import_game_text`, `export_pgn`) |
