# src/components/chessboard/themes — Stili di scacchiere e pezzi

Fogli di stile che collegano le scelte dell'utente alle immagini in `src/assets/`.
La scacchiera ha due attributi, impostati da `Chessboard.tsx` in base alle impostazioni:

- `data-board` (es. `rovere`) sceglie l'immagine delle case e il colore della cornice;
- `data-pieces` (es. `icona`) sceglie i disegni dei pezzi.

## Contenuto

| File                  | Cosa fa                                                                                                               |
| --------------------- | --------------------------------------------------------------------------------------------------------------------- |
| `index.css`           | Raccoglie tutti gli altri file; è l'unico importato (da `Chessboard.tsx`)                                             |
| `boards.css`          | Le 6 scacchiere: immagine delle case e colore della cornice                                                           |
| `highlights.css`      | I segni sopra le case (case raggiungibili, ultima mossa, scacco) e il movimento dei pezzi (mano, sollevamento, ombra) |
| `pieces-icona.css`    | Collega i 12 pezzi dello stile Icona alle loro immagini                                                               |
| `pieces-morbido.css`  | Idem per lo stile Morbido                                                                                             |
| `pieces-legno.css`    | Idem per lo stile Legno                                                                                               |
| `pieces-metallo.css`  | Idem per lo stile Metallo                                                                                             |
| `pieces-gettoni.css`  | Idem per lo stile Gettoni                                                                                             |
| `pieces-classico.css` | Idem per lo stile Classico                                                                                            |

## Come si aggiunge uno stile

1. Si aggiungono le immagini in `src/assets/boards/` o in una nuova cartella di `src/assets/pieces/`.
2. Si aggiunge qui la regola (o il nuovo file `pieces-*.css`, importato in `index.css`).
3. Si aggiunge il valore in Rust (`crates/app-settings/`) e la voce in `src/features/settings/options/`.
