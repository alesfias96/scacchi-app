# crates/chess-core — Le regole degli scacchi

Il "cervello" che conosce le regole. Sa:

- creare la posizione iniziale, oppure una posizione descritta in notazione **FEN**
  (il modo standard di scrivere una posizione come testo);
- calcolare tutte le **mosse legali** del giocatore di turno;
- capire se il re è sotto **scacco**;
- tenere una **partita**: giocare le mosse (arrocco, en passant e promozione compresi),
  annullarle, tornare a una posizione passata;
- capire quando la partita è **finita**: scacco matto, stallo, materiale insufficiente,
  triplice ripetizione, regola delle 50 mosse;
- scrivere e leggere le partite in formato **PGN** (il formato standard delle partite).

Il lavoro pesante sulle regole lo fa la libreria `shakmaty` (veloce e collaudata).
Questa crate la avvolge e restituisce dati semplici, pronti per l'interfaccia.
Non sa nulla di grafica né di Tauri.

## Contenuto

| Elemento     | Cosa contiene                                                         |
| ------------ | --------------------------------------------------------------------- |
| `Cargo.toml` | Descrizione della crate e librerie usate                              |
| `src/`       | Il codice, un file per responsabilità (vedi il README della cartella) |

## L'opzione `bindings`

Normalmente la crate è pura logica. Con l'opzione `bindings` (attivata da `src-tauri`):

- i tipi pubblici diventano convertibili in JSON, per essere inviati all'interfaccia;
- i test generano automaticamente i tipi TypeScript corrispondenti in `src/bindings/`.

## Test

`npm run test:rust` esegue i test di questa crate insieme a tutti gli altri. Verificano tra l'altro che:

- la posizione iniziale abbia esattamente 20 mosse legali;
- una FEN scritta male o una posizione impossibile producano un errore chiaro, senza crash;
- una posizione di scacco matto abbia 0 mosse legali;
- l'arrocco sia descritto come mossa del re (e1 → g1);
- arrocco, en passant e promozione (anche sotto-promozione) funzionino;
- ogni modo di finire la partita venga riconosciuto (matto, stallo, materiale insufficiente,
  triplice ripetizione, 50 mosse);
- una partita scritta in PGN e riletta sia identica all'originale.
