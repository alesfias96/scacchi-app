# src-tauri/src — Il codice Rust dell'app

## Contenuto

| Elemento    | Cosa fa                                                                                         |
| ----------- | ----------------------------------------------------------------------------------------------- |
| `main.rs`   | Il punto di avvio del programma: si limita a chiamare `run()` in `lib.rs`                       |
| `lib.rs`    | Costruisce l'applicazione: apre la finestra e registra i comandi che l'interfaccia può chiamare |
| `commands/` | I comandi: le richieste che l'interfaccia può fare, un file per argomento                       |
| `state/`    | Le informazioni tenute in memoria mentre l'app è aperta (la partita in corso)                   |

Perché due file? `main.rs` è solo il "pulsante di accensione"; tutto il resto sta in `lib.rs`,
così il codice si può testare e riutilizzare senza avviare l'intera app.

## Cartelle che verranno aggiunte

| Cartella  | Fase | Scopo                                                                        |
| --------- | ---- | ---------------------------------------------------------------------------- |
| `events/` | 3    | I messaggi inviati in tempo reale all'interfaccia (es. l'analisi del motore) |
