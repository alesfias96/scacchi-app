# crates — Il nucleo dell'app

Qui vive il "cervello" dell'app: tutta la logica, scritta in **Rust** e divisa in librerie
indipendenti chiamate _crate_. Ogni crate ha un solo compito, si può testare da sola
e non sa nulla dell'interfaccia grafica.

Le crate vengono aggiunte una alla volta, solo quando servono.

## Contenuto

| Crate           | Compito                                                                                                            |
| --------------- | ------------------------------------------------------------------------------------------------------------------ |
| `chess-core/`   | Le regole degli scacchi: posizione, mosse legali, partita, fine partita, notazioni FEN/PGN/SAN                     |
| `app-settings/` | Le impostazioni dell'utente (tema, scacchiera, pezzi, notazione, suoni, ultima partita scelta)                     |
| `engine/`       | Il dialogo con i motori di scacchi (Stockfish): analisi, mosse del computer, livelli di gioco                      |
| `game-session/` | La partita in corso: avversario, livello, orologio, chi può muovere, abbandono e tempo scaduto                     |
| `openings/`     | Il riconoscimento delle aperture (elenco di Lichess): codice ECO e nome                                            |
| `storage/`      | Il database delle partite (SQLite): salvataggio a ogni mossa, archivio, ripresa                                    |
| `analysis/`     | I rilevatori: minacce, tattiche, struttura dei pedoni, sicurezza del re, attività dei pezzi                        |
| `training/`     | L'allenamento con i puzzle: scelta, verifica delle mosse, rating, ripasso                                          |
| `tutor/`        | Il tutor: spiega le posizioni unendo i fatti dei rilevatori e la base di conoscenza (`knowledge/`)                 |
| `review/`       | L'analisi delle partite giocate: giudizio delle mosse, precisione, momenti critici, puzzle dagli errori            |
| `stats/`        | Le statistiche della sezione «Progressi»: livello stimato, risultati, errori per fase, aperture, calcolo e tattica |

Come si usano tra loro: `game-session` usa `chess-core` (le regole), `engine` (i livelli e le
richieste), `app-settings` (la configurazione scelta) e `openings` (l'apertura giocata);
`storage` salva e rilegge le "fotografie" delle partite di `game-session`; `engine` usa
`chess-core` per scrivere le varianti in notazione SAN; `training` usa `chess-core` (le mosse del
puzzle), `analysis` (perché la soluzione funziona) e `app-settings` (le scelte dell'allenamento),
e `storage` ne salva tentativi, rating e ripasso; `tutor` usa `analysis` (i fatti), `openings`
(l'apertura), `chess-core` (le mosse provate) e i tipi della valutazione di `engine`. `review` usa `chess-core`,
`analysis`, `tutor` e i tipi di `engine` per la revisione delle partite. `stats` usa `review` (lo stesso giudizio delle mosse), `training`
(i nomi dei temi), `engine` (l'Elo dei livelli) e i tipi di `chess-core`, `analysis` e
`app-settings`. Nessuna crate conosce Tauri.
