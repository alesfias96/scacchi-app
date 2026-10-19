# crates — Il nucleo dell'app

Qui vive il "cervello" dell'app: tutta la logica, scritta in **Rust** e divisa in librerie
indipendenti chiamate _crate_. Ogni crate ha un solo compito, si può testare da sola
e non sa nulla dell'interfaccia grafica.

Le crate vengono aggiunte una alla volta, solo quando servono.

## Contenuto

| Crate           | Compito                                                                                        |
| --------------- | ---------------------------------------------------------------------------------------------- |
| `chess-core/`   | Le regole degli scacchi: posizione, mosse legali, partita, fine partita, notazioni FEN/PGN/SAN |
| `app-settings/` | Le impostazioni dell'utente (tema, scacchiera, pezzi, notazione, suoni, ultima partita scelta) |
| `engine/`       | Il dialogo con i motori di scacchi (Stockfish): analisi, mosse del computer, livelli di gioco  |
| `game-session/` | La partita in corso: avversario, livello, orologio, chi può muovere, abbandono e tempo scaduto |
| `openings/`     | Il riconoscimento delle aperture (elenco di Lichess): codice ECO e nome                        |
| `storage/`      | Il database delle partite (SQLite): salvataggio a ogni mossa, archivio, ripresa                |
| `analysis/`     | I rilevatori: minacce, tattiche, struttura dei pedoni, sicurezza del re, attività dei pezzi    |
| `training/`     | L'allenamento con i puzzle: scelta, verifica delle mosse, rating, ripasso                      |

Come si usano tra loro: `game-session` usa `chess-core` (le regole), `engine` (i livelli e le
richieste), `app-settings` (la configurazione scelta) e `openings` (l'apertura giocata);
`storage` salva e rilegge le "fotografie" delle partite di `game-session`; `engine` usa
`chess-core` per scrivere le varianti in notazione SAN; `training` usa `chess-core` (le mosse del
puzzle), `analysis` (perché la soluzione funziona) e `app-settings` (le scelte dell'allenamento),
e `storage` ne salva tentativi, rating e ripasso. Nessuna crate conosce Tauri.

## Crate che verranno aggiunte

| Crate    | Fase | Compito                                                         |
| -------- | ---- | --------------------------------------------------------------- |
| `tutor/` | 7    | Unisce analisi e conoscenza strategica e produce le spiegazioni |
| `stats/` | 9    | Il calcolo delle statistiche                                    |
