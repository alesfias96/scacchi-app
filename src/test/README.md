# src/test — Impostazioni comuni dei test

I test veri stanno accanto al file che verificano (es. `App.test.tsx` vicino a `App.tsx`).
Qui c'è solo ciò che vale per tutti i test.

## Contenuto

| File                   | Cosa fa                                                                                                                                                                    |
| ---------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `setup.ts`             | Eseguito prima dei test: pulisce la pagina simulata dopo ogni test, simula la modalità chiara/scura del sistema e lo scorrimento automatico (`scrollIntoView`, `scrollTo`) |
| `renderWithRouter.tsx` | Disegna un componente dentro un router di prova, per i componenti che contengono collegamenti tra sezioni                                                                  |
| `fakeAudioContext.ts`  | Un finto mixer audio per i test dei suoni: ricorda quali suoni sarebbero partiti e quando                                                                                  |
| `puzzleAttempts.ts`    | Due puzzle di prova (appena cominciato e risolto) per i test della sezione "Allenamento"                                                                                   |
| `tutorExplanations.ts` | Una spiegazione e una mossa provata di prova, per i test del tutor                                                                                                         |
| `ideas.ts`             | Le idee del tutor di prova (un'idea, le idee di una posizione, l'idea di una mossa con la migliore)                                                                        |
| `gameReviews.ts`       | Una revisione di prova, per i test della sezione «Analisi partite»                                                                                                         |
| `moveTrees.ts`         | Un albero di mosse con varianti, per i test della lista mosse                                                                                                              |
