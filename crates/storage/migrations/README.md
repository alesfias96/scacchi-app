# crates/storage/migrations — Lo schema del database

Ogni file descrive un passo nella storia dello schema del database, in linguaggio SQL. All'apertura
il database controlla quali passi ha già fatto e applica, in ordine, quelli mancanti.

**Regola:** un file già rilasciato non si modifica mai. Per cambiare lo schema se ne aggiunge uno
nuovo, con il numero successivo (`0002_descrizione.sql`).

| File                       | Cosa fa                                                                                  |
| -------------------------- | ---------------------------------------------------------------------------------------- |
| `0001_games_and_moves.sql` | Crea le tabelle `games` (le partite) e `moves` (le mosse, con i dati per le statistiche) |
