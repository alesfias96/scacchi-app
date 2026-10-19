# crates/stats/src — Il codice delle statistiche

Ogni file ha un solo compito. I test stanno in fondo a `report.rs`, che mette insieme tutto.

## Contenuto

| File               | Cosa fa                                                                               |
| ------------------ | ------------------------------------------------------------------------------------- |
| `lib.rs`           | La "porta d'ingresso": elenca i moduli e decide cosa è visibile da fuori              |
| `stats_input.rs`   | I dati grezzi: partite, mosse delle partite analizzate, tentativi dei puzzle          |
| `stats_view.rs`    | Le statistiche pronte per l'interfaccia, una struttura per scheda                     |
| `judged_move.rs`   | Le mosse della persona con il loro giudizio e la precisione (come nell'analisi)       |
| `game_score.rs`    | Il risultato di una partita dal punto di vista della persona (vinta, patta, persa)    |
| `level_stats.rs`   | La scheda «Livello»: Elo stimato, rating dei puzzle e precisione nel tempo            |
| `game_stats.rs`    | La scheda «Partite»: risultati per colore e livello, errori per fase e con poco tempo |
| `opening_stats.rs` | La scheda «Aperture»: le famiglie più giocate per colore                              |
| `tactics_stats.rs` | La scheda «Calcolo e tattica»: puzzle per numero di mosse, profondità nel tempo, temi |
| `report.rs`        | Tutte le statistiche insieme (`build_report`) e i test con partite e puzzle di prova  |
