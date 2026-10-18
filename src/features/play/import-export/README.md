# src/features/play/import-export — Copia e incolla di posizioni e partite

| Pulsante  | Cosa fa                                                                  |
| --------- | ------------------------------------------------------------------------ |
| Copia FEN | Copia negli appunti la posizione mostrata                                |
| Copia PGN | Copia negli appunti l'intera partita                                     |
| Incolla   | Apre una finestra dove incollare una posizione (FEN) o una partita (PGN) |

**FEN** e **PGN** sono i due formati "universali" degli scacchi: li capiscono
tutti i programmi e i siti (Lichess, Chess.com, ChessBase, …).

- **FEN** descrive una sola posizione, in una riga
  (es. `rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1`).
- **PGN** descrive una partita intera: intestazioni (giocatori, data, …) e mosse
  (es. `1. e4 e5 2. Nf3 Nc6 *`).

Nella finestra "Incolla" l'app riconosce da sola quale dei due formati è stato
incollato. Commenti, varianti e simboli come `!?` nel PGN vengono ignorati.
Il salvataggio delle partite su file `.pgn` arriverà nella Fase 4.

| File               | Cosa fa                                   |
| ------------------ | ----------------------------------------- |
| `ImportExport.tsx` | I tre pulsanti e la conferma della copia  |
| `ImportDialog.tsx` | La finestra "Incolla posizione o partita" |
| `*.module.css`     | Gli stili di ciascun componente           |
| `*.test.tsx`       | I test                                    |
