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

| Ambito                   | Scelta                                                                                                                                                | Stato                                                  |
| ------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------ |
| Contenitore desktop      | Tauri 2                                                                                                                                               | ✅ deciso                                              |
| Logica, motore, dati     | Rust                                                                                                                                                  | ✅ deciso                                              |
| Interfaccia              | React 19 + TypeScript                                                                                                                                 | ✅ deciso                                              |
| Build interfaccia        | Vite                                                                                                                                                  | ✅ deciso                                              |
| Regole degli scacchi     | `shakmaty` (Rust)                                                                                                                                     | ✅ deciso                                              |
| Motore forte             | Stockfish 19 nativo (sidecar Tauri), scaricato e verificato da `npm run engines`                                                                      | ✅ deciso                                              |
| Dialogo con i motori     | crate `engine` (UCI), asincrona con `tokio`; un motore che gioca e uno che analizza                                                                   | ✅ deciso                                              |
| Livelli del computer     | 10 livelli con nome ed Elo indicativo (`UCI_Elo` sopra 1320, `Skill Level` sotto)                                                                     | ✅ deciso                                              |
| Motore "umano"           | Maia (via lc0), opzionale                                                                                                                             | ✅ deciso                                              |
| Database                 | SQLite tramite `sqlx` 0.9 (con migrazioni), file nella cartella dati dell'app                                                                         | ✅ deciso                                              |
| Aperture                 | elenco di Lichess (CC0) in `data/openings/`, crate `openings`                                                                                         | ✅ deciso                                              |
| File PGN                 | `tauri-plugin-dialog` (finestre di sistema); lettura e scrittura da Rust                                                                              | ✅ deciso                                              |
| Rilevatori               | crate `analysis` (regole + scambio statico, senza motore); scheda "Osservazioni"                                                                      | ✅ deciso                                              |
| Puzzle                   | ~100.000 puzzle di Lichess (CC0) in `data/puzzles/`, inclusi nell'app e tenuti in memoria; crate `training`                                           | ✅ deciso                                              |
| Tutor                    | crate `tutor`: fatti dei rilevatori + schede TOML in `knowledge/` (incluse con `build.rs`); un terzo Stockfish per valutazioni brevi                  | ✅ deciso                                              |
| Analisi partite          | crate `review`: probabilità di vittoria persa (come Lichess), precisione, momenti critici; un quarto Stockfish, 1 s per posizione; grafico SVG nostro | ✅ deciso                                              |
| Tipi condivisi Rust → TS | generati automaticamente (`ts-rs`) in `src/bindings/`                                                                                                 | ✅ deciso (sostituisce `tauri-specta`, vedi registro)  |
| Scacchiera               | `chessground` avvolto in un componente React                                                                                                          | ✅ deciso                                              |
| Stato interfaccia        | Zustand                                                                                                                                               | ✅ deciso                                              |
| Navigazione              | TanStack Router                                                                                                                                       | ✅ deciso                                              |
| CSS                      | CSS Modules + design token                                                                                                                            | ✅ deciso                                              |
| Font                     | uno per tema, inclusi nell'app (`@fontsource`): Outfit, Manrope, Plus Jakarta Sans                                                                    | ✅ deciso                                              |
| Temi                     | Studio (predefinito), Sera, Giorno, Automatico; scelti nelle Impostazioni                                                                             | ✅ deciso                                              |
| Scacchiere e pezzi       | 6 scacchiere e 6 set di pezzi, scelti nelle Impostazioni, validi in tutta l'app                                                                       | ✅ deciso                                              |
| Impostazioni             | crate `app-settings`, file `settings.json` nella cartella di configurazione dell'app                                                                  | ✅ deciso                                              |
| Suoni                    | creati al momento con la Web Audio API (nessun file audio), spegnibili                                                                                | ✅ deciso                                              |
| Grafici statistiche      | da decidere più avanti                                                                                                                                | ⚪ aperto                                              |
| Linguaggio del tutor     | regole (Fase 7) → poi AI locale (Ollama, Fase 10)                                                                                                     | 🔵 regole fatte                                        |
| Qualità codice           | `rustfmt` + `clippy`, ESLint + Prettier                                                                                                               | ✅ deciso                                              |
| Test                     | `cargo test`, Vitest                                                                                                                                  | ✅ deciso                                              |
| Sicurezza (CSP)          | Content Security Policy restrittiva                                                                                                                   | ⚪ aperto — da attivare quando l'interfaccia è stabile |

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
├── scripts/                    comandi di servizio in Node (es. download di Stockfish)
│
├── crates/                     NUCLEO RUST — una crate per responsabilità
│   ├── chess-core/             posizione, mosse, regole, partita, PGN/FEN/SAN (su shakmaty)
│   ├── app-settings/           impostazioni dell'utente (e ultima partita scelta): valori, salvataggio
│   ├── engine/                 dialogo con i motori (protocollo UCI): analisi, mosse, livelli
│   ├── game-session/           partita in corso: avversario, livello, orologio, chi può muovere,
│   │                           tempo di ogni mossa, fotografia da salvare e ripresa
│   ├── openings/               riconoscimento delle aperture (elenco di Lichess)
│   ├── analysis/               rilevatori: minacce, tattiche, struttura pedonale,
│   │                           sicurezza del re, attività, materiale, fase di gioco
│   ├── tutor/                  spiegazioni: riconosce strutture, finali, temi e principi, unisce
│   │                           fatti e schede di knowledge/; scenari "e se…"
│   ├── training/               puzzle: scelta, verifica delle mosse, rating Elo, ripasso (scatole)
│   ├── review/                 analisi delle partite: giudizio delle mosse, precisione, momenti
│   │                           critici, dati per le statistiche, puzzle dagli errori gravi
│   ├── stats/                  calcolo delle statistiche
│   └── storage/                database SQLite: partite, archivio, tentativi dei puzzle, ripasso
│
├── src-tauri/                  PONTE — solo comandi ed eventi
│   ├── src/commands/           un file per gruppo di comandi
│   ├── src/events/             eventi inviati all'interfaccia (mossa del computer, analisi live)
│   ├── src/reactions/          ciò che succede dopo ogni cambiamento (computer, orologio, analisi,
│   │                           salvataggio) e all'avvio (ripresa della partita)
│   ├── src/state/              stato condiviso dell'app (partita, motori, database, puzzle in corso)
│   └── binaries/               eseguibili dei motori (Stockfish; poi lc0), scaricati, non in git
│
├── src/                        INTERFACCIA REACT
│   ├── app/                    avvio, router, cornice comune, tema
│   │   ├── settings-gate/      legge le impostazioni prima di mostrare l'app
│   │   ├── root-layout/        cornice comune a tutte le pagine
│   │   ├── theme-controller/   applica il tema scelto
│   │   ├── app-layout/         area di lavoro a tutta finestra + menu principale a comparsa
│   │   └── sidebar/            barra laterale e elenco delle sezioni
│   ├── styles/                 index.css, tokens.css, themes/, reset.css, base.css
│   ├── assets/                 immagini: scacchiere (boards/) e pezzi (pieces/)
│   ├── state/                  stato condiviso (Zustand): impostazioni, partita, motore, allenamento
│   ├── bindings/               tipi generati da Rust (NON modificare a mano)
│   ├── services/               chiamate ai comandi e ascolto eventi
│   ├── components/             componenti generici riutilizzabili
│   │   ├── chessboard/         scacchiera (involucro di chessground) + themes/
│   │   ├── option-group/       gruppo di scelte a riquadri
│   │   ├── piece-icon/         un pezzo fuori dalla scacchiera, nello stile scelto
│   │   ├── panel-card/         scheda con titolo dei pannelli
│   │   ├── button/             pulsante (principale, secondario, icona)
│   │   ├── icon/               icone a tratto
│   │   ├── eval-bar/           barra di valutazione
│   │   ├── switch/             interruttore acceso/spento
│   │   ├── choice-group/       gruppo di scelte a pastiglie (compatto)
│   │   ├── side-drawer/        barra laterale a comparsa (il menu principale, a sinistra)
│   │   ├── promotion-picker/   scelta del pezzo nella promozione (Gioca e Allenamento)
│   │   ├── knowledge-cards/    schede della base di conoscenza (pannello del tutor e modalità Tutor)
│   │   ├── tab-switch/         selettore a schede («Partita | Tutor»)
│   │   └── eval-graph/         grafico della partita (probabilità di vittoria)
│   ├── features/               aree dell'app, ognuna autonoma
│   │   ├── play/               partita contro il computer o a due giocatori, con orologio
│   │   │                       move-list/, game-controls/,
│   │   │                       game-result-banner/, import-export/, position-panel/,
│   │   │                       player-bar/, clock-face/, new-game-dialog/, analysis-panel/,
│   │   │                       opening-label/, pgn-file/, observations-panel/,
│   │   │                       coach/ (la sezione «Tutor»: commenti dopo ogni mossa)
│   │   ├── archive/            archivio delle partite: filtri, rivedi, continua, elimina
│   │   ├── training/           puzzle: calcolo, visualizzazione, "cosa minaccia?", ripasso
│   │   │                       training-setup/, puzzle-status/, puzzle-reveal/
│   │   ├── tutor/              pannello del tutor (voce «Tutor» della colonna di destra, tasto T):
│   │   │                       "e se…"; tutor-sections/, what-if-results/
│   │   ├── review/             analisi partite: grafico, momenti critici, riepilogo, mosse con ?!/?/??
│   │   │                       review-summary/, critical-moments/, review-moves/
│   │   ├── progress/           statistiche e livello
│   │   └── settings/           impostazioni: tema, scacchiera, pezzi, notazione, suoni
│   ├── sounds/                 suoni del gioco (mossa, presa, fine partita), creati con Web Audio
│   ├── hooks/                  hook React condivisi (tastiera, suoni, tema di sistema, orologio)
│   ├── lib/                    piccole utility (notazione, varianti, fatti, errori, appunti, tempi)
│   └── test/                   impostazioni comuni dei test
│
├── knowledge/                  base di conoscenza strategica: schede TOML in italiano
│   ├── principles/             principi generali (sviluppo, arrocco, centro, …)
│   ├── structures/             strutture pedonali e piani tipici
│   ├── openings/               idee delle aperture, col bianco e col nero
│   ├── middlegame/             concetti di mediogioco
│   └── endgames/               finali
│
└── data/                       dati inclusi nell'app: openings/ e puzzles/ (di Lichess, CC0)
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
| 8-bis. Varianti                 | Una mossa da una posizione passata crea una variante; lista mosse ad albero; PGN con varianti                                                                  | Si esplorano le alternative senza perdere la partita  |
| 9. Statistiche                  | Metriche, grafici, livello stimato, profondità di calcolo nel tempo                                                                                            | Progressi misurabili                                  |
| 10. Estensioni                  | Maia (avversario "umano"); AI locale per spiegazioni più naturali; conoscenza avanzata; installatori e aggiornamenti                                           | App completa                                          |
| 11. Appunti personali           | Uno spazio dove scrivere e ritrovare idee, pensieri e note di studio, collegabili a partite e posizioni                                                        | Le proprie idee restano nell'app                      |

---

## 9. Stato di avanzamento

| Fase                            | Stato         |
| ------------------------------- | ------------- |
| 1. Fondamenta                   | ✅ completata |
| 1-bis. Temi e personalizzazione | ✅ completata |
| 2. Regole e partita             | ✅ completata |
| 3. Motore                       | ✅ completata |
| 4. Database                     | ✅ completata |
| 5. Analisi                      | ✅ completata |
| 6. Allenamento calcolo          | ✅ completata |
| 7. Tutor                        | ✅ completata |
| 8. Analisi partite              | ✅ completata |
| 8-bis. Varianti                 | ⚪ da fare    |
| 9. Statistiche                  | ⚪ da fare    |
| 10. Estensioni                  | ⚪ da fare    |
| 11. Appunti personali           | ⚪ da fare    |

Legenda: ⚪ da fare · 🔵 in test · ✅ completata (tutti i test verdi)

---

## 10. Registro delle decisioni

| Data       | Decisione                                                                                                                                                                                        | Motivo                                                                                                                                                                                                                                        |
| ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 2026-10-18 | App desktop, tutto offline                                                                                                                                                                       | Requisito di partenza                                                                                                                                                                                                                         |
| 2026-10-18 | Tauri 2 + Rust                                                                                                                                                                                   | Più robusto, leggero e moderno di Electron                                                                                                                                                                                                    |
| 2026-10-18 | Regole degli scacchi in Rust                                                                                                                                                                     | Una sola fonte di verità                                                                                                                                                                                                                      |
| 2026-10-18 | React per l'interfaccia                                                                                                                                                                          | Ecosistema più ampio                                                                                                                                                                                                                          |
| 2026-10-18 | Windows, macOS e Linux                                                                                                                                                                           | L'app deve funzionare su qualsiasi sistema                                                                                                                                                                                                    |
| 2026-10-18 | Build automatiche con GitHub Actions                                                                                                                                                             | Verificare le tre piattaforme a ogni modifica                                                                                                                                                                                                 |
| 2026-10-18 | Approvate tutte le scelte dello stack                                                                                                                                                            | Conferma di Alessandro                                                                                                                                                                                                                        |
| 2026-10-18 | Nomi nel codice in inglese, commenti e README in italiano                                                                                                                                        | Conferma di Alessandro                                                                                                                                                                                                                        |
| 2026-10-18 | Scheletro scritto a mano, senza template dimostrativo                                                                                                                                            | Nessun codice morto da cancellare                                                                                                                                                                                                             |
| 2026-10-18 | Workspace Cargo con le crate aggiunte una alla volta                                                                                                                                             | Si aggiunge una crate solo quando serve davvero                                                                                                                                                                                               |
| 2026-10-18 | ESLint `strictTypeChecked` + Prettier, separati                                                                                                                                                  | ESLint controlla la correttezza, Prettier l'aspetto: nessuna sovrapposizione                                                                                                                                                                  |
| 2026-10-18 | Clippy `pedantic` con avvisi trattati come errori                                                                                                                                                | Massimo rigore fin dall'inizio, quando costa poco                                                                                                                                                                                             |
| 2026-10-18 | `npm run check` come unico comando di verifica                                                                                                                                                   | Stesso controllo sul PC e su GitHub                                                                                                                                                                                                           |
| 2026-10-18 | Un passo di lavoro = una fase intera della roadmap                                                                                                                                               | Richiesta di Alessandro: procedere più velocemente                                                                                                                                                                                            |
| 2026-10-18 | `ts-rs` al posto di `tauri-specta` per i tipi condivisi                                                                                                                                          | `tauri-specta` è ancora in versione pre-release (rc.25) con documentazione non compilabile; `ts-rs` è stabile (v12). Stesso risultato: tipi scritti una volta sola, in Rust                                                                   |
| 2026-10-18 | chessground dal pacchetto `@lichess-org/chessground`                                                                                                                                             | È il nuovo nome ufficiale del pacchetto                                                                                                                                                                                                       |
| 2026-10-18 | Tema chiaro e scuro automatici                                                                                                                                                                   | Segue il sistema operativo; costo minimo grazie ai token                                                                                                                                                                                      |
| 2026-10-18 | Identità "panno da torneo + ottone", font IBM Plex Sans incluso                                                                                                                                  | Coerente con l'icona; il font incluso funziona offline e identico ovunque                                                                                                                                                                     |
| 2026-10-18 | Licenza: chessground e Stockfish sono GPL-3.0                                                                                                                                                    | Uso personale libero; un'eventuale distribuzione richiede una licenza compatibile                                                                                                                                                             |
| 2026-10-18 | Vitest 5 (da 3) e Node.js minimo 22.12                                                                                                                                                           | Corregge 3 vulnerabilità (1 moderata, 2 critiche) di Vitest 3 e della sua dipendenza `tinypool`; riguardano solo i test, non l'app                                                                                                            |
| 2026-10-18 | Fase 1-bis: temi, scacchiere e pezzi selezionabili                                                                                                                                               | Richiesta di Alessandro; predefinito Studio + Rovere + Icona                                                                                                                                                                                  |
| 2026-10-18 | Scacchiere e pezzi disegnati per l'app (tranne il Classico "cburnett")                                                                                                                           | Nessun vincolo di licenza esterno; stile coerente                                                                                                                                                                                             |
| 2026-10-18 | Le scelte di stile valgono in tutta l'app                                                                                                                                                        | La scacchiera legge da sola le impostazioni: ogni sezione la usa senza occuparsene                                                                                                                                                            |
| 2026-10-18 | Impostazioni salvate da Rust (crate `app-settings`)                                                                                                                                              | Una sola fonte di verità; file nella cartella di configurazione fornita da Tauri                                                                                                                                                              |
| 2026-10-18 | Navigazione con TanStack Router (cronologia a hash)                                                                                                                                              | Ora le sezioni utilizzabili sono due; l'hash funziona uguale su ogni sistema                                                                                                                                                                  |
| 2026-10-18 | FEN tolta dal pannello di Gioca                                                                                                                                                                  | Informazione tecnica; tornerà nella Fase 2 come copia/incolla della posizione                                                                                                                                                                 |
| 2026-10-18 | Fase 1-bis approvata                                                                                                                                                                             | Conferma di Alessandro: aspetto e scelte di stile approvati                                                                                                                                                                                   |
| 2026-10-18 | Fase 2: la partita vive in Rust (`Game` in `chess-core`, custodita da `src-tauri/state`)                                                                                                         | Una sola fonte di verità per le regole; l'interfaccia riceve sempre la partita completa (`GameView`)                                                                                                                                          |
| 2026-10-18 | Notazione predefinita a figurine, con scelta Italiana / Internazionale                                                                                                                           | Richiesta di Alessandro; Rust tiene sempre la notazione internazionale, l'interfaccia la traduce                                                                                                                                              |
| 2026-10-18 | Una mossa giocata da una posizione passata sostituisce il seguito                                                                                                                                | Semplice e prevedibile; le varianti arriveranno nella Fase 8                                                                                                                                                                                  |
| 2026-10-18 | Copia/incolla di FEN e PGN negli appunti; salvataggio su file nella Fase 4                                                                                                                       | Il salvataggio su file richiede l'archivio partite                                                                                                                                                                                            |
| 2026-10-18 | Componenti comuni `Button`, `Icon`, `PanelCard`, `PieceIcon`                                                                                                                                     | Stesso aspetto ovunque; sostituiscono la cartella `ui/` prevista                                                                                                                                                                              |
| 2026-10-18 | Progetto sotto git (ramo `main`), un commit per ogni passo                                                                                                                                       | Ogni fase è reversibile e si vede esattamente cosa cambia                                                                                                                                                                                     |
| 2026-10-18 | Suoni del gioco creati con Web Audio: mossa, presa, fine partita; impostazione "Suoni"                                                                                                           | Richiesta di Alessandro. Nessun file né licenza (i suoni classici di Lichess non sono liberi); suonano solo avanzando di una mossa                                                                                                            |
| 2026-10-18 | `MoveRecord` dice se la mossa è una presa (`capture`)                                                                                                                                            | L'interfaccia sceglie il suono senza conoscere le regole                                                                                                                                                                                      |
| 2026-10-18 | "Annulla" toglie l'ultima mossa della partita; "Nuova partita" senza conferma                                                                                                                    | Comportamenti confermati da Alessandro                                                                                                                                                                                                        |
| 2026-10-18 | Fase 2 approvata; `PASSAGGIO-CONSEGNE.md` cancellato                                                                                                                                             | Conferma di Alessandro; appunti per le fasi future e trappole note riportati in §11 e §12                                                                                                                                                     |
| 2026-10-18 | Ritocchi alla scacchiera: pallini con dissolvenza, anelli sulle prese, colori ottone, pezzo sollevato con ombra, cursore a mano                                                                  | Richiesta di Alessandro: più moderna e interattiva, senza appesantire (si animano solo opacità e dimensione dell'immagine)                                                                                                                    |
| 2026-10-18 | Stockfish 19 scaricato da `npm run engines` (Node), con impronta SHA-256 verificata; binari fuori da git                                                                                         | Scelta 1A di Alessandro: uguale sui tre sistemi, niente file pesanti in git; parte da solo con `check` e `tauri`                                                                                                                              |
| 2026-10-18 | Stockfish avviato dalla crate `engine` con `tokio::process`, senza `tauri-plugin-shell`                                                                                                          | Il motore resta indipendente da Tauri e testabile; nessun permesso da dare all'interfaccia                                                                                                                                                    |
| 2026-10-18 | Due Stockfish separati: uno gioca, uno analizza; avviati al primo uso                                                                                                                            | L'analisi non rallenta mai l'avversario; nessun costo se non si usano                                                                                                                                                                         |
| 2026-10-18 | 10 livelli con nome ed Elo indicativo                                                                                                                                                            | Scelta 2A di Alessandro                                                                                                                                                                                                                       |
| 2026-10-18 | Analisi spenta di base contro il computer, con interruttore                                                                                                                                      | Scelta 3A di Alessandro: vederla mentre si gioca toglierebbe l'esercizio di calcolo                                                                                                                                                           |
| 2026-10-18 | Tempi: senza orologio, 1+0, 2+1, 3+0, 3+2, 5+0, 5+3, 10+0, 10+5, 15+10, 30+0, 30+20, personalizzato                                                                                              | Approvati da Alessandro                                                                                                                                                                                                                       |
| 2026-10-18 | Nuova crate `game-session`: avversario, orologio, regole su chi può muovere e ritirare le mosse                                                                                                  | La logica di partita resta nel nucleo; il ponte esegue soltanto                                                                                                                                                                               |
| 2026-10-18 | Orologio contato in Rust; parte dopo la prima mossa; con l'orologio le mosse non si ritirano                                                                                                     | Precisione e regole come nei siti di scacchi                                                                                                                                                                                                  |
| 2026-10-18 | Contro il computer o con l'orologio si gioca solo dall'ultima posizione; "annulla" contro il computer ritira anche la sua risposta                                                               | Comportamento prevedibile; torna sempre il turno della persona                                                                                                                                                                                |
| 2026-10-18 | Tempo scaduto con l'avversario senza materiale per vincere = patta                                                                                                                               | Regolamento FIDE                                                                                                                                                                                                                              |
| 2026-10-18 | L'ultima configurazione di partita si salva in `settings.json` (`GameSetup` in `app-settings`)                                                                                                   | La finestra "Nuova partita" ripropone le scelte precedenti anche dopo il riavvio                                                                                                                                                              |
| 2026-10-18 | Fase 3 approvata ("va abbastanza bene")                                                                                                                                                          | Conferma di Alessandro                                                                                                                                                                                                                        |
| 2026-10-18 | Copia FEN/PGN negli appunti: problema noto, rimandato                                                                                                                                            | Nell'app la copia non arriva negli appunti; Alessandro preferisce rimandare. Soluzione proposta: `tauri-plugin-clipboard-manager` (vedi §12)                                                                                                  |
| 2026-10-18 | Nuova Fase 11 "Appunti personali", in fondo alla roadmap                                                                                                                                         | Richiesta di Alessandro: uno spazio per salvare idee e pensieri, da fare alla fine                                                                                                                                                            |
| 2026-10-18 | Fase 4: si salvano tutte le partite con almeno una mossa (computer, due giocatori, incollate)                                                                                                    | Scelta 1A di Alessandro                                                                                                                                                                                                                       |
| 2026-10-18 | Partita con l'orologio ripresa con l'orologio fermo fino alla mossa successiva                                                                                                                   | Scelta 2A di Alessandro: chiudere l'app non fa perdere la partita                                                                                                                                                                             |
| 2026-10-18 | Riconoscimento delle aperture già nella Fase 4 (elenco Lichess, CC0, nomi in inglese)                                                                                                            | Scelta 3A di Alessandro: serve all'archivio e alle statistiche                                                                                                                                                                                |
| 2026-10-18 | Colonne per valutazioni e statistiche già nello schema, riempite nella Fase 8                                                                                                                    | Scelta 4A di Alessandro: lo schema non dovrà cambiare                                                                                                                                                                                         |
| 2026-10-18 | Nuove crate `openings` e `storage`; `game-session` produce la "fotografia" da salvare                                                                                                            | Ogni crate un compito: le regole non sanno del database, il database non sa delle regole                                                                                                                                                      |
| 2026-10-18 | Salvataggi in coda con un unico lavoratore, solo quando la partita cambia davvero                                                                                                                | Nessun blocco durante il gioco; una versione vecchia non sovrascrive mai una nuova                                                                                                                                                            |
| 2026-10-18 | All'avvio si riprende l'ultima partita non finita                                                                                                                                                | Nessuna partita va persa (traguardo della Fase 4)                                                                                                                                                                                             |
| 2026-10-18 | Nuova sezione "Archivio" nella barra laterale                                                                                                                                                    | Rivedere, continuare, eliminare e filtrare le partite                                                                                                                                                                                         |
| 2026-10-18 | File `.pgn` con la finestra di sistema (`tauri-plugin-dialog` 2.8); la lettura accetta codifiche vecchie                                                                                         | Promesso nella Fase 2; nessun permesso da dare all'interfaccia (lo fa Rust)                                                                                                                                                                   |
| 2026-10-19 | Fase 4 approvata                                                                                                                                                                                 | Conferma di Alessandro ("va tutto bene")                                                                                                                                                                                                      |
| 2026-10-19 | Fase 5: osservazioni spente di base contro il computer, con interruttore                                                                                                                         | Scelta 1A di Alessandro                                                                                                                                                                                                                       |
| 2026-10-19 | Minacce calcolate solo dai rilevatori in Rust (non da Stockfish)                                                                                                                                 | Scelta 2A di Alessandro: istantanee e spiegabili; Stockfish servirà agli scenari "e se…" del tutor                                                                                                                                            |
| 2026-10-19 | Tutti i rilevatori della roadmap, con filtri contro i falsi allarmi                                                                                                                              | Scelta 3A di Alessandro; i fatti devono essere veri e utili, non rumore                                                                                                                                                                       |
| 2026-10-19 | Le frasi dei fatti le scrive Rust; le mosse citate restano a parte e l'interfaccia le traduce nella notazione scelta                                                                             | Una sola fonte per il contenuto, la notazione resta una scelta dell'utente                                                                                                                                                                    |
| 2026-10-19 | Fase 5 approvata                                                                                                                                                                                 | Conferma di Alessandro; le frasi dei fatti si ritoccano man mano che le vede                                                                                                                                                                  |
| 2026-10-19 | "Gioca": solo la scacchiera; tutte le schede in una barra laterale a comparsa ("Partita")                                                                                                        | Richiesta di Alessandro: più spazio alla scacchiera mentre si gioca                                                                                                                                                                           |
| 2026-10-19 | La barra si apre col mouse sul bordo destro, con il tasto I o la linguetta; "Fissa" la tiene aperta; Esc la chiude                                                                               | Scelta 1A di Alessandro: funziona anche senza mouse                                                                                                                                                                                           |
| 2026-10-19 | L'annuncio di fine partita compare sopra la scacchiera; "Nuova partita" resta nella barra                                                                                                        | Scelte 2A e 3A di Alessandro                                                                                                                                                                                                                  |
| 2026-10-19 | Cambio di disposizione: la colonna "Partita" torna fissa (sempre aperta, con scorrimento interno); diventa a comparsa il **menu principale** a sinistra (tasto M), che può coprire la scacchiera | Richiesta di Alessandro: la scacchiera deve restare sempre tutta visibile mentre si leggono le schede                                                                                                                                         |
| 2026-10-19 | La barra di valutazione sta accanto alla scacchiera, alta quanto lei, in uno spazio sempre riservato                                                                                             | Comparendo non deve rimpicciolire la scacchiera né allungare la pagina                                                                                                                                                                        |
| 2026-10-19 | Fase 6: circa 100.000 puzzle di Lichess, scelti dallo script `npm run puzzles`                                                                                                                   | Solo puzzle affidabili (giocati ≥ 300 volte, popolarità ≥ 85, rating ben misurato), distribuiti in modo equilibrato tra difficoltà (400–2999) e lunghezze (1–6 mosse); il file (15 MB) sta nel repository, così chi compila non scarica nulla |
| 2026-10-19 | I puzzle restano in memoria, inclusi nell'eseguibile; nel database solo tentativi e ripasso                                                                                                      | Cambio rispetto al piano (che prevedeva l'importazione nel database con barra di avanzamento): leggerli all'avvio richiede una frazione di secondo, la scelta è istantanea e non serve nessuna importazione                                   |
| 2026-10-19 | Difficoltà adattiva: rating dei puzzle personale (Elo, K = 32, parte da 1500), con la scelta "più facile / normale / più difficile"                                                              | Il puzzle proposto sta vicino al proprio livello (−150 / +100 / +350 punti); se non ce ne sono si allarga la ricerca                                                                                                                          |
| 2026-10-19 | Come su Lichess, un errore, un aiuto o la soluzione mostrata fanno contare il puzzle come sbagliato; si può comunque riprovare                                                                   | Il rating misura ciò che si trova da soli, ma si impara anche dagli errori                                                                                                                                                                    |
| 2026-10-19 | È giusta anche una mossa diversa da quella prevista se dà scacco matto                                                                                                                           | Alcuni puzzle hanno più di un matto: non si punisce una mossa vincente                                                                                                                                                                        |
| 2026-10-19 | Visualizzazione: i pezzi spariscono dopo 2–15 secondi (5 la prima volta, la scelta si ricorda); si gioca con due clic, senza i pallini delle case                                                | I pallini rivelerebbero dove sono i pezzi; le mosse giocate restano scritte nella scheda "Puzzle"                                                                                                                                             |
| 2026-10-19 | "Cosa minaccia?": la scacchiera è girata dalla parte di chi difende e si indovina la prima mossa dell'avversario                                                                                 | Allena a vedere le minacce prima di muovere                                                                                                                                                                                                   |
| 2026-10-19 | Ripasso con le scatole di Leitner: 1, 3, 7, 14, 30 giorni; un nuovo errore riporta alla prima; il ripasso non cambia il rating                                                                   | Metodo semplice e collaudato; ripassare un puzzle già visto non misura il livello                                                                                                                                                             |
| 2026-10-19 | A puzzle finito si mostrano rating, difficoltà, temi in italiano, soluzione e i fatti dei rilevatori a favore di chi risolve                                                                     | "Perché funziona" usa i rilevatori della Fase 5, senza motore                                                                                                                                                                                 |
| 2026-10-19 | La scelta della promozione e la scrittura di varianti e fatti diventano condivise (`components/promotion-picker`, `lib/formatLine`, `lib/formatFact`)                                            | Servono sia in "Gioca" sia in "Allenamento"                                                                                                                                                                                                   |
| 2026-10-19 | Fase 6 approvata                                                                                                                                                                                 | Test manuali superati                                                                                                                                                                                                                         |
| 2026-10-19 | Fase 7: base di conoscenza in `knowledge/`, una scheda TOML per argomento (57 schede: principi, strutture, aperture, mediogioco, finali)                                                         | Testi leggibili e modificabili senza programmare; `build.rs` le include tutte nell'app, i test controllano che siano valide e collegate                                                                                                       |
| 2026-10-19 | Una scheda compare solo se una regola verificabile dice che vale (`structure:…`, `opening:<nome Lichess>`, `endgame:…`, `principle:…`, `middlegame:…`)                                           | Il tutor non inventa: racconta solo ciò che riconosce sulla scacchiera                                                                                                                                                                        |
| 2026-10-19 | Spiegazione a parti: situazione, cosa succede adesso, piano, da ricordare; più le schede da aprire                                                                                               | Dal concreto (minacce) al generale (piani e principi)                                                                                                                                                                                         |
| 2026-10-19 | Il tutor sta in una barra a comparsa a destra (tasto T), uguale in ogni sezione, con una sua piccola scacchiera                                                                                  | Non copre mai la scacchiera principale; le case delle frasi e le mosse provate si vedono lì                                                                                                                                                   |
| 2026-10-19 | Scenari "e se…" sulla scacchiera del tutor: la partita vera non cambia; la promozione è sempre a donna                                                                                           | Provare idee senza rovinare la partita                                                                                                                                                                                                        |
| 2026-10-19 | Un terzo Stockfish solo per il tutor, con valutazioni di mezzo secondo (interruttore "Giudizio di Stockfish")                                                                                    | Non interrompe l'analisi in tempo reale né il computer avversario                                                                                                                                                                             |
| 2026-10-19 | Contro il computer, a partita in corso, il tutor è spento ma si può accendere comunque; nei puzzle si accende solo a puzzle finito                                                               | Sarebbe un aiuto / rivelerebbe la soluzione                                                                                                                                                                                                   |
| 2026-10-19 | Sezione «Tutor» nel menu: la partita di «Gioca» con i commenti dopo ogni mossa, in popup sopra la scacchiera, uno alla volta, con le case mostrate sulla scacchiera                              | Richiesta di Alessandro: imparare mentre si gioca                                                                                                                                                                                             |
| 2026-10-19 | Ordine dei messaggi: commento alle mosse appena giocate (anche la risposta del computer), minacce attuali, poi solo le novità                                                                    | Evitare di ripetere a ogni mossa le stesse frasi                                                                                                                                                                                              |
| 2026-10-19 | Popup scorsi a mano (Avanti/Indietro, Invio/Spazio, Esc), in basso e spostabili in alto; giudizio di Stockfish acceso di base                                                                    | Si legge al proprio ritmo, senza coprire la zona interessante                                                                                                                                                                                 |
| 2026-10-19 | Cambio: i messaggi della modalità Tutor stanno in una fascia sotto la scacchiera (non sopra); la scacchiera è un po' più piccola                                                                 | Richiesta di Alessandro: vedere sempre tutta la scacchiera                                                                                                                                                                                    |
| 2026-10-19 | La colonna di destra è alta quanto l'area della scacchiera, con scorrimento interno (`contain: size`); la pagina non scorre                                                                      | Lo scroll della pagina era eccessivo                                                                                                                                                                                                          |
| 2026-10-19 | Cambio: niente più barra a comparsa del tutor; in cima alla colonna di destra un selettore «Partita \| Tutor» («Allenamento \| Tutor»), tasto T                                                  | Prima la barra si sovrapponeva alla colonna                                                                                                                                                                                                   |
| 2026-10-19 | Fase 7 approvata                                                                                                                                                                                 | Test manuali superati                                                                                                                                                                                                                         |
| 2026-10-19 | Fase 8: l'analisi parte con «Analizza» (Archivio e fine partita in «Gioca»), in sottofondo, interrompibile; il risultato si salva                                                                | Scelta 1A: niente lavoro del processore quando non serve; la seconda volta la revisione è immediata                                                                                                                                           |
| 2026-10-19 | Un quarto Stockfish dedicato all'analisi delle partite, circa 1 secondo per posizione                                                                                                            | Non disturba il computer avversario né il tutor; una partita di 40 mosse richiede circa un minuto e mezzo                                                                                                                                     |
| 2026-10-19 | Giudizio delle mosse dalla probabilità di vittoria persa (curva di Lichess): imprecisione ≥ 10, errore ≥ 20, errore grave ≥ 30 punti; precisione con la formula di Lichess                       | Due pedoni persi in una posizione già vinta contano poco, in una pari moltissimo                                                                                                                                                              |
| 2026-10-19 | Momenti critici: le 5 mosse che perdono di più, spiegate con gli scenari del tutor (cosa concedeva la mossa, cosa otteneva la migliore)                                                          | Imparare dai propri errori con i fatti verificati                                                                                                                                                                                             |
| 2026-10-19 | Grafico della partita disegnato da noi in SVG, con i colori del tema                                                                                                                             | La libreria dei grafici si sceglie nella Fase 9; qui ne basta uno semplice                                                                                                                                                                    |
| 2026-10-19 | Gli errori gravi delle proprie partite diventano puzzle (posizione prima della mossa dell'avversario, una mossa da trovare), subito in ripasso; contro il computer solo i propri                 | Extra 6 approvato: è il modo più diretto di imparare dalle proprie partite                                                                                                                                                                    |
| 2026-10-19 | La revisione si ricalcola ogni volta dai dati salvati (valutazioni e mossa migliore di ogni mossa)                                                                                               | Nel database restano solo i dati grezzi, già pronti per le statistiche della Fase 9                                                                                                                                                           |
| 2026-10-19 | Le varianti nella lista mosse si fanno in una Fase 8-bis separata                                                                                                                                | Scelta 5A: cambiano il cuore della partita, il salvataggio e il PGN                                                                                                                                                                           |
| 2026-10-19 | Fase 8 approvata                                                                                                                                                                                 | Test manuali superati                                                                                                                                                                                                                         |

---

## 11. Appunti tecnici per le fasi successive

Base per il piano di ogni fase: va sempre presentato ad Alessandro e approvato prima di scrivere codice.

- **Fase 3 — Motore.** Fatta (vedi registro §10). Da riprendere più avanti: più varianti nell'analisi (`MultiPV`) e il motore "umano" Maia (Fase 10).
- **Fase 4 — Database.** Fatta (vedi registro §10). Per la Fase 8: riempire le colonne `eval_*`, `best_move_uci`, `centipawn_loss`, `game_phase`, `tactical_themes`, `missed_threats` della tabella `moves` (vedi `crates/storage/migrations/`).
- **Fase 5 — Analisi.** Fatta (vedi registro §10). Per la Fase 8: riempire `tactical_themes` e `missed_threats` con questi rilevatori. Limite noto: lo scambio statico ignora i pezzi inchiodati.
- **Fase 6 — Allenamento.** Fatta (vedi registro §10). Da valutare più avanti: puzzle dalle proprie partite (Fase 8), statistiche per tema (Fase 9), animare la risposta dell'avversario con una piccola pausa.
- **Fase 7 — Tutor.** Fatta (vedi registro §10). Da arricchire nel tempo: altre schede (aperture meno comuni, finali specifici), frasi più chiare man mano che Alessandro le legge.
- **Fase 8 — Analisi partite.** Fatta (vedi registro §10).
- **Fase 8-bis — Varianti.** Una mossa giocata da una posizione passata crea una variante invece di sostituire il seguito: albero di mosse in `Game` (chess-core), salvataggio delle varianti, PGN con varianti, lista mosse ad albero.
- **Fase 9 — Statistiche.** Crate `crates/stats`; libreria dei grafici da scegliere (offline, rispettosa dei temi).
- **Fase 10 — Estensioni.** Maia via lc0 (sidecar), AI locale (es. Ollama) che racconta senza decidere, installatori firmati e aggiornamenti (`tauri-plugin-updater`), CSP restrittiva.
- **Fase 11 — Appunti personali.** Richiesta di Alessandro, da fare alla fine: note libere (idee, pensieri, studio), salvate nel database della Fase 4, con ricerca; collegabili a una partita o a una posizione.
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
- **ts-rs:** i numeri `u64` diventano `bigint` in TypeScript; per i dati verso l'interfaccia si usa `u32` (o `usize`).
- **Tauri sidecar:** la compilazione (anche Clippy) fallisce se il file di Stockfish manca: `npm run engines`.
- **tokio `select!`:** per leggere il motore si usa `Lines::next_line`, che si può interrompere senza perdere righe; i comandi `stop` vanno inviati subito (non in un compito separato) per non arrivare dopo una richiesta più recente.
- **Clippy pedantic** vieta più di tre `bool` in una struttura: in `SessionView` è permesso, con la spiegazione.
- **Shell:** comandi molto lunghi con più `heredoc` e apostrofi possono rompersi; i file si scrivono con lo strumento di scrittura.
- **Problema noto — appunti:** "Copia FEN" e "Copia PGN" usano `navigator.clipboard.writeText`, che dentro l'app non scrive negli appunti (probabilmente perché il motore web la blocca quando non parte nell'istante del clic). Soluzione proposta e rimandata: il plugin ufficiale `tauri-plugin-clipboard-manager`, con il solo permesso di scrittura.
- **sqlx 0.9:** le query costruite con `format!` non sono accettate (protezione contro l'SQL injection): si usano testi fissi o `QueryBuilder` (senza parametro di durata, `QueryBuilder<Sqlite>`). Con SQLite in memoria serve una sola connessione (`max_connections(1)`), altrimenti ogni connessione vede un database diverso.
- **Comandi Tauri sincroni** girano sul thread principale: le finestre di sistema "bloccanti" (`blocking_save_file`) vanno aperte in un comando `async`, dentro `spawn_blocking`.
- **Aperture:** i nomi di Lichess sono in inglese; la ricerca nell'archivio va fatta in inglese ("Sicilian") o per codice ECO.
- **Database dei puzzle di Lichess:** è compresso con `pzstd`, che aggiunge "frame saltabili" che `zlib.createZstdDecompress` di Node (≥ 22.15) non accetta: lo script li toglie prima di decomprimere.
- **Schede di `knowledge/`:** le include `build.rs` della crate `tutor`; un file nuovo si vede dopo la ricompilazione. Nei TOML i campi semplici (`ideas`, `mistakes`) vanno prima delle tabelle `[[plans]]`, altrimenti finiscono dentro l'ultimo piano. Le virgolette nei testi si scrivono «così».
- **Colonna di destra alta quanto la scacchiera:** `contain: size` sulla colonna (e `align-content: start` sulla griglia): la colonna non conta nel calcolo dell'altezza della riga e scorre al suo interno.
- **TanStack Router:** una pagina che legge l'indirizzo con `useSearch({ strict: false })` funziona anche nei test (`renderWithRouter`), senza dipendere dalla rotta.
- **Puzzle di Lichess:** la prima mossa è dell'avversario; il puzzle comincia dopo. La "profondità" è il numero di mosse di chi risolve (mosse ÷ 2).
- **Gusti di Alessandro:** cornici attorno alla scacchiera, pezzi puliti e minimal, tanta scelta di stili, ogni scelta valida in tutta l'app; a ogni fase chiede ritocchi estetici.
