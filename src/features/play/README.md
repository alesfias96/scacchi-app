# src/features/play — La sezione "Gioca"

La sezione dove si gioca, **contro il computer** (Stockfish, a dieci livelli) oppure **a due
giocatori sulla stessa scacchiera**, con o senza orologio. Valgono tutte le regole: arrocco,
presa en passant, promozione (con scelta del pezzo), scacco matto, stallo, patte automatiche
(materiale insufficiente, triplice ripetizione, regola delle 50 mosse), tempo scaduto e abbandono.

Le regole stanno tutte in Rust: la pagina invia le richieste e mostra la partita aggiornata che
riceve (stato in `src/state/gameStore.ts`). La partita può anche cambiare da sola: la mossa del
computer e il tempo scaduto arrivano da Rust come eventi.

Ogni mossa ha il suo suono (colpo di legno; doppio colpo per le prese; melodia a fine
partita), spegnibile nelle Impostazioni: vedi `src/sounds/`.

Ogni partita si salva da sola nell'archivio a ogni mossa (sezione "Archivio"); all'avvio
l'ultima partita non finita riappare qui, con l'orologio fermo fino alla mossa successiva.

## Come si usa

- A sinistra la **scacchiera**, con i giocatori, l'orologio e (se accesa) la barra di valutazione
  accanto, alta quanto lei. La scacchiera è grande quanto permette l'altezza della finestra e resta
  sempre tutta visibile. A destra la colonna **"Partita"** con tutte le schede (mosse, analisi,
  osservazioni, situazione, copia e incolla, file), sempre aperta e con il suo scorrimento interno.
  Il menu principale delle sezioni è a comparsa sul bordo sinistro (tasto **M**). A fine partita
  l'annuncio compare sopra la scacchiera.

- **Nuova partita** apre la finestra in cui si sceglie l'avversario, il colore, il livello del
  computer e il tempo. La finestra ripropone le scelte dell'ultima partita.
- Si muove trascinando un pezzo (il cursore diventa una mano sui pezzi che si possono muovere),
  oppure con un clic sul pezzo e uno sulla casa d'arrivo. Le case raggiungibili hanno un pallino.
- Contro il computer, mentre lui pensa la scacchiera è bloccata e la sua striscia dice "sta pensando".
- ← → scorrono la partita mossa per mossa; Home e Fine vanno all'inizio e all'ultima mossa.
- **Analisi**: l'interruttore nella scheda "Analisi" accende Stockfish sulla posizione mostrata:
  barra di valutazione accanto alla scacchiera, freccia della mossa migliore, variante principale.
  Contro il computer parte spenta, perché sarebbe un aiuto.
- **Osservazioni**: l'interruttore nella scheda "Osservazioni" accende i rilevatori, che elencano
  minacce, tattiche, pedoni, re e pezzi; passando il mouse su un fatto, la scacchiera lo mostra.
  Anche questa contro il computer parte spenta.
- A due giocatori senza orologio si può rigiocare da una posizione passata (le mosse successive
  vengono sostituite; le varianti arriveranno nella Fase 8). Contro il computer o con l'orologio
  si gioca solo dall'ultima posizione.

## Contenuto

| Elemento              | Cosa fa                                                                                                                             |
| --------------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| `PlayPage.tsx`        | La pagina: la scacchiera con giocatori, barra di valutazione e annuncio di fine partita, e la colonna "Partita" con tutte le schede |
| `PlayPage.module.css` | La disposizione: scacchiera a sinistra (grande quanto la finestra permette), colonna "Partita" a destra con lo scorrimento interno  |
| `PlayPage.test.tsx`   | I test: attesa, errore, mosse, promozione, fine partita, computer che pensa, nuova partita, analisi, tastiera                       |
| `player-bar/`         | La striscia di un giocatore sopra o sotto la scacchiera: nome, livello, "sta pensando", orologio                                    |
| `clock-face/`         | Il quadrante dell'orologio, con i secondi che scorrono                                                                              |
| `new-game-dialog/`    | La finestra "Nuova partita": avversario, colore, livello, tempo                                                                     |
| `analysis-panel/`     | La scheda "Analisi": interruttore, valutazione, profondità, variante                                                                |
| `observations-panel/` | La scheda "Osservazioni": i fatti dei rilevatori, per gruppo; passandoci sopra si evidenziano sulla scacchiera                      |
| `position-panel/`     | La scheda "Situazione": chi muove, mosse possibili, scacco                                                                          |
| `move-list/`          | La scheda "Mosse": la partita scritta come nei libri                                                                                |
| `game-controls/`      | I comandi: scorri la partita, nuova partita, annulla, abbandona, gira la scacchiera                                                 |
| `promotion-picker/`   | La scelta del pezzo quando un pedone arriva in fondo                                                                                |
| `game-result-banner/` | L'annuncio di fine partita (motivo, vincitore, punteggio)                                                                           |
| `import-export/`      | Copia di posizione (FEN) e partita (PGN), e finestra per incollarle                                                                 |
| `pgn-file/`           | Salvare la partita in un file `.pgn` e aprirne uno, con la finestra del sistema                                                     |
| `opening-label/`      | Il nome dell'apertura giocata, sopra l'elenco delle mosse                                                                           |
