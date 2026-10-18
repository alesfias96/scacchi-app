# src/features/play/player-bar — La striscia di un giocatore

Sopra e sotto la scacchiera c'è una striscia per ciascun giocatore: in basso il colore mostrato
in basso sulla scacchiera, in alto l'avversario. Mostra:

- il colore (cerchio bianco o nero) e il nome: "Tu" e "Stockfish" contro il computer, "Bianco" e
  "Nero" a due giocatori;
- per il computer il livello, con la forza indicativa (es. "Circolo · ~1400 Elo");
- tre puntini e "sta pensando" mentre il computer cerca la sua mossa;
- l'orologio, se la partita ce l'ha (vedi `clock-face/`).

| File                   | Cosa fa                                                          |
| ---------------------- | ---------------------------------------------------------------- |
| `PlayerBar.tsx`        | Il componente                                                    |
| `PlayerBar.module.css` | I suoi stili (compresi i puntini animati)                        |
| `describePlayer.ts`    | Prepara nome, dettaglio, orologio e "sta pensando" dalla partita |
| `*.test.ts(x)`         | I test                                                           |
