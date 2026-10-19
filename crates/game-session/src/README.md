# crates/game-session/src — Il codice della partita in corso

Ogni file ha un solo compito. I test di ogni file stanno in fondo al file stesso.

## Contenuto

| File                  | Cosa fa                                                                                                                   |
| --------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| `lib.rs`              | La "porta d'ingresso": elenca i moduli e decide cosa è visibile da fuori                                                  |
| `game_session.rs`     | La partita in corso: mosse della persona e del computer, annulla, abbandona, controllo del tempo, posizione da analizzare |
| `clock.rs`            | L'orologio: tempo di ciascuno, incremento, chi sta consumando il tempo, chi ha finito il tempo                            |
| `clock_view.rs`       | L'orologio come lo mostra l'interfaccia (millesimi di secondo e chi è in moto)                                            |
| `computer_view.rs`    | Il computer avversario come lo mostra l'interfaccia (colore, motore e livello)                                            |
| `session_view.rs`     | Tutto ciò che l'interfaccia mostra: partita, avversario, orologio, cosa si può fare adesso                                |
| `engine_request.rs`   | La richiesta di una mossa a Stockfish, con il contrassegno dello stato della partita                                      |
| `maia_request.rs`     | La richiesta di una mossa a Maia (partita, livello, pausa minima), con lo stesso contrassegno                             |
| `computer_level.rs`   | I livelli dei due motori: nome ed Elo di un livello di Stockfish o di Maia                                                |
| `move_timer.rs`       | Il tempo impiegato per ogni mossa e il tempo rimasto sull'orologio dopo la mossa                                          |
| `session_snapshot.rs` | La "fotografia" della partita da salvare nell'archivio, e la ripresa da una fotografia                                    |
| `session_id.rs`       | Il nome unico di ogni partita e la data in cui è cominciata                                                               |
| `coin_flip.rs`        | Il sorteggio del colore ("a sorte")                                                                                       |
| `session_error.rs`    | Gli errori possibili, con messaggi chiari (non è il tuo turno, torna all'ultima mossa, …)                                 |
