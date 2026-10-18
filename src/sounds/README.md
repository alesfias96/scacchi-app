# src/sounds — I suoni del gioco

I suoni che accompagnano la partita: il **colpo di legno** di una mossa, il **doppio colpo**
di una presa e la **melodia di fine partita** (quattro note che salgono per lo scacco
matto, due note calme per la patta).

Non sono file registrati: l'app li **costruisce al momento** con la Web Audio API, che
esiste nel motore web di Windows, macOS e Linux. Così non servono file da scaricare né
licenze, funzionano offline e il timbro si regola cambiando pochi numeri in questi file.
Si possono spegnere nelle Impostazioni ("Suoni").

Un colpo di legno è fatto di due parti: un **clic** brevissimo (fruscio filtrato, il
contatto tra legno e legno) e un **corpo** grave che si spegne subito (la risonanza della
tavola). Le note di fine partita sono note morbide, simili a una marimba.

## Contenuto

| File                  | Cosa fa                                                                               |
| --------------------- | ------------------------------------------------------------------------------------- |
| `getSoundOutput.ts`   | Il "mixer" audio unico dell'app, con il volume generale; `null` se non si può suonare |
| `getNoiseBuffer.ts`   | Il fruscio (rumore bianco) da cui nasce il clic del legno, calcolato una volta sola   |
| `playWoodTap.ts`      | Un colpo di legno, con tono e intensità a scelta                                      |
| `playSoftNote.ts`     | Una nota morbida (nota principale più due armoniche)                                  |
| `playMoveSound.ts`    | Il suono della mossa: un colpo                                                        |
| `playCaptureSound.ts` | Il suono della presa: un colpo leggero e acuto seguito da uno grave e deciso          |
| `playGameEndSound.ts` | La melodia di fine partita: decisa (matto) o patta                                    |
| `pickGameSounds.ts`   | Decide quali suoni far sentire quando la partita mostrata cambia                      |
| `*.test.ts`           | I test di ogni file, con un finto mixer audio (`src/test/fakeAudioContext.ts`)        |

## Quando si sentono

Solo quando la scacchiera avanza di **una** mossa nella stessa partita: una mossa appena
giocata, oppure un passo avanti con → o ▶. Tornando indietro, saltando a inizio o fine,
iniziando una nuova partita o incollandone una non si sente nulla. Il collegamento con la
partita è in `src/hooks/useMoveSound.ts`.

Se una mossa è una presa lo dice Rust (campo `capture` di ogni mossa): l'interfaccia non
indovina nulla dalle regole.
