# src/components/chessboard — La scacchiera

Il componente che disegna la scacchiera, con la sua cornice. Usa **chessground**, la stessa
scacchiera di Lichess: pezzi che scivolano, coordinate, evidenziazione dello scacco e dell'ultima mossa,
trascinamento dei pezzi. Sopra ci sono i nostri segni (`themes/highlights.css`): pallini discreti
sulle case raggiungibili, anelli sui pezzi da catturare, la "mano" sui pezzi che si possono muovere,
il pezzo che si solleva con la sua ombra quando lo si trascina.

Può essere **in sola lettura** (come l'anteprima nelle Impostazioni) oppure
**interattiva** (come in Gioca): in quel caso il giocatore di turno può muovere solo verso
le case permesse dalle mosse legali calcolate da Rust, e la scacchiera comunica la mossa fatta.
Può anche disegnare una **freccia** (ad esempio la mossa migliore secondo il motore).

È **l'unica scacchiera dell'app**: ogni sezione (Gioca, Allenamento, Tutor, ...) usa questo
componente. Lo stile della scacchiera e dei pezzi non si passa come proprietà: il componente
lo legge da solo dalle impostazioni dell'utente. Così ogni scacchiera rispetta sempre le
scelte fatte nelle Impostazioni e si aggiorna subito quando cambiano.

Il componente è anche un **involucro**: il resto dell'app usa solo `Chessboard` e non sa che
sotto c'è chessground. Per cambiare libreria basterebbe modificare questa cartella.

## Contenuto

| Elemento                  | Cosa fa                                                                                                                            |
| ------------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| `Chessboard.tsx`          | Il componente: crea la scacchiera, la aggiorna quando la posizione cambia, la libera quando viene rimossa, applica lo stile scelto |
| `Chessboard.module.css`   | La cornice e la forma quadrata                                                                                                     |
| `Chessboard.test.tsx`     | I test, con chessground simulato                                                                                                   |
| `buildDests.ts`           | Trasforma le mosse legali di Rust nel formato di chessground (per ogni casa, dove può andare il pezzo)                             |
| `isSquareKey.ts`          | Controlla che un testo sia una casa valida (da a1 a h8)                                                                            |
| `prefersReducedMotion.ts` | Dice se il sistema chiede di ridurre le animazioni (allora i pezzi non scivolano)                                                  |
| `*.test.ts`               | I test delle funzioni qui sopra                                                                                                    |
| `themes/`                 | Gli stili delle scacchiere e dei set di pezzi (vedi il README della cartella)                                                      |

## Eccezione alla regola dei CSS Modules

`Chessboard.tsx` importa alcuni fogli di stile **globali**, perché chessground funziona così:

| File                    | Contenuto                                                                                                                                |
| ----------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| `chessground.base.css`  | La struttura della scacchiera (fornito da chessground)                                                                                   |
| `chessground.brown.css` | I colori di coordinate, ultima mossa e case evidenziate (fornito da chessground; le sue case marroni sono coperte dalle nostre immagini) |
| `themes/index.css`      | Le nostre scacchiere e i nostri set di pezzi                                                                                             |

Si importano **solo qui**. I due file di chessground non vanno modificati.

## Licenza

chessground è distribuito con licenza **GPL-3.0**: usarlo nell'app va benissimo, ma se un giorno
l'app venisse distribuita ad altri dovrebbe esserlo con una licenza compatibile (lo stesso vale per Stockfish).
