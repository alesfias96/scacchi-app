# crates/review/src — Il codice dell'analisi delle partite

Ogni file ha un solo compito. I test di ogni file stanno in fondo al file stesso.

## Contenuto

| File                 | Cosa fa                                                                            |
| -------------------- | ---------------------------------------------------------------------------------- |
| `lib.rs`             | La "porta d'ingresso": elenca i moduli e decide cosa è visibile da fuori           |
| `review_input.rs`    | Cosa serve per analizzare: la partita e il giudizio del motore su ogni posizione   |
| `replay.rs`          | Rigioca la partita e ne raccoglie le posizioni (FEN e posizioni per la scacchiera) |
| `win_chance.rs`      | La probabilità di vittoria da una valutazione e la precisione di una mossa         |
| `move_quality.rs`    | Il giudizio di una mossa: ottima, buona, imprecisione, errore, errore grave        |
| `game_reviewer.rs`   | La revisione completa: giudizi, riepiloghi dei giocatori, momenti critici, grafico |
| `review_view.rs`     | La forma della revisione per l'interfaccia                                         |
| `move_analysis.rs`   | I dati di ogni mossa da salvare nel database, per le statistiche                   |
| `own_puzzle.rs`      | I puzzle nati dagli errori gravi                                                   |
| `review_progress.rs` | I messaggi sull'analisi in corso (avanzamento, fine, coda)                             |
| `review_error.rs`    | Gli errori possibili                                                               |
| `test_game.rs`       | Una partita di prova, solo per i test                                              |
