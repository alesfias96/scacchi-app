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

| Comando                      | Cosa fa                                                                                                        |
| ---------------------------- | -------------------------------------------------------------------------------------------------------------- |
| `npm install`                | Scarica le librerie dell'interfaccia (da fare la prima volta)                                                  |
| `npm run engines`            | Scarica Stockfish per il sistema in uso (parte da solo con i comandi qui sotto; serve internet la prima volta) |
| `npm run tauri dev`          | Avvia l'app in modalità sviluppo, con ricaricamento automatico                                                 |
| `npm run tauri build`        | Crea l'installatore finale per il sistema in uso                                                               |
| `npm run build`              | Controlla e compila solo l'interfaccia                                                                         |
| `npm run check`              | **Esegue tutti i controlli di qualità e tutti i test** (deve essere verde prima di ogni consegna)              |
| `npm run format`             | Sistema automaticamente la formattazione dell'interfaccia                                                      |
| `npm run set-version 0.11.0` | Cambia il numero di versione in tutti i file che lo contengono (prima di pubblicare una versione nuova)        |

L'elenco completo dei controlli è in `PROGETTO.md`, sezione 7.1.

## Installare l'app

Gli installatori di ogni versione sono nella pagina **Releases** del progetto:
<https://github.com/alesfias96/scacchi-app/releases>.

- **Windows**: il file `…_x64-setup.exe`. Al primo avvio Windows può mostrare «Windows ha protetto il
  PC» (l'app non ha una firma a pagamento): si preme **Ulteriori informazioni**, poi **Esegui comunque**.
- **macOS**: il file `.dmg` (per i Mac con processore Apple). Al primo avvio: tasto destro sull'app
  → **Apri** → **Apri**.
- **Linux**: il file `.AppImage` (si rende eseguibile e si avvia) oppure il `.deb`.

Per creare l'installatore direttamente sul proprio computer: `npm run tauri build` (il file finisce
in `target/release/bundle/`).

## Aggiornamenti

Nelle **Impostazioni → Aggiornamenti** si vede la versione installata e si controlla se ce n'è una
nuova; se c'è, «Installa e riavvia» la scarica e la installa. Con «Controlla a ogni avvio» il
controllo si fa anche da solo all'apertura (è spento di default: l'app non si collega a Internet se
non lo si chiede).

Ogni aggiornamento è **firmato**: l'app contiene la chiave pubblica (in `src-tauri/tauri.conf.json`)
e installa solo i file firmati con la chiave privata corrispondente, che sta soltanto sul computer di
Alessandro (`C:\Users\afias\.tauri\scacchi.key`) e tra i segreti del repository su GitHub. Se la
chiave privata o la sua password si perdono, le app già installate non accettano più aggiornamenti
(bisognerebbe reinstallarle a mano con una chiave nuova).

## Pubblicare una versione nuova

Una volta sola, su GitHub: **Settings → Secrets and variables → Actions → New repository secret**:

- `TAURI_SIGNING_PRIVATE_KEY`: tutto il contenuto del file `scacchi.key`;
- `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`: la sua password.

A ogni versione:

1. `npm run set-version 0.11.0` (il numero nuovo), poi `npm run check`.
2. Commit e invio su GitHub (`git push`).
3. L'etichetta della versione: `git tag v0.11.0` e `git push origin v0.11.0`.
4. Su GitHub, scheda **Actions**, il flusso «Release» crea gli installatori dei tre sistemi
   (circa 20 minuti) in una **bozza** nella pagina **Releases**.
5. Si apre la bozza, si controlla e si preme **Publish release**: da quel momento le app installate
   trovano l'aggiornamento.
