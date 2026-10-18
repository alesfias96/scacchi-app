# crates/chess-core/src — Il codice delle regole

Ogni file ha un solo compito. I test di ogni file stanno in fondo al file stesso.

## Contenuto

| File               | Cosa fa                                                                                                                                                             |
| ------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `lib.rs`           | La "porta d'ingresso": elenca i moduli e decide cosa è visibile da fuori                                                                                            |
| `side.rs`          | Il colore di un giocatore (bianco o nero)                                                                                                                           |
| `legal_move.rs`    | Una mossa legale: casa di partenza, casa di arrivo, notazione UCI                                                                                                   |
| `position_view.rs` | La descrizione completa di una posizione (FEN, turno, scacco, mosse legali)                                                                                         |
| `setup.rs`         | Crea una posizione: quella iniziale o una data in notazione FEN                                                                                                     |
| `setup_error.rs`   | Gli errori possibili nella creazione di una posizione, con messaggi chiari                                                                                          |
| `game.rs`          | La partita: l'elenco delle posizioni e delle mosse, la mossa mostrata ("cursore"), giocare, annullare, scorrere, abbandonare, perdere per tempo, capire se è finita |
| `game_view.rs`     | La descrizione completa della partita da inviare all'interfaccia                                                                                                    |
| `move_record.rs`   | Una mossa già giocata: numero, colore, notazione SAN e UCI, case, se è una presa                                                                                    |
| `game_result.rs`   | Il risultato di una partita finita e il motivo (matto, stallo, materiale insufficiente, ripetizione, 50 mosse, tempo scaduto, abbandono)                            |
| `game_error.rs`    | Gli errori possibili durante la partita (mossa illegale, partita finita, FEN o PGN non validi), con messaggi chiari                                                 |
| `pgn_export.rs`    | Scrive la partita in formato PGN                                                                                                                                    |
| `pgn_import.rs`    | Legge una partita in formato PGN (ignora commenti, varianti e annotazioni)                                                                                          |
| `game_import.rs`   | Riconosce da solo se il testo incollato è una FEN o un PGN e lo carica                                                                                              |
| `line_view.rs`     | Una sequenza di mosse (ad esempio la variante proposta dal motore) scritta in notazione SAN, con il numero di mossa                                                 |
