# src/features/play/game-controls — I comandi della partita

I pulsanti sotto l'elenco delle mosse:

| Pulsante                 | Tasto | Cosa fa                                                   |
| ------------------------ | ----- | --------------------------------------------------------- |
| ⏮ Posizione iniziale     | Home  | Mostra la posizione prima della prima mossa               |
| ◀ Mossa precedente       | ←     | Torna indietro di una mossa                               |
| ▶ Mossa successiva       | →     | Va avanti di una mossa                                    |
| ⏭ Ultima mossa           | Fine  | Mostra la posizione attuale                               |
| Nuova partita            |       | Ricomincia dalla posizione iniziale                       |
| ↶ Annulla l'ultima mossa |       | Ritira l'ultima mossa della partita                       |
| ⇅ Gira la scacchiera     |       | Mostra la scacchiera dal punto di vista dell'altro colore |

Tornare indietro non cancella niente: se però si gioca una mossa da una
posizione passata, le mosse successive vengono sostituite dalla nuova.

| File                      | Cosa fa       |
| ------------------------- | ------------- |
| `GameControls.tsx`        | Il componente |
| `GameControls.module.css` | I suoi stili  |
| `GameControls.test.tsx`   | I suoi test   |
