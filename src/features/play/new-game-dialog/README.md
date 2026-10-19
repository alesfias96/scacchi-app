# src/features/play/new-game-dialog — La finestra "Nuova partita"

Si apre con il pulsante "Nuova partita" e chiede come si vuole giocare:

| Scelta        | Possibilità                                                                                                                                                                 |
| ------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Avversario    | Computer (Stockfish), Umano (Maia: gioca come una persona di quel livello) oppure due giocatori sulla stessa scacchiera                                                     |
| Il tuo colore | Bianco, Nero o a sorte (solo contro il computer)                                                                                                                            |
| Livello       | Stockfish: dieci livelli, da Principiante (~800 Elo) a Sovrumano (piena forza). Maia: nove livelli, da 1100 a 1900 Elo. Ogni motore ricorda il suo; l'elenco arriva da Rust |
| Tempo         | Senza orologio; Bullet 1+0, 2+1; Blitz 3+0, 3+2, 5+0, 5+3; Rapid 10+0, 10+5, 15+10; Classica 30+0, 30+20; Personalizzato                                                    |

"5+3" vuol dire 5 minuti a testa, più 3 secondi aggiunti a ogni mossa giocata.
La finestra ripropone le scelte dell'ultima partita, che vengono salvate con le impostazioni.
Esc o "Annulla" chiudono senza cambiare nulla.

| File                       | Cosa fa                                                        |
| -------------------------- | -------------------------------------------------------------- |
| `NewGameDialog.tsx`        | La finestra                                                    |
| `NewGameDialog.module.css` | I suoi stili                                                   |
| `timePresets.ts`           | I tempi proposti e come riconoscere quello dell'ultima partita |
| `*.test.ts(x)`             | I test                                                         |
