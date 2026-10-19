# crates/storage/src — Il codice del database

Ogni file ha un solo compito. I test di ogni file stanno in fondo al file stesso.

| File                 | Cosa fa                                                                                                                                    |
| -------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| `lib.rs`             | La "porta d'ingresso": elenca i moduli e decide cosa è visibile da fuori                                                                   |
| `database.rs`        | Apre il database (file o memoria, per i test) e applica le migrazioni                                                                      |
| `save_game.rs`       | Salva una partita: la crea o la aggiorna, tutto in una "transazione" (o si salva tutto, o niente)                                          |
| `load_game.rs`       | Legge una partita salvata e trova l'ultima non finita (da riprendere all'avvio)                                                            |
| `list_games.rs`      | L'elenco dell'archivio, dalla più recente, con i filtri                                                                                    |
| `delete_game.rs`     | Elimina una partita e le sue mosse (gli appunti collegati restano, senza il collegamento alla partita)                                     |
| `puzzle_attempts.rs` | Salva i tentativi dei puzzle; legge il rating attuale, i puzzle già fatti, quanti risolti                                                  |
| `puzzle_reviews.rs`  | Il ripasso dei puzzle sbagliati: scatola e scadenza di ogni puzzle, quali sono da ripassare oggi                                           |
| `move_analysis.rs`   | Salva e rilegge i dati dell'analisi di ogni mossa (valutazioni, mossa migliore, perdita, fase, temi)                                       |
| `own_puzzles.rs`     | Salva e rilegge i puzzle nati dagli errori gravi delle proprie partite                                                                     |
| `stats_data.rs`      | I dati grezzi per le statistiche: partite, mosse analizzate e tentativi dei puzzle (con i filtri); le partite finite non ancora analizzate |
| `notes_data.rs`      | Salva, rilegge ed elimina gli appunti personali                                                                                            |
| `game_summary.rs`    | Il riassunto di una partita per una riga dell'archivio                                                                                     |
| `game_filter.rs`     | I filtri: avversario, esito (vinte, perse, patte, in corso), testo nell'apertura                                                           |
| `codes.rs`           | Le parole fisse con cui colori, avversario e motivi di fine partita sono scritti nel database (non si cambiano)                            |
| `storage_error.rs`   | Gli errori possibili, con messaggi chiari                                                                                                  |
