# crates — Il nucleo dell'app

Qui vive il "cervello" dell'app: tutta la logica, scritta in **Rust** e divisa in librerie
indipendenti chiamate _crate_. Ogni crate ha un solo compito, si può testare da sola
e non sa nulla dell'interfaccia grafica.

Le crate vengono aggiunte una alla volta, solo quando servono.

## Contenuto

| Crate           | Compito                                                                              |
| --------------- | ------------------------------------------------------------------------------------ |
| `chess-core/`   | Le regole degli scacchi: posizione, mosse legali, scacco, notazione FEN              |
| `app-settings/` | Le impostazioni dell'utente (tema, scacchiera, pezzi): valori, salvataggio e lettura |

## Crate che verranno aggiunte

| Crate       | Fase | Compito                                                                 |
| ----------- | ---- | ----------------------------------------------------------------------- |
| `engine/`   | 3    | Dialogo con i motori scacchistici (Stockfish, Maia)                     |
| `storage/`  | 4    | Il database: salvataggio di partite, mosse, progressi                   |
| `analysis/` | 5    | I rilevatori: minacce, tattiche, struttura dei pedoni, sicurezza del re |
| `training/` | 6    | Gli esercizi di calcolo e visualizzazione                               |
| `tutor/`    | 7    | Unisce analisi e conoscenza strategica e produce le spiegazioni         |
| `stats/`    | 9    | Il calcolo delle statistiche                                            |
