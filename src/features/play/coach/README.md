# src/features/play/coach — La modalità Tutor

La sezione **Tutor** del menu principale è la pagina «Gioca» con i commenti del tutor. La partita è
la stessa (si passa da una sezione all'altra senza perderla). Dopo ogni mossa compaiono dei
**messaggi sopra la scacchiera, uno alla volta**. Mentre un messaggio è aperto, la scacchiera grande
mostra le case e le frecce di cui parla.

L'ordine dei messaggi:

1. il **commento a ogni mossa appena giocata**: la propria e la risposta del computer, così la
   propria non va persa. Contiene il giudizio di Stockfish (se acceso) con la valutazione prima e
   dopo, la risposta migliore, e poi cosa la mossa concede, cosa crea, quali minacce risolve;
2. le **minacce e le tattiche** della posizione (sempre);
3. **solo le novità**: situazione, piano e principi già mostrati alle mosse precedenti non si
   ripetono. La prima volta si vede tutta la situazione.

Si scorre con «Indietro» e «Avanti» (o Invio e Spazio); Esc o «Fine» chiude. «Approfondisci» apre
la scheda della base di conoscenza nella scheda «Tutor» a destra. «Sposta in alto» sposta il
messaggio se copre la zona interessante. Nella scheda «Tutor» ci sono anche l'interruttore
«Giudizio di Stockfish» (acceso di base) e «Rivedi i commenti».

## Contenuto

| File                         | Cosa fa                                                                |
| ---------------------------- | ---------------------------------------------------------------------- |
| `buildCoachMessages.ts`      | Sceglie i messaggi, in ordine, senza ripetere quelli già visti         |
| `buildCoachMessages.test.ts` | I suoi test                                                            |
| `useCoachMessages.ts`        | Chiede a Rust commenti e spiegazione a ogni mossa, uno dopo l'altro    |
| `useCoachPopup.ts`           | Quale messaggio è aperto; una mossa nuova riparte dal primo            |
| `CoachPopup.tsx`             | Il messaggio sopra la scacchiera, con i pulsanti e la tastiera         |
| `CoachPopup.module.css`      | I suoi stili: semitrasparente, con un leggero movimento quando compare |
| `CoachPopup.test.tsx`        | I suoi test                                                            |
| `CoachPanel.tsx`             | La scheda «Tutor» nella colonna di destra                              |
| `CoachPanel.module.css`      | I suoi stili                                                           |
