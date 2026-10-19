# src/hooks — Hook React condivisi

Gli "hook" sono piccole funzioni che danno ai componenti un'informazione sempre aggiornata.
Qui stanno quelli usati da più parti dell'app.

## Contenuto

| File                          | Cosa fa                                                                                                                                  |
| ----------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| `useColorSchemePreference.ts` | Dice se Windows/macOS/Linux è in modalità scura, e si aggiorna se l'utente la cambia. Serve al tema "Automatico"                         |
| `useInitialPosition.ts`       | Chiede a Rust la posizione iniziale e tiene traccia della richiesta (in attesa, pronta, errore). Usato dall'anteprima nelle Impostazioni |
| `useGameKeyboard.ts`          | Scorre la partita con la tastiera: ← → per le mosse, Home e Fine per inizio e fine. Ignora i tasti mentre si scrive in un campo          |
| `useTutorTarget.ts`           | Una sezione dice al tutor quale posizione sta mostrando (e quando il tutor deve restare spento)                                          |
| `useTutorTab.ts`              | La voce scelta in cima alla colonna di destra (contenuto della sezione o «Tutor»); il tasto T passa dall'una all'altra                   |
| `useMoveSound.ts`             | Fa sentire i suoni della partita (mossa, presa, fine partita) quando la scacchiera avanza di una mossa, se i suoni sono attivi           |
| `useGameKeyboard.test.tsx`    | I test della tastiera                                                                                                                    |
| `useMoveSound.test.tsx`       | I test dei suoni della partita                                                                                                           |
| `useCountdown.ts`             | Fa scorrere i secondi dell'orologio sullo schermo tra un aggiornamento di Rust e l'altro                                                 |
| `useCountdown.test.tsx`       | I suoi test                                                                                                                              |
| `useEngineLevels.ts`          | Chiede a Rust i livelli di un motore (i dieci di Stockfish o i nove di Maia), una volta sola                                             |
| `usePositionReport.ts`        | Chiede a Rust i fatti dei rilevatori a ogni cambio di posizione, quando le osservazioni sono accese                                      |
| `usePositionReport.test.tsx`  | I suoi test                                                                                                                              |
