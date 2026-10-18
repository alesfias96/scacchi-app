# src-tauri/src — Il codice Rust dell'app

## Contenuto

| Elemento     | Cosa fa                                                                                                |
| ------------ | ------------------------------------------------------------------------------------------------------ |
| `main.rs`    | Il punto di avvio del programma: si limita a chiamare `run()` in `lib.rs`                              |
| `lib.rs`     | Costruisce l'applicazione: apre la finestra, registra i comandi e le informazioni da tenere in memoria |
| `commands/`  | I comandi: le richieste che l'interfaccia può fare, un file per argomento                              |
| `state/`     | Le informazioni tenute in memoria mentre l'app è aperta (la partita in corso, i due motori Stockfish)  |
| `events/`    | I messaggi che Rust manda da solo all'interfaccia (mossa del computer, analisi, problemi del motore)   |
| `reactions/` | Ciò che succede da solo dopo ogni cambiamento della partita (mossa del computer, orologio, analisi)    |

Perché due file? `main.rs` è solo il "pulsante di accensione"; tutto il resto sta in `lib.rs`,
così il codice si può testare e riutilizzare senza avviare l'intera app.

## Come scorre una mossa contro il computer

1. L'interfaccia chiama il comando `play_move` (`commands/game.rs`).
2. Il comando gioca la mossa nella partita (`state/`) e risponde subito con la partita aggiornata.
3. Poi avvia le reazioni (`reactions/after_change.rs`): la partita dice che tocca al computer,
   quindi parte la richiesta a Stockfish, in un compito separato. L'interfaccia non aspetta.
4. Quando Stockfish risponde, la mossa viene giocata e l'interfaccia riceve l'evento
   `game-updated` (`events/`) con la partita nuova.
