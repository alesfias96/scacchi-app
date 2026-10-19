# crates/tutor/src — Il codice del tutor

Ogni file ha un solo compito. I test di ogni file stanno in fondo al file stesso.

## Contenuto

| File                   | Cosa fa                                                                                                              |
| ---------------------- | -------------------------------------------------------------------------------------------------------------------- |
| `lib.rs`               | La "porta d'ingresso": elenca i moduli e decide cosa è visibile da fuori                                             |
| `topic.rs`             | Gli argomenti delle schede (aperture, strutture, …) e i "quando vale" (`structure:carlsbad`)                         |
| `knowledge_card.rs`    | Una scheda com'è scritta nel suo file, e la sua lettura con i controlli                                              |
| `library.rs`           | Tutte le schede incluse nell'app; la scheda di una chiave e quella dell'apertura giocata                             |
| `board_query.rs`       | Piccole domande sulla scacchiera viste da un colore ("pedone in d4" per il Bianco = d5 per il Nero)                  |
| `recognition.rs`       | Ciò che il tutor riconosce, con l'elenco di tutte le chiavi che sa riconoscere                                       |
| `structures.rs`        | Le strutture pedonali: isolano, pedoni sospesi, Carlsbad, Maróczy, Stonewall, catene, Benoni, centro aperto          |
| `endgame_kind.rs`      | Il tipo di finale dai pezzi rimasti                                                                                  |
| `middlegame_themes.rs` | I temi del mediogioco: arrocchi opposti, avamposti, torre in settima                                                 |
| `principles.rs`        | I principi: sviluppo, arrocco, centro, colonne aperte, coppia degli alfieri, re attivo, pedoni passati, semplificare |
| `evaluation_words.rs`  | La valutazione di Stockfish a parole e il giudizio su una mossa provata                                              |
| `explanation.rs`       | La spiegazione di una posizione: mette in ordine fatti, riconoscimenti e schede                                      |
| `explanation_view.rs`  | La forma della spiegazione per l'interfaccia (parti, frasi, schede, piani)                                           |
| `card_view.rs`         | Una scheda adattata alla posizione: i piani con il colore giusto                                                     |
| `what_if.rs`           | Gli scenari "e se…": cosa cambia con una mossa                                                                       |
| `tutor_error.rs`       | Gli errori possibili, con messaggi chiari                                                                            |
