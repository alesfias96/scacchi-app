# crates/game-session — La partita in corso

`chess-core` conosce le regole degli scacchi. Questa crate aggiunge ciò che serve per **giocare
davvero** una partita:

- **l'avversario**: un'altra persona sulla stessa scacchiera, oppure il computer a uno dei dieci livelli;
- **l'orologio**: il tempo di ciascuno, l'incremento a ogni mossa, la sconfitta quando il tempo finisce
  (patta se l'avversario non ha più pezzi per dare matto, come da regolamento);
- **chi può fare cosa**: contro il computer si muovono solo i propri pezzi e solo dall'ultima posizione;
  con l'orologio le mosse non si ritirano; contro il computer "annulla" ritira anche la sua risposta;
- **l'abbandono**;
- **le richieste per il motore** quando tocca al computer: posizione, tempo, livello.

Non avvia il motore e non conosce Tauri: prepara solo le richieste. Il ponte `src-tauri` le manda
a Stockfish e riporta qui le sue mosse. Ogni richiesta ha un **contrassegno**: se nel frattempo la
partita è cambiata (mossa ritirata, nuova partita), la mossa del computer viene scartata.

Il tempo non viene mai letto "da solo": ogni funzione riceve l'istante attuale. Così i test
simulano minuti di partita in un attimo.

## Contenuto

| Elemento     | Cosa contiene                                                         |
| ------------ | --------------------------------------------------------------------- |
| `Cargo.toml` | Descrizione della crate e librerie usate                              |
| `src/`       | Il codice, un file per responsabilità (vedi il README della cartella) |
