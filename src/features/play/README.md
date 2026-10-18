# src/features/play — La sezione "Gioca"

La sezione dove si gioca. Dalla Fase 2 è una partita **a due giocatori sulla stessa
scacchiera**, con tutte le regole: arrocco, presa en passant, promozione (con scelta del
pezzo), scacco matto, stallo e patte automatiche (materiale insufficiente, triplice
ripetizione, regola delle 50 mosse). Dalla Fase 3 si potrà giocare contro il computer.

Ogni mossa ha il suo suono (colpo di legno; doppio colpo per le prese; melodia a fine
partita), spegnibile nelle Impostazioni: vedi `src/sounds/`.

Le regole stanno tutte in Rust: la pagina invia le mosse fatte sulla scacchiera e mostra
la partita aggiornata che riceve (stato in `src/state/gameStore.ts`).

## Come si usa

- Si muove trascinando un pezzo, oppure con un clic sul pezzo e uno sulla casa d'arrivo.
  Le case raggiungibili sono indicate da un pallino.
- ← → scorrono la partita mossa per mossa; Home e Fine vanno all'inizio e all'ultima mossa.
  Un clic su una mossa dell'elenco mostra la posizione dopo quella mossa.
- Se si gioca una mossa da una posizione passata, le mosse successive vengono sostituite
  (le varianti arriveranno nella Fase 8).

## Contenuto

| Elemento              | Cosa fa                                                                                   |
| --------------------- | ----------------------------------------------------------------------------------------- |
| `PlayPage.tsx`        | La pagina: collega scacchiera, schede, tastiera, suoni e promozione alla partita in corso |
| `PlayPage.module.css` | La disposizione della pagina (scacchiera grande, schede accanto o sotto)                  |
| `PlayPage.test.tsx`   | I test: attesa, errore, mosse, promozione, fine partita, tastiera, scacchiera girata      |
| `position-panel/`     | La scheda "Situazione": chi muove, mosse possibili, scacco                                |
| `move-list/`          | La scheda "Mosse": la partita scritta come nei libri                                      |
| `game-controls/`      | I comandi: scorri la partita, nuova partita, annulla, gira la scacchiera                  |
| `promotion-picker/`   | La scelta del pezzo quando un pedone arriva in fondo                                      |
| `game-result-banner/` | L'annuncio di fine partita (motivo, vincitore, punteggio)                                 |
| `import-export/`      | Copia di posizione (FEN) e partita (PGN), e finestra per incollarle                       |
