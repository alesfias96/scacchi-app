# PROGETTO.md — Memoria del progetto

> **Versione 1.0 — approvata il 2026-10-18.** Questo file è la memoria permanente del progetto.
> Va riletto **prima di ogni modifica**, da chiunque lavori sul codice (persone o AI).
> Quando una decisione cambia, si aggiorna qui e si annota nel registro in fondo.

---

## 1. Cos'è l'app

Un'applicazione desktop, completamente offline, per **giocare, imparare e allenarsi a scacchi**.
Funziona su **Windows, macOS e Linux** con lo stesso codice.

Obiettivi, in ordine di importanza:

1. **Allenare il calcolo**: imparare a prevedere le mosse dell'avversario e a ragionare più mosse avanti.
2. **Tutor "master"**: spiega ogni situazione nella sua interezza (minacce, piani, scenari possibili, strategie col bianco e col nero), sempre richiamabile in qualsiasi schermata e in qualsiasi partita.
3. **Giocare** contro il computer, da livello principiante fino a livello sovrumano.
4. **Statistiche avanzate** su tutte le variabili utili al miglioramento.
5. Interfaccia **moderna, reattiva e interattiva**: niente si blocca mai mentre il motore pensa.

---

## 2. Regole di lavoro (obbligatorie)

1. **Prima analisi, poi piano, poi autorizzazione.** Prima di modificare qualsiasi cosa: analisi completa della parte coinvolta e piano d'azione con le strade possibili. **Nessuna modifica finché non arriva l'autorizzazione esplicita.**
2. **Una responsabilità per file.** Ogni file fa una sola cosa precisa. Ogni componente sta nel suo file. Se un file inizia a fare due cose, si divide.
3. **Un README in ogni cartella.** Scritto in modo comprensibile anche a chi non sa programmare: spiega a cosa serve la cartella e cosa fa ogni file contenuto.
4. **Commenti esaustivi.** Ogni funzione, componente, struttura dati e modulo ha un commento che spiega cosa fa, cosa riceve, cosa restituisce e perché esiste.
5. **Il codice morto si cancella.** Codice sostituito o non più usato va eliminato, non commentato. Eccezioni: richiesta esplicita, oppure se la cancellazione romperebbe l'applicazione (in quel caso si annota il motivo).
6. **Una sola metodologia CSS** (vedi §5). Nessuno stile fuori da questo sistema.
7. **Una sola fonte di verità.** Le regole degli scacchi, i tipi di dati e la logica vivono in Rust. L'interfaccia mostra e raccoglie input, non decide.
8. **Questo file si aggiorna** a ogni decisione architetturale.
9. **Multipiattaforma sempre** (vedi §5-bis). Nessuna funzionalità può dipendere da un solo sistema operativo.
10. **Ogni passo ha i suoi test.** Si passa al successivo solo quando tutti i test sono verdi.

---

## 3. Stack tecnologico

| Ambito                   | Scelta                                                                               | Stato                                                  |
| ------------------------ | ------------------------------------------------------------------------------------ | ------------------------------------------------------ |
| Contenitore desktop      | Tauri 2                                                                              | ✅ deciso                                              |
| Logica, motore, dati     | Rust                                                                                 | ✅ deciso                                              |
| Interfaccia              | React 19 + TypeScript                                                                | ✅ deciso                                              |
| Build interfaccia        | Vite                                                                                 | ✅ deciso                                              |
| Regole degli scacchi     | `shakmaty` (Rust)                                                                    | ✅ deciso                                              |
| Motore forte             | Stockfish nativo (sidecar Tauri)                                                     | ✅ deciso                                              |
| Motore "umano"           | Maia (via lc0), opzionale                                                            | ✅ deciso                                              |
| Database                 | SQLite tramite `sqlx` (con migrazioni)                                               | ✅ deciso                                              |
| Tipi condivisi Rust → TS | generati automaticamente (`ts-rs`) in `src/bindings/`                                | ✅ deciso (sostituisce `tauri-specta`, vedi registro)  |
| Scacchiera               | `chessground` avvolto in un componente React                                         | ✅ deciso                                              |
| Stato interfaccia        | Zustand                                                                              | ✅ deciso                                              |
| Navigazione              | TanStack Router                                                                      | ✅ deciso                                              |
| CSS                      | CSS Modules + design token                                                           | ✅ deciso                                              |
| Font                     | uno per tema, inclusi nell'app (`@fontsource`): Outfit, Manrope, Plus Jakarta Sans   | ✅ deciso                                              |
| Temi                     | Studio (predefinito), Sera, Giorno, Automatico; scelti nelle Impostazioni            | ✅ deciso                                              |
| Scacchiere e pezzi       | 6 scacchiere e 6 set di pezzi, scelti nelle Impostazioni, validi in tutta l'app      | ✅ deciso                                              |
| Impostazioni             | crate `app-settings`, file `settings.json` nella cartella di configurazione dell'app | ✅ deciso                                              |
| Suoni                    | creati al momento con la Web Audio API (nessun file audio), spegnibili               | ✅ deciso                                              |
| Grafici statistiche      | da decidere più avanti                                                               | ⚪ aperto                                              |
| Linguaggio del tutor     | regole → poi AI locale (Ollama)                                                      | ⚪ aperto                                              |
| Qualità codice           | `rustfmt` + `clippy`, ESLint + Prettier                                              | ✅ deciso                                              |
| Test                     | `cargo test`, Vitest                                                                 | ✅ deciso                                              |
| Sicurezza (CSP)          | Content Security Policy restrittiva                                                  | ⚪ aperto — da attivare quando l'interfaccia è stabile |

Versioni minime: **Node.js 22.12+** (richiesto da Vitest 5), **Rust stabile** con edizione **2024**.

---

## 4. Architettura

### 4.1 Principio

```
┌──────────────────────────────────────────────┐
│  INTERFACCIA (React)                         │
│  mostra, anima, raccoglie i clic             │
└───────────────▲──────────────┬───────────────┘
   eventi live  │              │ comandi
   (analisi in  │              │ (fai mossa, avvia
    streaming)  │              ▼  partita, chiedi tutor)
┌───────────────┴──────────────────────────────┐
│  PONTE (src-tauri)                           │
│  solo comandi ed eventi, nessuna logica      │
└───────────────▲──────────────┬───────────────┘
                │              ▼
┌──────────────────────────────────────────────┐
│  NUCLEO (crates Rust indipendenti)           │
│  regole · motori · analisi · tutor ·         │
│  statistiche · database                      │
└──────────────────────────────────────────────┘
```

Il nucleo è diviso in **crate** (librerie Rust separate). Ognuna ha un compito solo, si testa da sola e non conosce l'interfaccia.

### 4.2 Il tutor a tre livelli

1. **Analisi (fatti verificati)**: motore + rilevatori (minacce, tattiche, struttura pedonale, sicurezza del re, attività dei pezzi).
2. **Conoscenza (il perché strategico)**: base di conoscenza curata in `knowledge/` con piani tipici per strutture, aperture, tipi di posizione e finali.
3. **Linguaggio (come lo dice)**: trasforma fatti e conoscenza in spiegazioni. Il livello linguistico **non decide mai nulla**: racconta solo ciò che i primi due livelli hanno verificato.

### 4.3 Dati per le statistiche

Ogni mossa giocata viene registrata con: valutazione prima e dopo, perdita rispetto alla mossa migliore, tempo impiegato, fase di gioco, apertura, temi tattici presenti, minacce viste o mancate. Si analizza solo ciò che si registra: lo schema del database si progetta prima delle schermate.

---

## 5. Metodologia CSS

- **Design token** in `src/styles/tokens.css` (misure, tipografia, forme, durate, colori delle cornici) e in `src/styles/themes/` (colori e font di ogni tema). Sono **gli unici posti** dove compaiono valori grezzi (es. `#1e1e1e`, `16px`).
- **CSS Modules**: ogni componente ha il suo `NomeComponente.module.css` accanto al `.tsx`. Gli stili sono isolati: non possono influenzare altri componenti.
- **Nei moduli si usano solo variabili** (`var(--color-surface)`), mai valori grezzi.
- **Stili globali** solo in `src/styles/` (token, temi, reset, base), nient'altro.
- **Temi**: ogni tema ridefinisce le stesse variabili sotto `[data-theme='nome']`. Il tema attivo è scritto su `<html>` da `src/app/theme-controller/`; i componenti non sanno quale tema è attivo. Un elemento con `data-theme` mostra un tema anche da solo (usato per le anteprime).
- Vietati: stili inline, `!important`, selettori che raggiungono componenti altrui.
- Classi condizionali con `classNames()` (`src/lib/classNames.ts`), mai concatenazioni a mano.
- Animazioni: solo con le durate dei token; con "riduci movimento" attivo nel sistema le durate diventano 0.
- **Unica eccezione:** gli stili globali della scacchiera, importati solo in `src/components/chessboard/Chessboard.tsx`: due file di chessground (struttura e colori di coordinate ed evidenziazioni, che non si modificano) e `src/components/chessboard/themes/` (le nostre scacchiere e i nostri pezzi, attivati dagli attributi `data-board` e `data-pieces`).
- Identità visiva: tre atmosfere (Studio, Sera, Giorno) con la stessa struttura; la scacchiera, sempre con la sua cornice, è la protagonista.

---

## 5-bis. Regole multipiattaforma (Windows · macOS · Linux)

L'interfaccia gira dentro il motore web del sistema, che è diverso su ogni piattaforma:

| Sistema | Motore web               | Note                                    |
| ------- | ------------------------ | --------------------------------------- |
| Windows | WebView2 (base Chromium) | già presente su Windows 10/11           |
| macOS   | WKWebView (base Safari)  | il più restrittivo: si testa sempre qui |
| Linux   | WebKitGTK                | va installato come dipendenza           |

Regole:

1. **Percorsi di sistema solo tramite le API di Tauri** (cartella dati dell'app, cache, documenti). Mai percorsi scritti a mano, mai `\` o `/` inseriti manualmente: in Rust si usa `Path`/`PathBuf`.
2. **Database e impostazioni** vivono nella cartella dati dell'app fornita da Tauri.
3. **Motori esterni** (Stockfish, lc0) come _sidecar_ Tauri: un eseguibile per piattaforma in `src-tauri/binaries/`, con il nome che indica la piattaforma (es. `stockfish-x86_64-pc-windows-msvc.exe`, `stockfish-aarch64-apple-darwin`, `stockfish-x86_64-unknown-linux-gnu`). Tauri sceglie da solo quello giusto.
4. **CSS e JavaScript** solo con funzionalità supportate da tutti e tre i motori web.
5. **Script del progetto** solo in `npm` o Rust, mai script `bash` o `PowerShell` specifici.
6. **Fine riga** uniformate con `.gitattributes`, così Windows e Mac non si "litigano" i file.
7. **Build ufficiali** generate automaticamente da GitHub Actions su tutte e tre le piattaforme a ogni modifica: se una piattaforma si rompe, lo vediamo subito.
8. Scorciatoie da tastiera: `Ctrl` su Windows/Linux, `Cmd` su macOS, gestite in un unico punto.

---

## 6. Struttura delle cartelle

```
app-scacchi/
├── PROGETTO.md                 memoria del progetto (questo file)
├── README.md                   presentazione generale
├── Cargo.toml                  elenco delle crate Rust (workspace)
├── package.json                dipendenze dell'interfaccia
├── .cargo/config.toml          dove ts-rs scrive i tipi generati
├── .github/workflows/ci.yml    controlli automatici su Windows/macOS/Linux
│
├── crates/                     NUCLEO RUST — una crate per responsabilità
│   ├── chess-core/             posizione, mosse, regole, partita, PGN/FEN (su shakmaty)
│   ├── app-settings/           impostazioni dell'utente: valori, salvataggio, lettura
│   ├── engine/                 dialogo con i motori (protocollo UCI)
│   │                           interfaccia comune + Stockfish + Maia
│   ├── analysis/               rilevatori: minacce, tattiche, struttura,
│   │                           sicurezza del re, fase di gioco
│   ├── tutor/                  unisce analisi e conoscenza, produce spiegazioni
│   ├── training/               generazione e verifica degli esercizi di calcolo
│   ├── stats/                  calcolo delle statistiche
│   └── storage/                database SQLite, schema, migrazioni
│
├── src-tauri/                  PONTE — solo comandi ed eventi
│   ├── src/commands/           un file per gruppo di comandi
│   ├── src/events/             eventi inviati all'interfaccia (analisi live)
│   ├── src/state/              stato condiviso dell'app (partita in corso; poi motori attivi)
│   └── binaries/               eseguibili dei motori (Stockfish, lc0)
│
├── src/                        INTERFACCIA REACT
│   ├── app/                    avvio, router, cornice comune, tema
│   │   ├── settings-gate/      legge le impostazioni prima di mostrare l'app
│   │   ├── root-layout/        cornice comune a tutte le pagine
│   │   ├── theme-controller/   applica il tema scelto
│   │   ├── app-layout/         griglia: barra laterale + area di lavoro
│   │   └── sidebar/            barra laterale e elenco delle sezioni
│   ├── styles/                 index.css, tokens.css, themes/, reset.css, base.css
│   ├── assets/                 immagini: scacchiere (boards/) e pezzi (pieces/)
│   ├── state/                  stato condiviso (Zustand): impostazioni, partita in corso
│   ├── bindings/               tipi generati da Rust (NON modificare a mano)
│   ├── services/               chiamate ai comandi e ascolto eventi
│   ├── components/             componenti generici riutilizzabili
│   │   ├── chessboard/         scacchiera (involucro di chessground) + themes/
│   │   ├── option-group/       gruppo di scelte a riquadri
│   │   ├── piece-icon/         un pezzo fuori dalla scacchiera, nello stile scelto
│   │   ├── panel-card/         scheda con titolo dei pannelli
│   │   ├── button/             pulsante (principale, secondario, icona)
│   │   ├── icon/               icone a tratto
│   │   └── eval-bar/           barra di valutazione
│   ├── features/               aree dell'app, ognuna autonoma
│   │   ├── play/               partita (a due giocatori; dalla Fase 3 contro il computer)
│   │   │                       move-list/, game-controls/, promotion-picker/,
│   │   │                       game-result-banner/, import-export/, position-panel/
│   │   ├── training/           esercizi di calcolo e visualizzazione
│   │   ├── tutor/              pannello del tutor, apribile ovunque
│   │   ├── review/             analisi delle partite giocate
│   │   ├── progress/           statistiche e livello
│   │   └── settings/           impostazioni: tema, scacchiera, pezzi, notazione
│   ├── sounds/                 suoni del gioco (mossa, presa, fine partita), creati con Web Audio
│   ├── hooks/                  hook React condivisi (tastiera, suoni, tema di sistema)
│   ├── lib/                    piccole utility (notazione, errori, appunti)
│   └── test/                   impostazioni comuni dei test
│
├── knowledge/                  base di conoscenza strategica (testi curati)
│   ├── structures/             strutture pedonali e piani tipici
│   ├── openings/               idee delle aperture, col bianco e col nero
│   ├── middlegame/             concetti di mediogioco
│   └── endgames/               finali
│
└── data/                       dati importati (puzzle Lichess, nomi aperture)
```

### Convenzione per ogni componente React

```
features/play/move-list/
├── README.md
├── MoveList.tsx             il componente
├── MoveList.module.css      i suoi stili
└── MoveList.test.tsx        i suoi test
```

---

## 7. Convenzioni

- **Nomi nel codice**: in inglese (standard del settore).
- **Commenti e README**: in italiano.
- Il punto di ingresso dell'interfaccia è `src/main.tsx` (convenzione di Vite); tutto il resto vive nelle sottocartelle di `src/`.
- Nel workspace Rust, la libreria dell'app si chiama `scacchi_app_lib` (nome diverso dall'eseguibile per evitare conflitti su Windows).
- File React: `PascalCase.tsx`. Altri file TypeScript: `camelCase.ts` (es. `positionService.ts`). Cartelle: `kebab-case`. Rust: `snake_case`.
- I componenti specifici di una sezione stanno in una sottocartella della sezione (es. `features/play/position-panel/`); quelli usati da più sezioni in `src/components/`.
- I nomi dei comandi Rust compaiono **solo** in `src/services/`.
- Ogni crate Rust espone solo ciò che serve; il resto resta privato.

---

### 7.1 Controlli di qualità

**Regola: prima di considerare finita qualsiasi modifica, `npm run check` deve essere tutto verde.**
Lo stesso comando viene eseguito da GitHub su Windows, macOS e Linux a ogni invio.

| Comando                     | Cosa controlla                                                                                   |
| --------------------------- | ------------------------------------------------------------------------------------------------ |
| `npm run check`             | Tutti i controlli qui sotto, in sequenza (prima Rust, poi interfaccia); si ferma al primo errore |
| `npm run format:check`      | Che il codice TS/JSON/CSS/Markdown sia formattato secondo Prettier                               |
| `npm run lint`              | Errori e cattive pratiche nel codice TS/React (ESLint, livello severo)                           |
| `npm run typecheck`         | Che i tipi TypeScript siano tutti coerenti                                                       |
| `npm run test`              | I test dell'interfaccia (Vitest)                                                                 |
| `npm run rust:format:check` | Che il codice Rust sia formattato (rustfmt)                                                      |
| `npm run rust:lint`         | Errori e cattive pratiche nel codice Rust (Clippy, livello pedantic; ogni avviso è un errore)    |
| `npm run test:rust`         | I test di tutte le crate Rust; **rigenerano anche i tipi in `src/bindings/`**                    |

Per sistemare automaticamente la formattazione: `npm run format` (interfaccia) e `npm run rust:format` (Rust).

Regole Rust valide per tutte le crate (definite in `Cargo.toml` della radice):

- `unsafe` vietato;
- ogni elemento pubblico deve avere un commento di documentazione;
- ogni funzione che può fermarsi con un errore lo dichiara nella sezione `# Panics` del suo commento;
- ogni funzione che restituisce un `Result` spiega i casi di errore nella sezione `# Errors`.

**Tipi condivisi:** dopo aver modificato un tipo Rust esposto all'interfaccia, eseguire `npm run check`
(che rigenera `src/bindings/`) e salvare i file rigenerati. GitHub verifica che siano aggiornati.

I test stanno **accanto al file che verificano**, con suffisso `.test.ts` / `.test.tsx` (interfaccia)
o in un modulo `tests` nello stesso file (Rust).

---

## 8. Roadmap

**Un passo di lavoro = una fase intera.** Ogni fase si chiude con tutti i test verdi.

| Fase                            | Contenuto                                                                                                                                                      | Traguardo                                             |
| ------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------- |
| 1. Fondamenta                   | Ambiente, scheletro, controlli di qualità, CI, design token e guscio grafico, crate `chess-core`, scacchiera                                                   | L'app si apre e mostra la posizione calcolata da Rust |
| 1-bis. Temi e personalizzazione | Impostazioni con 4 temi dell'app, 6 scacchiere, 6 set di pezzi; scelte salvate e valide ovunque; pannello senza FEN                                            | L'app ha l'aspetto scelto dall'utente                 |
| 2. Regole e partita             | Mosse sulla scacchiera validate da Rust; arrocco, promozione, en passant; lista mosse navigabile; fine partita; import/export PGN e FEN                        | Partita completa a due giocatori                      |
| 3. Motore                       | Stockfish sulle tre piattaforme; dialogo UCI; analisi live con barra e frecce; partita contro il computer a livelli; orologio                                  | **Si gioca contro Stockfish a qualsiasi livello**     |
| 4. Database                     | Schema con migrazioni; salvataggio automatico di ogni mossa con i dati per le statistiche; archivio partite                                                    | Nessuna partita va persa                              |
| 5. Analisi                      | Rilevatori: pezzi indifesi e minacce; tattiche (forchette, inchiodature, infilate…); struttura pedonale; sicurezza del re e attività dei pezzi                 | L'app "capisce" la posizione                          |
| 6. Allenamento calcolo          | Import puzzle Lichess; puzzle per profondità; modalità visualizzazione; "cosa minaccia?"; ripasso degli errori                                                 | **Allenamento al calcolo completo**                   |
| 7. Tutor                        | Pannello richiamabile ovunque; spiegazioni delle posizioni; base di conoscenza (principi, aperture col bianco e col nero, mediogioco, finali); scenari "e se…" | **Il tutor spiega ogni posizione**                    |
| 8. Analisi partite              | Revisione automatica; momenti critici; mosse mancate spiegate                                                                                                  | Si impara dalle proprie partite                       |
| 9. Statistiche                  | Metriche, grafici, livello stimato, profondità di calcolo nel tempo                                                                                            | Progressi misurabili                                  |
| 10. Estensioni                  | Maia (avversario "umano"); AI locale per spiegazioni più naturali; conoscenza avanzata; installatori e aggiornamenti                                           | App completa                                          |

---

## 9. Stato di avanzamento

| Fase                            | Stato         |
| ------------------------------- | ------------- |
| 1. Fondamenta                   | ✅ completata |
| 1-bis. Temi e personalizzazione | ✅ completata |
| 2. Regole e partita             | ✅ completata |
| 3. Motore                       | ⚪ da fare    |
| 4. Database                     | ⚪ da fare    |
| 5. Analisi                      | ⚪ da fare    |
| 6. Allenamento calcolo          | ⚪ da fare    |
| 7. Tutor                        | ⚪ da fare    |
| 8. Analisi partite              | ⚪ da fare    |
| 9. Statistiche                  | ⚪ da fare    |
| 10. Estensioni                  | ⚪ da fare    |

Legenda: ⚪ da fare · 🔵 in test · ✅ completata (tutti i test verdi)

---

## 10. Registro delle decisioni

| Data       | Decisione                                                                                | Motivo                                                                                                                                                                      |
| ---------- | ---------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 2026-10-18 | App desktop, tutto offline                                                               | Requisito di partenza                                                                                                                                                       |
| 2026-10-18 | Tauri 2 + Rust                                                                           | Più robusto, leggero e moderno di Electron                                                                                                                                  |
| 2026-10-18 | Regole degli scacchi in Rust                                                             | Una sola fonte di verità                                                                                                                                                    |
| 2026-10-18 | React per l'interfaccia                                                                  | Ecosistema più ampio                                                                                                                                                        |
| 2026-10-18 | Windows, macOS e Linux                                                                   | L'app deve funzionare su qualsiasi sistema                                                                                                                                  |
| 2026-10-18 | Build automatiche con GitHub Actions                                                     | Verificare le tre piattaforme a ogni modifica                                                                                                                               |
| 2026-10-18 | Approvate tutte le scelte dello stack                                                    | Conferma di Alessandro                                                                                                                                                      |
| 2026-10-18 | Nomi nel codice in inglese, commenti e README in italiano                                | Conferma di Alessandro                                                                                                                                                      |
| 2026-10-18 | Scheletro scritto a mano, senza template dimostrativo                                    | Nessun codice morto da cancellare                                                                                                                                           |
| 2026-10-18 | Workspace Cargo con le crate aggiunte una alla volta                                     | Si aggiunge una crate solo quando serve davvero                                                                                                                             |
| 2026-10-18 | ESLint `strictTypeChecked` + Prettier, separati                                          | ESLint controlla la correttezza, Prettier l'aspetto: nessuna sovrapposizione                                                                                                |
| 2026-10-18 | Clippy `pedantic` con avvisi trattati come errori                                        | Massimo rigore fin dall'inizio, quando costa poco                                                                                                                           |
| 2026-10-18 | `npm run check` come unico comando di verifica                                           | Stesso controllo sul PC e su GitHub                                                                                                                                         |
| 2026-10-18 | Un passo di lavoro = una fase intera della roadmap                                       | Richiesta di Alessandro: procedere più velocemente                                                                                                                          |
| 2026-10-18 | `ts-rs` al posto di `tauri-specta` per i tipi condivisi                                  | `tauri-specta` è ancora in versione pre-release (rc.25) con documentazione non compilabile; `ts-rs` è stabile (v12). Stesso risultato: tipi scritti una volta sola, in Rust |
| 2026-10-18 | chessground dal pacchetto `@lichess-org/chessground`                                     | È il nuovo nome ufficiale del pacchetto                                                                                                                                     |
| 2026-10-18 | Tema chiaro e scuro automatici                                                           | Segue il sistema operativo; costo minimo grazie ai token                                                                                                                    |
| 2026-10-18 | Identità "panno da torneo + ottone", font IBM Plex Sans incluso                          | Coerente con l'icona; il font incluso funziona offline e identico ovunque                                                                                                   |
| 2026-10-18 | Licenza: chessground e Stockfish sono GPL-3.0                                            | Uso personale libero; un'eventuale distribuzione richiede una licenza compatibile                                                                                           |
| 2026-10-18 | Vitest 5 (da 3) e Node.js minimo 22.12                                                   | Corregge 3 vulnerabilità (1 moderata, 2 critiche) di Vitest 3 e della sua dipendenza `tinypool`; riguardano solo i test, non l'app                                          |
| 2026-10-18 | Fase 1-bis: temi, scacchiere e pezzi selezionabili                                       | Richiesta di Alessandro; predefinito Studio + Rovere + Icona                                                                                                                |
| 2026-10-18 | Scacchiere e pezzi disegnati per l'app (tranne il Classico "cburnett")                   | Nessun vincolo di licenza esterno; stile coerente                                                                                                                           |
| 2026-10-18 | Le scelte di stile valgono in tutta l'app                                                | La scacchiera legge da sola le impostazioni: ogni sezione la usa senza occuparsene                                                                                          |
| 2026-10-18 | Impostazioni salvate da Rust (crate `app-settings`)                                      | Una sola fonte di verità; file nella cartella di configurazione fornita da Tauri                                                                                            |
| 2026-10-18 | Navigazione con TanStack Router (cronologia a hash)                                      | Ora le sezioni utilizzabili sono due; l'hash funziona uguale su ogni sistema                                                                                                |
| 2026-10-18 | FEN tolta dal pannello di Gioca                                                          | Informazione tecnica; tornerà nella Fase 2 come copia/incolla della posizione                                                                                               |
| 2026-10-18 | Fase 1-bis approvata                                                                     | Conferma di Alessandro: aspetto e scelte di stile approvati                                                                                                                 |
| 2026-10-18 | Fase 2: la partita vive in Rust (`Game` in `chess-core`, custodita da `src-tauri/state`) | Una sola fonte di verità per le regole; l'interfaccia riceve sempre la partita completa (`GameView`)                                                                        |
| 2026-10-18 | Notazione predefinita a figurine, con scelta Italiana / Internazionale                   | Richiesta di Alessandro; Rust tiene sempre la notazione internazionale, l'interfaccia la traduce                                                                            |
| 2026-10-18 | Una mossa giocata da una posizione passata sostituisce il seguito                        | Semplice e prevedibile; le varianti arriveranno nella Fase 8                                                                                                                |
| 2026-10-18 | Copia/incolla di FEN e PGN negli appunti; salvataggio su file nella Fase 4               | Il salvataggio su file richiede l'archivio partite                                                                                                                          |
| 2026-10-18 | Componenti comuni `Button`, `Icon`, `PanelCard`, `PieceIcon`                             | Stesso aspetto ovunque; sostituiscono la cartella `ui/` prevista                                                                                                            |
| 2026-10-18 | Progetto sotto git (ramo `main`), un commit per ogni passo                               | Ogni fase è reversibile e si vede esattamente cosa cambia                                                                                                                   |
| 2026-10-18 | Suoni del gioco creati con Web Audio: mossa, presa, fine partita; impostazione "Suoni"   | Richiesta di Alessandro. Nessun file né licenza (i suoni classici di Lichess non sono liberi); suonano solo avanzando di una mossa                                          |
| 2026-10-18 | `MoveRecord` dice se la mossa è una presa (`capture`)                                    | L'interfaccia sceglie il suono senza conoscere le regole                                                                                                                    |
| 2026-10-18 | "Annulla" toglie l'ultima mossa della partita; "Nuova partita" senza conferma            | Comportamenti confermati da Alessandro                                                                                                                                      |
| 2026-10-18 | Fase 2 approvata; `PASSAGGIO-CONSEGNE.md` cancellato                                     | Conferma di Alessandro; appunti per le fasi future e trappole note riportati in §11 e §12                                                                                   |

---

## 11. Appunti tecnici per le fasi successive

Base per il piano di ogni fase: va sempre presentato ad Alessandro e approvato prima di scrivere codice.

- **Fase 3 — Motore.**
  - Nuova crate `crates/engine`: un "trait" `Engine` con un'unica implementazione UCI (`uci`, `isready`, `ucinewgame`, `position fen … moves …`, `go depth|movetime|wtime/btime`, lettura di `info … score cp|mate … pv …` e `bestmove`), asincrona con tokio e cancellabile (`stop`).
  - Stockfish come sidecar: binari in `src-tauri/binaries/` con il nome della piattaforma (`stockfish-x86_64-pc-windows-msvc.exe`, `stockfish-aarch64-apple-darwin`, `stockfish-x86_64-apple-darwin`, `stockfish-x86_64-unknown-linux-gnu`), `bundle.externalBin` in `tauri.conf.json`, `tauri-plugin-shell` con il permesso in `capabilities/default.json`. Licenza GPL-3.0 da annotare.
  - **Da decidere:** come procurarsi i binari (script npm multipiattaforma che li scarica, oppure a mano) e la scala dei livelli (`Skill Level` 0–20, `UCI_LimitStrength` + `UCI_Elo` 1320–3190, oppure livelli con nomi).
  - Analisi live come eventi Tauri (`emit`, nuova cartella `src-tauri/src/events/`), mai come attesa bloccante.
  - Interfaccia: `components/eval-bar/`, frecce della mossa migliore (`drawable.autoShapes` di chessground), variante principale nella notazione scelta, scelta del colore, orologio (conteggio in Rust). La scacchiera resta bloccata durante il turno del computer; i suoni delle sue mosse partono già da soli (`useMoveSound`).
- **Fase 4 — Database.** Crate `crates/storage` (SQLite con `sqlx`, migrazioni, file nella `app_data_dir`). Schema prima delle schermate (§4.3). Salvataggio a ogni mossa e ripresa all'avvio; archivio partite con ricerca; file `.pgn` con `tauri-plugin-dialog`.
- **Fase 5 — Analisi.** Crate `crates/analysis`, un rilevatore per file (pezzi indifesi, minacce, tattiche, struttura pedonale, sicurezza del re, attività, fase di gioco), ognuno con test su FEN note.
- **Fase 6 — Allenamento.** Crate `crates/training`; puzzle di Lichess (CSV, licenza CC0) in `data/`; puzzle per profondità, visualizzazione, "cosa minaccia?", ripasso degli errori con ripetizione dilazionata.
- **Fase 7 — Tutor.** Crate `crates/tutor` e `knowledge/` con testi curati; pannello richiamabile ovunque con una scorciatoia unica.
- **Fase 8 — Analisi partite.** Momenti critici, mosse mancate spiegate, grafico della valutazione; varianti nella lista mosse (albero al posto della sostituzione del seguito: richiede di estendere `Game`).
- **Fase 9 — Statistiche.** Crate `crates/stats`; libreria dei grafici da scegliere (offline, rispettosa dei temi).
- **Fase 10 — Estensioni.** Maia via lc0 (sidecar), AI locale (es. Ollama) che racconta senza decidere, installatori firmati e aggiornamenti (`tauri-plugin-updater`), CSP restrittiva.
- **Prima di distribuire:** chessground e Stockfish sono GPL-3.0; per uso personale nessun problema.

---

## 12. Trappole note e lezioni imparate

- **shakmaty:** serve `fen.parse::<Fen>()`, altrimenti errore E0282.
- **Clippy pedantic:** nei test non `assert!(v.is_empty())` ma `assert_eq!(v, [])`.
- **ESLint strict:** niente cast `as HTMLInputElement` (si usa `getByRole<HTMLInputElement>(…)`); niente `invoke<void>`; `FormEvent` è deprecato (`SyntheticEvent`); `Array.prototype.at` non disponibile (libreria ES2021); niente parametri inutilizzati, nemmeno con `_` (per i finti si dichiara il tipo con `vi.fn<…>()`).
- **CSS Modules:** con `noUncheckedIndexedAccess` le classi sono `string | undefined`: sempre tramite `classNames()`.
- **Test:** la pulizia tra un test e l'altro e le simulazioni che jsdom non ha (`matchMedia`, `scrollIntoView`, `scrollTo`) stanno in `src/test/setup.ts`; per i suoni c'è `src/test/fakeAudioContext.ts`.
- **Vite inserisce le immagini piccole nel codice** (indirizzi `data:`): i test non devono cercare il nome del file nell'indirizzo, ma confrontare con `pieceImageUrl(…)`.
- **Prettier** riformatta le tabelle Markdown: dopo aver scritto un README eseguire `npm run format`.
- **Tipi in `src/bindings/`:** li genera `ts-rs` con `npm run test:rust`; non si scrivono a mano.
- **npm audit:** mai `npm audit fix --force`.
- **`tauri-specta`** è stato scartato perché instabile; non va riproposto. Il pacchetto chessground si chiama `@lichess-org/chessground`.
- **Gusti di Alessandro:** cornici attorno alla scacchiera, pezzi puliti e minimal, tanta scelta di stili, ogni scelta valida in tutta l'app; a ogni fase chiede ritocchi estetici.
