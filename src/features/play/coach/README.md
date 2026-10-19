# src/features/play/coach — La modalità Tutor

La sezione **Tutor** del menu principale è la pagina «Gioca» con i commenti del tutor. La partita è
la stessa (si passa da una sezione all'altra senza perderla). Dopo ogni mossa compaiono dei
**messaggi sotto la scacchiera, uno alla volta**, in una fascia di altezza fissa: la scacchiera,
un po' più piccola che in «Gioca», resta tutta visibile. Mentre un messaggio è aperto, la scacchiera grande
mostra le case e le frecce di cui parla.

L'ordine dei messaggi:

1. il **commento a ogni mossa appena giocata**: la propria e la risposta del computer, così la
   propria non va persa. Contiene il giudizio di Stockfish (se acceso) con la valutazione prima e
   dopo, la risposta migliore, e poi cosa la mossa concede, cosa crea, quali minacce risolve;
2. le **minacce e le tattiche** della posizione (sempre);
3. **solo le novità**: situazione, piano e principi già mostrati alle mosse precedenti non si
   ripetono. La prima volta si vede tutta la situazione.

Si scorre con «Indietro» e «Avanti» (o Invio e Spazio); Esc o «Fine» chiude. «Approfondisci» apre
la scheda della base di conoscenza nella scheda «Tutor» della colonna di destra, dove c'è anche
l'interruttore «Giudizio di Stockfish» (acceso di base). Quando i messaggi sono chiusi, nella
fascia compare «Rivedi i commenti».

## Contenuto

| File                         | Cosa fa                                                                                                                                                                                        |
| ---------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `buildCoachMessages.ts`      | Sceglie i messaggi, in ordine, senza ripetere quelli già visti                                                                                                                                 |
| `buildCoachMessages.test.ts` | I suoi test                                                                                                                                                                                    |
| `useCoachMessages.ts`        | Chiede a Rust commenti e spiegazione a ogni mossa, uno dopo l'altro                                                                                                                            |
| `useCoachPopup.ts`           | Quale messaggio è aperto; una mossa nuova riparte dal primo                                                                                                                                    |
| `CoachPopup.tsx`             | Il messaggio sotto la scacchiera, con i pulsanti e la tastiera                                                                                                                                 |
| `CoachPopup.module.css`      | I suoi stili: scheda nella fascia, con un leggero movimento quando compare                                                                                                                     |
| `CoachPopup.test.tsx`        | I suoi test                                                                                                                                                                                    |
| `CoachPanel.tsx`             | La scheda «Tutor» nella colonna di destra                                                                                                                                                      |
| `CoachPlanPanel.tsx`         | «Il piano» nella colonna di destra: il piano di chi muove e quello dell'avversario, con «Piano nuovo» quando cambia                                                                            |
| `CoachPlanPanel.module.css`  | I suoi stili                                                                                                                                                                                   |
| `CoachPlanPanel.test.tsx`    | I suoi test                                                                                                                                                                                    |
| `CoachIdeasPanel.tsx`        | Lo sguardo lungo nella colonna di destra: «L'idea della tua mossa» (con la mossa migliore e il passo del metodo che l'avrebbe trovata), «Le idee per continuare» e «Le strade dell'avversario» |
| `CoachIdeasPanel.module.css` | I suoi stili                                                                                                                                                                                   |
| `CoachIdeasPanel.test.tsx`   | I suoi test                                                                                                                                                                                    |
| `useCoachIdeas.ts`           | Dopo i commenti, chiede in sottofondo l'idea della tua ultima mossa e le idee della posizione                                                                                                  |
| `CoachPanel.module.css`      | I suoi stili                                                                                                                                                                                   |
