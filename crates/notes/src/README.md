# crates/notes/src — Il codice degli appunti

Ogni file ha un solo compito. I test di ogni file stanno in fondo al file stesso.

| File         | Cosa fa                                                                                                       |
| ------------ | ------------------------------------------------------------------------------------------------------------- |
| `lib.rs`     | La "porta d'ingresso": elenca i moduli e decide cosa è visibile da fuori                                      |
| `note.rs`    | L'appunto (`Note`), come lo scrive l'interfaccia (`NoteInput`) e il collegamento a una posizione (`NoteLink`) |
| `prepare.rs` | Dall'appunto scritto a quello da salvare: nome unico, titolo, etichette ripulite, date                        |
| `folding.rs` | Il testo minuscolo e senza accenti, per la ricerca                                                            |
| `search.rs`  | La ricerca (`NoteQuery`) e l'elenco delle etichette                                                           |
