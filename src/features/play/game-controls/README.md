# src/features/play/game-controls — I comandi della partita

I pulsanti sotto l'elenco delle mosse:

| Pulsante                 | Tasto | Cosa fa                                                                                 |
| ------------------------ | ----- | --------------------------------------------------------------------------------------- |
| ⏮ Posizione iniziale     | Home  | Mostra la posizione prima della prima mossa                                             |
| ◀ Mossa precedente       | ←     | Torna indietro di una mossa                                                             |
| ▶ Mossa successiva       | →     | Va avanti di una mossa                                                                  |
| ⏭ Ultima mossa           | Fine  | Mostra la posizione attuale                                                             |
| Nuova partita            |       | Apre la finestra "Nuova partita"                                                        |
| ↶ Annulla l'ultima mossa |       | Ritira l'ultima mossa (contro il computer anche la sua risposta); spento con l'orologio |
| ⚑ Abbandona              |       | Chiude la partita dandola vinta all'avversario, dopo una conferma                       |
| ⇅ Gira la scacchiera     |       | Mostra la scacchiera dal punto di vista dell'altro colore                               |

Tornare indietro non cancella niente. A due giocatori senza orologio, se si gioca una mossa da
una posizione passata le mosse successive vengono sostituite dalla nuova; contro il computer o
con l'orologio si gioca solo dall'ultima posizione. Quali pulsanti sono attivi lo decide Rust.

| File                      | Cosa fa       |
| ------------------------- | ------------- |
| `GameControls.tsx`        | Il componente |
| `GameControls.module.css` | I suoi stili  |
| `GameControls.test.tsx`   | I suoi test   |
