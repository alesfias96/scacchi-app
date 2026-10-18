# crates/engine — Il dialogo con i motori di scacchi

Un **motore** è un programma che calcola le mosse, come **Stockfish**. Si parla con lui a righe di
testo, secondo il protocollo **UCI** (Universal Chess Interface):

```
app    → position fen <posizione> moves e2e4       "ecco la posizione"
app    → go movetime 500                           "pensa mezzo secondo"
motore → info depth 12 score cp 31 pv e7e5 g1f3    "a profondità 12 il Nero sta 0,31 peggio…"
motore → bestmove e7e5                             "gioco e5"
```

Questa crate fa tutto il dialogo: avvia il programma, gli manda le richieste e traduce le risposte.
È **asincrona**: chi chiede una mossa non resta bloccato ad aspettare, e una nuova richiesta
interrompe quella vecchia (il motore riceve `stop`). Non sa nulla di Tauri né dell'interfaccia.

## Cosa offre

- **Analisi continua** di una posizione: a ogni profondità completata arriva un `AnalysisUpdate`
  (valutazione dal punto di vista del Bianco, profondità, variante migliore già in notazione SAN).
- **Ricerca della mossa** da giocare, con i limiti di tempo (fisso o con l'orologio) e il livello.
- **Dieci livelli** di gioco, da Principiante (~800 Elo) a Sovrumano (piena forza): sopra i 1320
  Elo si usa l'impostazione ufficiale `UCI_Elo`, sotto lo `Skill Level` con poca profondità.
- I valori delle impostazioni vengono sempre riportati entro i limiti che il motore dichiara.

## Contenuto

| Elemento     | Cosa contiene                                                              |
| ------------ | -------------------------------------------------------------------------- |
| `Cargo.toml` | Descrizione della crate e librerie usate (tokio per l'asincronia)          |
| `src/`       | Il codice, un file per responsabilità (vedi il README della cartella)      |
| `tests/`     | La prova con il vero Stockfish (`real_stockfish.rs`), se è stato scaricato |

## Test

`npm run test:rust` esegue anche questi test. Quasi tutti usano un **finto motore** in memoria
(`src/fake_engine.rs`), che risponde come Stockfish ma all'istante. La prova in `tests/` usa il
vero Stockfish scaricato in `src-tauri/binaries/` e verifica che trovi un matto in una mossa.
