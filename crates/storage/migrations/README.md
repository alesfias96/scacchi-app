# crates/storage/migrations — Lo schema del database

Ogni file descrive un passo nella storia dello schema del database, in linguaggio SQL. All'apertura
il database controlla quali passi ha già fatto e applica, in ordine, quelli mancanti.

**Regola:** un file già rilasciato non si modifica mai. Per cambiare lo schema se ne aggiunge uno
nuovo, con il numero successivo (`0002_descrizione.sql`).

| File                       | Cosa fa                                                                                                                          |
| -------------------------- | -------------------------------------------------------------------------------------------------------------------------------- |
| `0001_games_and_moves.sql` | Crea le tabelle `games` (le partite) e `moves` (le mosse, con i dati per le statistiche)                                         |
| `0002_training.sql`        | Crea le tabelle `puzzle_attempts` (i tentativi dei puzzle) e `puzzle_reviews` (il ripasso)                                       |
| `0003_review.sql`          | Aggiunge `analyzed_at` alle partite (quando sono state analizzate) e crea la tabella `own_puzzles` (i puzzle dagli errori gravi) |
| `0004_computer_engine.sql` | Aggiunge `engine` alle partite: il motore del computer (`stockfish` o `maia`; le partite già salvate restano Stockfish)          |
| `0005_notes.sql`           | Crea la tabella `notes` (gli appunti: titolo, testo, etichette, posizione e partita collegate)                                   |
