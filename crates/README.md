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

Come si usano tra loro: `game-session` usa `chess-core` (le regole), `engine` (i livelli e le
richieste) e `app-settings` (la configurazione scelta); `engine` usa `chess-core` per scrivere
le varianti in notazione SAN. Nessuna crate conosce Tauri.

## Crate che verranno aggiunte

| Crate       | Fase | Compito                                                                 |
| ----------- | ---- | ----------------------------------------------------------------------- |
| `storage/`  | 4    | Il database: salvataggio di partite, mosse, progressi                   |
| `analysis/` | 5    | I rilevatori: minacce, tattiche, struttura dei pedoni, sicurezza del re |
| `training/` | 6    | Gli esercizi di calcolo e visualizzazione                               |
| `tutor/`    | 7    | Unisce analisi e conoscenza strategica e produce le spiegazioni         |
| `stats/`    | 9    | Il calcolo delle statistiche                                            |
