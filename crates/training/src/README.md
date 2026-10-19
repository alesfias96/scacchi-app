# crates/training/src — Il codice dell'allenamento

Ogni file ha un solo compito. I test di ogni file stanno in fondo al file stesso.

## Contenuto

| File                   | Cosa fa                                                                                                                                                    |
| ---------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `lib.rs`               | La "porta d'ingresso": elenca i moduli e decide cosa è visibile da fuori                                                                                   |
| `puzzle.rs`            | Un puzzle: posizione, mossa dell'avversario e soluzione, difficoltà, temi; la lettura di una riga                                                          |
| `catalog.rs`           | L'elenco dei puzzle inclusi nell'app (letto la prima volta che serve, poi tenuto in memoria)                                                               |
| `theme.rs`             | I temi dei puzzle con il nome in italiano, quanti puzzle ha ciascuno e il primo aiuto (il tema a parole)                                                   |
| `selection.rs`         | La scelta del prossimo puzzle (profondità, tema, difficoltà rispetto al rating) e di quello da ripassare                                                   |
| `random.rs`            | Una scelta "a caso" semplice e ripetibile (nei test dà sempre lo stesso risultato)                                                                         |
| `attempt.rs`           | Il puzzle in corso: verifica delle mosse, risposta dell'avversario, aiuto a gradini (prima il tema, poi il pezzo), soluzione spiegata dal tutor, risultato |
| `attempt_view.rs`      | La descrizione del puzzle in corso per l'interfaccia (e ciò che si scopre a puzzle finito)                                                                 |
| `rating.rs`            | Il rating dei puzzle, con la formula Elo                                                                                                                   |
| `review_schedule.rs`   | Le scadenze del ripasso (metodo delle scatole: 1, 3, 7, 14, 30 giorni)                                                                                     |
| `training_overview.rs` | Il riepilogo mostrato all'apertura della sezione: rating, puzzle risolti, ripassi, temi                                                                    |
| `training_error.rs`    | Gli errori possibili, con messaggi chiari                                                                                                                  |
