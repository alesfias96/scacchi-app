# src/features/training/puzzle-status — La scheda "Puzzle"

A che punto è il puzzle: il **messaggio** (la consegna all'inizio, poi "Giusto!", "Non è la mossa
giusta: riprova", "Risolto!"), quante mosse sono state trovate, le **mosse giocate** dall'inizio del
puzzle (nella visualizzazione sono l'unico modo di seguire la partita) e i pulsanti _Aiuto_, _Mostra
soluzione_ e, a puzzle finito, _Prossimo puzzle_.

| File                       | Cosa fa                                                        |
| -------------------------- | -------------------------------------------------------------- |
| `PuzzleStatus.tsx`         | Il componente                                                  |
| `PuzzleStatus.module.css`  | I suoi stili (verde per le mosse giuste, rosso per gli errori) |
| `describeFeedback.ts`      | Sceglie il messaggio e il suo colore                           |
| `describeFeedback.test.ts` | I suoi test                                                    |

L'aiuto è a gradini: il primo dice il tema a parole (es. «Cerca una mossa che attacchi due pezzi
insieme»), il secondo («Altro aiuto») cerchia il pezzo da muovere.
