# src-tauri — L'app desktop e il ponte

Questa cartella trasforma l'interfaccia in una vera **applicazione desktop** per Windows, macOS e Linux,
grazie a **Tauri**. È scritta in **Rust**. Avvia anche **Stockfish**, il motore di scacchi.

Il suo ruolo è quello di un cameriere: riceve le richieste dall'interfaccia, le passa al nucleo
(cartella `crates/`) e riporta le risposte. **Non contiene logica di gioco.**

## Contenuto

| Elemento                  | Cosa fa                                                                                                                                                                                               |
| ------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Cargo.toml`              | Descrive questa parte Rust e le librerie che usa                                                                                                                                                      |
| `build.rs`                | Prepara la compilazione: legge la configurazione, controlla icone e permessi                                                                                                                          |
| `tauri.conf.json`         | Configurazione dell'app: nome e versione, finestra, icone, come avviare l'interfaccia, programmi esterni (Stockfish), protezione dell'interfaccia (CSP) e aggiornamenti (chiave pubblica e indirizzo) |
| `tauri.release.conf.json` | Le aggiunte per pubblicare una versione (usate dal flusso `release.yml`): crea anche i file firmati per gli aggiornamenti                                                                             |
| `src/`                    | Il codice Rust dell'app                                                                                                                                                                               |
| `capabilities/`           | I permessi: cosa l'interfaccia può chiedere al sistema                                                                                                                                                |
| `icons/`                  | Le icone dell'app per ogni sistema operativo                                                                                                                                                          |
| `binaries/`               | Il programma di Stockfish per il sistema in uso, scaricato da `npm run engines` (non salvato in git)                                                                                                  |

## La protezione dell'interfaccia (CSP)

La "Content Security Policy" dice all'interfaccia cosa può caricare. Qui è stretta: solo i file
dell'app (`'self'`), le immagini e i font piccoli che Vite include direttamente nel codice
(`data:`) e il collegamento con Rust (`ipc:`). Nessun indirizzo Internet, nessun codice da fuori.
Durante lo sviluppo (`devCsp`) si permettono anche gli stili che Vite inserisce al volo e il suo
collegamento per il ricaricamento automatico. Gli aggiornamenti non passano dall'interfaccia ma da
Rust, quindi la CSP non li riguarda.

## Cartelle generate automaticamente (non modificare)

| Cartella | Cosa contiene                                  |
| -------- | ---------------------------------------------- |
| `gen/`   | Schemi creati da Tauri durante la compilazione |
