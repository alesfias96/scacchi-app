# src/features/archive/game-row — Una partita nell'elenco

Una riga per partita: a sinistra un'etichetta colorata con l'esito (verde **Vinta**, rossa **Persa**,
grigia **Patta** o "Vince il …", nel colore d'accento **In corso**), al centro apertura, data,
avversario, tempo e numero di mosse, a destra i pulsanti **Rivedi**/**Continua** ed **Elimina**
(che chiede conferma).

| File                 | Cosa fa                                                                      |
| -------------------- | ---------------------------------------------------------------------------- |
| `GameRow.tsx`        | Il componente                                                                |
| `GameRow.module.css` | I suoi stili                                                                 |
| `describeOutcome.ts` | L'esito in una parola (dal punto di vista della persona, contro il computer) |
| `*.test.ts(x)`       | I test                                                                       |
