# src/features/play/game-result-banner — L'annuncio di fine partita

Quando la partita finisce, sopra le schede compare un riquadro con il motivo,
il vincitore e il punteggio:

| Motivo (da Rust)       | Testo mostrato                  | Punteggio      |
| ---------------------- | ------------------------------- | -------------- |
| `checkmate`            | Scacco matto · Vince il …       | 1-0 oppure 0-1 |
| `stalemate`            | Stallo · Patta                  | ½-½            |
| `insufficientMaterial` | Materiale insufficiente · Patta | ½-½            |
| `threefoldRepetition`  | Triplice ripetizione · Patta    | ½-½            |
| `fiftyMoveRule`        | Regola delle 50 mosse · Patta   | ½-½            |

| File                          | Cosa fa                                  |
| ----------------------------- | ---------------------------------------- |
| `GameResultBanner.tsx`        | Il componente                            |
| `GameResultBanner.module.css` | I suoi stili                             |
| `describeResult.ts`           | Trasforma il risultato di Rust in parole |
| `*.test.ts(x)`                | I test                                   |
