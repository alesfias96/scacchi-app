# Scacchi

Applicazione desktop per **giocare, imparare e allenarsi a scacchi**, completamente offline.
Funziona su Windows, macOS e Linux.

L'obiettivo principale è allenare il **calcolo**: imparare a prevedere le mosse dell'avversario
e a ragionare più mosse avanti, con un tutor che spiega ogni situazione.

> Prima di modificare qualsiasi cosa, leggi **[PROGETTO.md](PROGETTO.md)**: contiene le regole,
> le decisioni prese e lo stato di avanzamento.
>
> Per riprendere il lavoro in una nuova sessione c'è **[CLAUDE.md](CLAUDE.md)**: in breve le regole,
> cosa è fatto, cosa resta e dove si era rimasti (Claude Code lo legge da solo all'avvio).

---

## Come è fatta (in parole semplici)

L'app è divisa in tre strati, come un ristorante:

| Strato      | Cartella                                                                            | Paragone                                                                          | Cosa fa                                                             |
| ----------- | ----------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- | ------------------------------------------------------------------- |
| Interfaccia | `src/`                                                                              | la sala                                                                           | Ciò che vedi e tocchi: scacchiera, pulsanti, pannelli               |
| Ponte       | `src-tauri/`                                                                        | il cameriere                                                                      | Porta le richieste dall'interfaccia al nucleo e riporta le risposte |
| Nucleo      | `crates/`                                                                           | Il nucleo: le librerie Rust con la logica (regole, motore, partita, impostazioni) |
| `scripts/`  | Comandi di servizio (ad esempio lo scaricamento di Stockfish, il motore di scacchi) |
| `data/`     | Dati inclusi nell'app, presi da fonti libere (l'elenco delle aperture di Lichess)   |

---

## Cartelle e file della radice

| Elemento                           | Cosa contiene                                                                       |
| ---------------------------------- | ----------------------------------------------------------------------------------- |
| `PROGETTO.md`                      | La memoria del progetto: regole di lavoro, decisioni, avanzamento                   |
| `README.md`                        | Questo file                                                                         |
| `src/`                             | L'interfaccia grafica (React + TypeScript)                                          |
| `src-tauri/`                       | L'app desktop vera e propria e il ponte verso il nucleo (Rust)                      |
| `crates/`                          | Il nucleo: le librerie Rust con la logica (per ora le regole degli scacchi)         |
| `.cargo/`                          | Impostazioni di Cargo (dove scrivere i tipi TypeScript generati da Rust)            |
| `index.html`                       | La pagina vuota dentro cui viene disegnata l'interfaccia                            |
| `package.json`                     | Elenco degli strumenti e librerie dell'interfaccia, e dei comandi disponibili       |
| `Cargo.toml`                       | Elenco delle parti Rust del progetto                                                |
| `vite.config.ts`                   | Impostazioni di Vite, lo strumento che prepara l'interfaccia                        |
| `tsconfig.json`                    | Collega le due configurazioni di TypeScript qui sotto                               |
| `tsconfig.app.json`                | Regole di controllo del codice TypeScript dell'interfaccia                          |
| `tsconfig.node.json`               | Regole di controllo per i file di configurazione (es. `vite.config.ts`)             |
| `.github/`                         | Controlli automatici eseguiti da GitHub su Windows, macOS e Linux                   |
| `eslint.config.js`                 | Regole del correttore del codice TypeScript/React (ESLint)                          |
| `.prettierrc.json`                 | Regole di formattazione automatica del codice (Prettier)                            |
| `.prettierignore`                  | File che Prettier non deve toccare                                                  |
| `vitest.config.ts`                 | Impostazioni dei test dell'interfaccia (Vitest)                                     |
| `rustfmt.toml`                     | Regole di formattazione del codice Rust                                             |
| `Cargo.lock` / `package-lock.json` | Versioni esatte delle librerie installate (generati automaticamente, vanno salvati) |
| `.gitignore`                       | File da non salvare nella cronologia del codice (generati automaticamente)          |
| `.gitattributes`                   | Uniforma le fine riga tra Windows, macOS e Linux                                    |
| `.editorconfig`                    | Regole di formattazione lette dagli editor                                          |

Cartelle che compaiono da sole e **non vanno modificate**: `node_modules/` (librerie scaricate),
`target/` (compilazioni Rust), `dist/` (interfaccia compilata).

---

## Requisiti

- **Node.js** 22.12 o successivo (consigliata la versione LTS)
- **Internet** solo la prima volta, per scaricare Stockfish (circa 80 MB); poi l'app funziona offline
- **Rust** stabile (tramite `rustup`)
- **Windows**: Microsoft C++ Build Tools e WebView2 (già presente su Windows 10/11)
- **macOS**: Xcode Command Line Tools
- **Linux**: le librerie di sistema indicate nella guida di Tauri (WebKitGTK e altre)

## Comandi principali

Da eseguire nella cartella del progetto:

| Comando               | Cosa fa                                                                                                        |
| --------------------- | -------------------------------------------------------------------------------------------------------------- |
| `npm install`         | Scarica le librerie dell'interfaccia (da fare la prima volta)                                                  |
| `npm run engines`     | Scarica Stockfish per il sistema in uso (parte da solo con i comandi qui sotto; serve internet la prima volta) |
| `npm run tauri dev`   | Avvia l'app in modalità sviluppo, con ricaricamento automatico                                                 |
| `npm run tauri build` | Crea l'installatore finale per il sistema in uso                                                               |
| `npm run build`       | Controlla e compila solo l'interfaccia                                                                         |
| `npm run check`       | **Esegue tutti i controlli di qualità e tutti i test** (deve essere verde prima di ogni consegna)              |
| `npm run format`      | Sistema automaticamente la formattazione dell'interfaccia                                                      |

L'elenco completo dei controlli è in `PROGETTO.md`, sezione 7.1.
