# src-tauri — L'app desktop e il ponte

Questa cartella trasforma l'interfaccia in una vera **applicazione desktop** per Windows, macOS e Linux,
grazie a **Tauri**. È scritta in **Rust**. Avvia anche **Stockfish**, il motore di scacchi.

Il suo ruolo è quello di un cameriere: riceve le richieste dall'interfaccia, le passa al nucleo
(cartella `crates/`) e riporta le risposte. **Non contiene logica di gioco.**

## Contenuto

| Elemento          | Cosa fa                                                                                                                    |
| ----------------- | -------------------------------------------------------------------------------------------------------------------------- |
| `Cargo.toml`      | Descrive questa parte Rust e le librerie che usa                                                                           |
| `build.rs`        | Prepara la compilazione: legge la configurazione, controlla icone e permessi                                               |
| `tauri.conf.json` | Configurazione dell'app: nome, dimensioni della finestra, icone, come avviare l'interfaccia, programmi esterni (Stockfish) |
| `src/`            | Il codice Rust dell'app                                                                                                    |
| `capabilities/`   | I permessi: cosa l'interfaccia può chiedere al sistema                                                                     |
| `icons/`          | Le icone dell'app per ogni sistema operativo                                                                               |
| `binaries/`       | Il programma di Stockfish per il sistema in uso, scaricato da `npm run engines` (non salvato in git)                       |

## Cartelle generate automaticamente (non modificare)

| Cartella | Cosa contiene                                  |
| -------- | ---------------------------------------------- |
| `gen/`   | Schemi creati da Tauri durante la compilazione |
