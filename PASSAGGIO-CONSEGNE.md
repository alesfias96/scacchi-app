# PASSAGGIO-CONSEGNE.md — Stato del lavoro e cose da fare

> Documento di passaggio scritto il **2026-10-18** alla fine della sessione che ha costruito
> le Fasi 1, 1-bis e 2. Serve a una nuova sessione di lavoro (persona o Claude Code CLI)
> per ripartire **senza perdere nulla**.
>
> **Ordine di lettura:** questo file → `PROGETTO.md` (le regole, vincolanti) →
> `README.md` della radice → i README delle cartelle che si toccano.
>
> Se questo file e `PROGETTO.md` dicono cose diverse, vale `PROGETTO.md`. Quando questo
> documento non serve più (Fase 2 chiusa e informazioni riportate in `PROGETTO.md`),
> si cancella, come prevede la regola sul codice morto.

---

## Indice

1. [Il progetto in una pagina](#1-il-progetto-in-una-pagina)
2. [Regole di lavoro da rispettare sempre](#2-regole-di-lavoro-da-rispettare-sempre)
3. [Ambiente, comandi e flusso di lavoro](#3-ambiente-comandi-e-flusso-di-lavoro)
4. [Stato delle fasi](#4-stato-delle-fasi)
5. [Cosa è stato fatto, nel dettaglio](#5-cosa-è-stato-fatto-nel-dettaglio)
6. [**Prossimo passo: chiudere la Fase 2**](#6-prossimo-passo-chiudere-la-fase-2)
7. [Le fasi successive (3–10) nel dettaglio](#7-le-fasi-successive-310-nel-dettaglio)
8. [Decisioni ancora aperte](#8-decisioni-ancora-aperte)
9. [Trappole note e lezioni imparate](#9-trappole-note-e-lezioni-imparate)

---

## 1. Il progetto in una pagina

**Cos'è:** app desktop di scacchi **completamente offline** per Windows, macOS e Linux.
È di Alessandro, che scrive in italiano, sta imparando Rust e vuole capire ogni scelta.

**Obiettivi, in ordine di importanza:**

1. Allenare il **calcolo**: prevedere le mosse dell'avversario e ragionare più mosse avanti.
2. Un **tutor "master"** che spiega ogni posizione per intero (minacce, piani, scenari,
   strategie col bianco e col nero), non solo le mosse "da libro". Deve essere richiamabile
   in ogni schermata.
3. **Giocare** contro motori forti: Stockfish, e Maia per un avversario "umano".
4. **Statistiche avanzatissime** su tutto ciò che serve a migliorare.
5. Interfaccia **molto moderna e reattiva**: niente si blocca mentre il motore pensa.

**Cartella del progetto:**

- su Windows: `C:\Users\afias\Desktop\Scacchi_app`
- Alessandro lavora anche su un Mac M1, quindi l'app deve funzionare identica su tutti e tre i sistemi.

**Stack (tutte scelte approvate):**

| Ambito            | Scelta                                                                             |
| ----------------- | ---------------------------------------------------------------------------------- |
| Contenitore       | Tauri **2.12**                                                                     |
| Nucleo            | Rust, edizione **2024**, workspace con resolver 3                                  |
| Regole scacchi    | `shakmaty` **0.30**                                                                |
| Interfaccia       | React **19** + TypeScript **5.9** (strict, `noUncheckedIndexedAccess`)             |
| Build             | Vite **7**                                                                         |
| Test interfaccia  | Vitest **5** + Testing Library + jsdom                                             |
| Scacchiera        | `@lichess-org/chessground` (nel lockfile: 10.4.2), avvolta in `Chessboard`         |
| Stato interfaccia | Zustand 5                                                                          |
| Navigazione       | TanStack Router v1 con cronologia a hash (`/` e `/impostazioni`)                   |
| Tipi Rust → TS    | `ts-rs` **12**, generati in `src/bindings/` quando si esegue `cargo test`          |
| CSS               | CSS Modules + design token (un'unica metodologia)                                  |
| Qualità           | rustfmt, Clippy `pedantic` con `-D warnings`, ESLint `strictTypeChecked`, Prettier |
| Requisiti         | Node ≥ 22.12, Rust stabile                                                         |

**Architettura a tre strati:**

| Strato      | Cartella     | Ruolo                                                                      |
| ----------- | ------------ | -------------------------------------------------------------------------- |
| Interfaccia | `src/`       | Mostra e raccoglie i clic. Non conosce le regole degli scacchi.            |
| Ponte       | `src-tauri/` | Contiene solo comandi, eventi e stato in memoria, nessuna logica.          |
| Nucleo      | `crates/`    | Una crate per responsabilità. Ognuna si testa da sola e non conosce Tauri. |

---

## 2. Regole di lavoro da rispettare sempre

Sono in `PROGETTO.md` §2, §5 e §7, e Alessandro le considera **vincolanti**. In sintesi:

1. **Prima analisi completa, poi piano con le strade possibili, poi autorizzazione esplicita.**
   Nessuna modifica ai file finché Alessandro non dice di procedere ("vai", "va bene", …).
2. **Una fase della roadmap per ogni passo di lavoro.** È una sua richiesta esplicita:
   "fai una fase a ogni step altrimenti dura troppo". Va proposta una fase intera, non mezza.
3. **A ogni fase va dato l'elenco dei test da fare a mano**, oltre ai test automatici.
4. **Un file = una responsabilità.** Ogni componente sta nella sua cartella con:
   - `Nome.tsx`, `Nome.module.css`, `Nome.test.tsx`;
   - un `README.md`;
   - le funzioni di supporto in file separati, una per file.
5. **Un README in ogni cartella**, comprensibile anche a chi non programma. Deve descrivere
   ogni file della cartella e va aggiornato quando un file viene aggiunto o tolto.
6. **Commenti esaustivi** in italiano su ogni funzione e componente: cosa fa, cosa riceve,
   cosa restituisce, perché esiste.
   - Rust: `missing_docs` è attivo; servono le sezioni `# Errors` e `# Panics` dove servono.
   - I **nomi** nel codice sono in inglese.
7. **Il codice morto si cancella**, non si commenta. Le eccezioni: lo chiede lui, oppure
   cancellarlo romperebbe l'app (e allora si annota il motivo).
8. **CSS: una sola metodologia.**
   - I valori grezzi (colori, misure) esistono **solo** in `src/styles/tokens.css` e
     `src/styles/themes/*.css`; nei moduli si usa solo `var(--…)`.
   - Vietati gli stili inline e `!important`.
   - Le classi condizionali si scrivono con `classNames()`.
   - Unica eccezione: gli stili globali della scacchiera (vedi `src/components/chessboard/README.md`).
9. **Le regole degli scacchi vivono solo in Rust.** I nomi dei comandi Rust compaiono **solo**
   in `src/services/`.
10. **Multipiattaforma sempre** (`PROGETTO.md` §5-bis):
    - percorsi solo tramite le API di Tauri;
    - motori esterni come sidecar;
    - nessuno script bash o PowerShell.
11. **`npm run check` verde** prima di considerare finita una fase.
12. **`PROGETTO.md` si aggiorna** a ogni decisione: stato in §9, registro decisioni in §10,
    struttura in §6.
13. Tono con Alessandro: italiano, spiegazioni semplici; sta imparando, quindi le scelte tecniche
    vanno motivate in parole chiare.

---

## 3. Ambiente, comandi e flusso di lavoro

### Comandi (dalla cartella del progetto)

| Comando               | A cosa serve                                                                     |
| --------------------- | -------------------------------------------------------------------------------- |
| `npm install`         | Installa le librerie dell'interfaccia                                            |
| `npm run check`       | **Tutti** i controlli: rustfmt, Clippy, test Rust, Prettier, ESLint, tsc, Vitest |
| `npm run tauri dev`   | Avvia l'app in sviluppo                                                          |
| `npm run test:rust`   | Esegue i test Rust e **rigenera `src/bindings/*.ts`**                            |
| `npm run format`      | Prettier in scrittura                                                            |
| `npm run rust:format` | rustfmt in scrittura                                                             |

### Cose da sapere

- **Il progetto NON è sotto git** (non esiste una cartella `.git`). È consigliato proporre ad
  Alessandro un `git init` con un primo commit **prima** di iniziare a modificare, così ogni
  fase è reversibile. Il CI (`.github/workflows/ci.yml`) è pronto ma non è mai partito,
  perché non c'è un repository GitHub.
- **I tipi in `src/bindings/` non si scrivono a mano**: li genera `ts-rs` durante
  `cargo test --all-features`. Le Fasi 1 e 1-bis sono state generate davvero. Quelli della
  Fase 2 li ho **scritti a mano** imitando il generatore, perché non potevo compilare: il primo
  `npm run check` li riscriverà, ed è normale. Il CI controlla che siano aggiornati con
  `git diff --exit-code -- src/bindings`.
- Prettier ignora `src/bindings/*.ts` e riformatta le tabelle Markdown: dopo aver scritto un
  README va eseguito `npm run format`.
- La configurazione `ts-rs` è in `.cargo/config.toml`:
  `TS_RS_EXPORT_DIR = { value = "src/bindings", relative = true }`.
- Ogni tipo esportato usa questo schema:
  `#[cfg_attr(feature = "bindings", derive(serde::Serialize, ts_rs::TS), serde(rename_all = "camelCase"), ts(export))]`.
  Fanno eccezione i tipi di `app-settings`, che derivano sempre `Serialize` e `Deserialize`
  perché vengono salvati su file.
- In `chessground` la versione nel lockfile (10.4.2) dichiara `engines: node >= 24`. Con
  Node 22 compare solo un avviso: non blocca nulla, ma va tenuto presente.

---

## 4. Stato delle fasi

| Fase                            | Stato                                                        |
| ------------------------------- | ------------------------------------------------------------ |
| 1. Fondamenta                   | ✅ completata (check verde sul PC di Alessandro)             |
| 1-bis. Temi e personalizzazione | ✅ completata e approvata ("Mi piace tutto, andiamo avanti") |
| 2. Regole e partita             | 🔵 **codice scritto, MAI compilato né testato** — vedi §6    |
| 3–10                            | ⚪ da fare                                                   |

---

## 5. Cosa è stato fatto, nel dettaglio

### 5.1 Fase 1 — Fondamenta

- **Workspace Rust** (`Cargo.toml` della radice):
  - membri: `src-tauri`, `crates/chess-core`, `crates/app-settings`;
  - regole: `unsafe_code = forbid`, `missing_docs = warn`, Clippy `all` + `pedantic`
    (unica eccezione: `module_name_repetitions` è permesso).
- **Controlli di qualità** come da `PROGETTO.md` §7.1, più il **CI** su Windows, macOS e
  Linux in `.github/workflows/ci.yml`.
- **`crates/chess-core`, prima versione** (un file per tipo):
  - `Side`;
  - `LegalMove { from, to, uci }`;
  - `PositionView { fen, board_fen, turn, in_check, legal_moves }`;
  - `setup.rs` con `initial_position()`, `position_from_fen()` e `pub(crate) parse_fen()`;
  - `SetupError`.
- **Guscio grafico**:
  - `src/app/`: `App`, `SettingsGate`, router, `RootLayout`, `AppLayout`, `Sidebar` con
    sezioni attive e "in arrivo", `ThemeController`;
  - token e temi CSS;
  - componente `Chessboard` (involucro di chessground).

### 5.2 Fase 1-bis — Temi e personalizzazione

- **Temi** (`src/styles/themes/`):
  - **Studio** (predefinito, font Outfit), **Sera** (Manrope), **Giorno** (Plus Jakarta Sans),
    più **Automatico**, che segue il sistema;
  - il tema si applica come `data-theme` su `<html>`.
- **6 scacchiere** in `src/assets/boards/*.jpg`, generate apposta:
  - Rovere (predefinita), Noce, Acero, Marmo, Ardesia, Torneo;
  - ognuna ha la sua cornice colorata (`--frame-*`).
- **6 set di pezzi** in `src/assets/pieces/<stile>/{w,b}{K,Q,R,B,N,P}.svg`:
  - Icona (predefinito), Morbido, Legno, Metallo, Gettoni, Classico (cburnett);
  - CSS in `src/components/chessboard/themes/`, attivato da `data-board` e `data-pieces`.
- **Le scelte valgono in tutta l'app**: `Chessboard` legge da solo le impostazioni dallo store.
- **Crate `crates/app-settings`**:
  - tipi `AppTheme`, `BoardStyle`, `PieceStyle`, `Settings`;
  - `store.rs` con `load`, `load_or_default` (non blocca mai) e `save`, sul file
    `settings.json` nella `app_config_dir` di Tauri.
- **Comandi** `get_settings` e `save_settings`; pagina **Impostazioni** con `OptionGroup` e anteprime.
- **FEN tolta dal pannello di Gioca**, su richiesta di Alessandro.

### 5.3 Fase 2 — Regole e partita (scritta in questa sessione)

Requisiti approvati da Alessandro ("Va bene, vai"):

- partita a due giocatori con tutte le regole;
- scelta del pezzo nella promozione, nello stile di pezzi attivo;
- fine partita riconosciuta: matto, stallo, materiale insufficiente, triplice ripetizione,
  50 mosse;
- lista mosse navigabile: clic, ←/→, Home/Fine;
- comandi nuova partita, annulla, gira scacchiera;
- copia FEN, copia PGN, "incolla posizione o partita";
- notazione predefinita **a figurine**, con impostazione Figurine / Italiana (R D T A C) /
  Internazionale (K Q R B N);
- una mossa giocata da una posizione passata **sostituisce il seguito** (le varianti arrivano
  nella Fase 8);
- il salvataggio su file `.pgn` è rimandato alla Fase 4.

#### Rust — `crates/chess-core/src/`

| File             | Contenuto                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| ---------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `game.rs`        | `Game { positions: Vec<Chess>, hashes: Vec<Zobrist64>, moves: Vec<MoveRecord>, cursor }`. Metodi: `new`, `from_position`, `from_fen`, `moves`, `starting_fen`, `play_uci` (errore `GameOver` se la posizione mostrata è finale), `pub(crate) play_move` (tronca dopo il cursore e calcola la SAN), `undo` (toglie l'ultima mossa e porta il cursore in fondo), `go_to(ply)` (limitato alla fine), `result`, `final_result`, `view`, `pub(crate) current`. La fine partita usa `Outcome` di shakmaty; la ripetizione è contata con lo Zobrist, le 50 mosse con `halfmoves() >= 100`. 17 test. |
| `game_view.rs`   | `GameView { position, moves, cursor, last_move: Option<LegalMove>, result: Option<GameResult>, starting_fen }`                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `move_record.rs` | `MoveRecord { ply, move_number, side, san (sempre internazionale), uci, from, to }`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `game_result.rs` | `GameEndReason { Checkmate, Stalemate, InsufficientMaterial, ThreefoldRepetition, FiftyMoveRule }` e `GameResult { winner: Option<Side>, reason, score }`, con `decisive()` e `draw()`                                                                                                                                                                                                                                                                                                                                                                                                       |
| `game_error.rs`  | `GameError { IllegalMove(String), GameOver, InvalidPosition(SetupError), InvalidPgn(String), EmptyImport }`, messaggi in italiano                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `pgn_export.rs`  | `to_pgn(&Game)`: intestazioni Seven Tag Roster, più `SetUp`/`FEN` se la partenza non è standard; numerazione corretta anche se inizia il nero; righe di 80 caratteri. 4 test.                                                                                                                                                                                                                                                                                                                                                                                                                |
| `pgn_import.rs`  | `from_pgn(&str)`: legge l'intestazione FEN; ignora commenti `{}` e `;`, varianti `()` annidate, `$n`, `!?`, numeri e risultato; accetta `0-0`. 7 test, compresa la verifica "esporta e rileggi".                                                                                                                                                                                                                                                                                                                                                                                             |
| `game_import.rs` | `import_game(&str)`: testo vuoto → `EmptyImport`; se sembra una FEN (una riga, niente `[` né `.`, 7 `/`) usa `from_fen`, altrimenti `from_pgn`. 3 test.                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `lib.rs`         | Moduli e `pub use` di tutti i tipi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |

#### Rust — impostazioni e ponte

- **`crates/app-settings`:**
  - nuovo `notation.rs`: `Notation { Figurine (predefinita), Italiana, Internazionale }`;
  - `Settings` ha il campo `notation`; i test sono aggiornati.
- **`src-tauri/src/state/game_state.rs`:**
  - `GameState { game: Mutex<Game> }`;
  - `with_game(|g| …)` è tollerante al Mutex "avvelenato";
  - registrato con `.manage(GameState::default())`.
- **`src-tauri/src/commands/game.rs`** (registrati in `lib.rs`). Ognuno restituisce `GameView`,
  oppure `Result<GameView, String>` dove può fallire:
  - `get_game`, `new_game`, `undo_move`;
  - `play_move(uci)`, `go_to_ply(ply)`;
  - `import_game_text(text)`: si chiama così perché `import_game` è già il nome della funzione
    di chess-core;
  - `export_pgn` restituisce una `String`;
  - `#![allow(clippy::needless_pass_by_value)]` a livello di modulo, con la spiegazione.

#### Interfaccia (`src/`)

| Dove                                         | Cosa                                                                                                                                                                                                                                         |
| -------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `bindings/`                                  | Nuovi `GameView`, `MoveRecord`, `GameResult`, `GameEndReason`, `Notation`; `Settings` aggiornato (scritti a mano, vedi §3)                                                                                                                   |
| `services/gameService.ts`                    | `fetchGame`, `startNewGame`, `playMove`, `undoMove`, `goToPly`, `importGame`, `exportPgn`                                                                                                                                                    |
| `state/gameStore.ts`                         | Zustand: `game`, `loadError`, `actionError`; azioni `load`, `newGame`, `play`, `undo`, `goTo`, `importText` (questa rilancia l'errore alla finestra "Incolla")                                                                               |
| `components/chessboard/`                     | `Chessboard` diventa interattiva: props `lastMove`, `legalMoves`, `onMove`, `resyncKey` (per rimettere a posto i pezzi dopo una promozione annullata o una mossa rifiutata); `buildDests.ts`, `isSquareKey.ts`                               |
| `components/piece-icon/`                     | `pieceCode.ts` (tipi), `pieceImageUrl.ts` (`import.meta.glob` di tutti gli SVG), `PieceIcon.tsx`                                                                                                                                             |
| `components/panel-card/`                     | `PanelCard`: scheda con titolo, collegata a `aria-labelledby`                                                                                                                                                                                |
| `components/button/`                         | `Button` nelle varianti `primary`, `secondary` e `icon` (in `icon` l'`aria-label` fa anche da suggerimento)                                                                                                                                  |
| `components/icon/`                           | `Icon` e `iconPaths.ts`: first, previous, next, last, flip, undo, plus, copy, paste                                                                                                                                                          |
| `features/play/PlayPage.tsx`                 | Collega tutto: orientamento, promozione in sospeso, `resyncKey`, tastiera; a partita finita passa `legalMoves` vuoto                                                                                                                         |
| `features/play/move-list/`                   | `MoveList` (scorre da sola alla mossa evidenziata), `groupMovesIntoRows.ts`                                                                                                                                                                  |
| `features/play/game-controls/`               | Navigazione ⏮◀▶⏭; Nuova partita, Annulla, Gira                                                                                                                                                                                               |
| `features/play/promotion-picker/`            | Velo sopra la scacchiera con 4 `PieceIcon`; la donna ha il focus; Esc o clic fuori annulla                                                                                                                                                   |
| `features/play/game-result-banner/`          | `GameResultBanner` e `describeResult.ts` (½-½ per le patte)                                                                                                                                                                                  |
| `features/play/import-export/`               | `ImportExport` (Copia FEN, Copia PGN, Incolla, conferma per 2,5 s) e `ImportDialog` (area di testo, errore mostrato sotto)                                                                                                                   |
| `features/play/position-panel/`              | Ora usa `PanelCard`; tolto il segnaposto "Mosse"                                                                                                                                                                                             |
| `features/settings/`                         | Gruppo "Notazione delle mosse": `options/notationOptions.tsx`, `previews/NotationPreview.tsx`; `pieceOptions.tsx` rifatto con `pieceImageUrl`                                                                                                |
| `hooks/useGameKeyboard.ts`                   | ←/→/Home/End; ignorati nei campi di testo, con Ctrl/Alt/Cmd o quando l'hook è disattivato                                                                                                                                                    |
| `lib/formatSan.ts`, `lib/copyToClipboard.ts` | Traduzione della notazione (figurine ♚♛♜♝♞ / R D T A C) e copia negli appunti                                                                                                                                                                |
| `styles/`                                    | Nuovi token in `tokens.css`: `--move-list-max-height`, `--move-number-width`, `--control-size`, `--icon-size`, `--promotion-piece-size`, `--dialog-width`, `--import-text-rows-height`. In ogni tema: `--color-on-accent`, `--color-overlay` |
| `test/setup.ts`                              | Simulazione di `Element.prototype.scrollIntoView`                                                                                                                                                                                            |
| Test esistenti                               | Tutti gli oggetti `Settings` hanno ora anche `notation`; `App.test.tsx` simula anche `gameService`                                                                                                                                           |

**Documentazione aggiornata:**

- tutti i README delle cartelle toccate o nuove;
- `PROGETTO.md`: §6 struttura, §9 (1-bis ✅, 2 🔵) e sei nuove righe nel registro §10.

**Già verificato sul PC:** Prettier `--check` verde, `tsc -b` senza errori, ESLint senza errori
su tutto `src/`.

---

## 6. Prossimo passo: chiudere la Fase 2

La Fase 2 è scritta ma **Rust non è mai stato compilato e Vitest non è mai stato eseguito**:
l'ambiente della sessione precedente non poteva scaricare le librerie. Va quindi chiusa così.

### 6.1 Verifica automatica

1. Alessandro esegue `npm install`, poi `npm run check`.
2. Si correggono gli errori **uno alla volta**, mostrando l'analisi prima di cambiare il codice
   (per correzioni piccole e ovvie Alessandro di solito autorizza subito).
3. Errori probabili da controllare per primi:
   - **Clippy pedantic** su `chess-core/src/game.rs`, `pgn_*.rs` e `commands/game.rs`. Possibili
     avvisi: `doc_markdown` (nomi tecnici senza backtick nei commenti), `must_use_candidate`,
     `redundant_closure_for_method_calls`, `cast_*`, `too_many_lines`, `missing_errors_doc`
     e `missing_panics_doc`.
   - **API di shakmaty 0.30** usate senza averle compilate:
     - `SanPlus::from_move_and_play_unchecked`;
     - `san.san.to_move(&pos)`;
     - `UciMove::from_standard`;
     - `zobrist_hash::<Zobrist64>(EnPassantMode::Legal)`;
     - `Outcome::Known(KnownOutcome::…)`;
     - `fullmoves().get()`.
   - **Binding**: dopo `npm run test:rust` confrontare i file rigenerati con quelli scritti a
     mano. Se cambia un tipo (es. `usize` → `number` o `bigint`), aggiornare l'interfaccia.
   - **Vitest**:
     - `import.meta.glob` con `query: '?url'` dentro jsdom;
     - la simulazione di `navigator.clipboard` con `Object.defineProperty`;
     - eventuali avvisi `act(...)` in `PlayPage.test.tsx`;
     - `vi.resetAllMocks()` combinato con `restoreMocks: true` in `vitest.config.ts`.

### 6.2 Test a mano (`npm run tauri dev`)

- [ ] **Mosse e notazione:**
  - [ ] 1.e4 e5 2.Cf3 Cc6: le mosse compaiono a figurine;
  - [ ] le case raggiungibili hanno il pallino;
  - [ ] l'ultima mossa è evidenziata.
- [ ] **Arrocco** corto e lungo, trascinando il re di due case. Dopo che il re si è mosso,
      l'arrocco non deve più essere possibile.
- [ ] **En passant:** 1.e4 a6 2.e5 d5 3.exd6. Il pedone catturato deve sparire.
- [ ] **Promozione:**
  - [ ] compare la scelta, nello stile di pezzi attivo;
  - [ ] cavallo → `=♞` nella lista mosse;
  - [ ] Esc → il pedone torna indietro.
- [ ] **Matto del barbiere:** 1.f3 e5 2.g4 Dh4#. Deve comparire "Scacco matto · Vince il Nero ·
      0-1" e la scacchiera si blocca.
- [ ] **Patte:**
  - [ ] stallo e materiale insufficiente, incollando FEN di prova (es. `7k/5Q2/6K1/8/8/8/8/8 b - - 0 1`);
  - [ ] triplice ripetizione: muovere avanti e indietro i cavalli.
- [ ] **Navigazione:**
  - [ ] clic sulle mosse, ←/→, Home, Fine;
  - [ ] i pulsanti disattivati agli estremi;
  - [ ] una mossa giocata da una posizione passata sostituisce il seguito.
- [ ] **Comandi:** Annulla, Gira scacchiera, Nuova partita.
- [ ] **Copia e incolla:**
  - [ ] Copia FEN e Copia PGN, con la conferma;
  - [ ] il PGN incollato su lichess.org/paste deve essere valido;
  - [ ] Incolla un PGN preso da un sito (con commenti e varianti);
  - [ ] Incolla un testo sbagliato: l'errore deve essere chiaro.
- [ ] **Impostazioni:** cambiare la notazione aggiorna subito la lista mosse; la scelta resta
      dopo il riavvio.
- [ ] **Temi:** tutto leggibile in Studio, Sera e Giorno (banner, scelta della promozione,
      finestra "Incolla").
- [ ] **Sul Mac:** clipboard e tastiera funzionano (WKWebView è il motore più restrittivo).

### 6.3 Chiusura

1. Recepire il feedback grafico di Alessandro: a ogni fase ha chiesto ritocchi estetici.
2. `PROGETTO.md`: Fase 2 → ✅, con eventuali decisioni nuove nel registro.
3. Cancellare questo file, o tenerlo aggiornato, come Alessandro preferisce.

### 6.4 Comportamenti da confermare con Alessandro

- **"Annulla" mentre si guarda una posizione passata** toglie l'ultima mossa della partita e
  porta all'ultima posizione. L'alternativa sarebbe togliere tutte le mosse dopo quella mostrata.
- **"Nuova partita"** non chiede conferma.
- **La partita in memoria si perde** quando si chiude l'app; il salvataggio arriva nella Fase 4.
- **Copia negli appunti:** usa `navigator.clipboard.writeText`. Se su qualche sistema non
  funziona, si passa al plugin ufficiale `tauri-plugin-clipboard-manager` (con il suo permesso
  in `src-tauri/capabilities/default.json`).

---

## 7. Le fasi successive (3–10) nel dettaglio

La roadmap ufficiale è in `PROGETTO.md` §8. Qui sotto ci sono i dettagli tecnici da usare come
**base per il piano** di ogni fase: vanno sempre presentati ad Alessandro e approvati prima di
scrivere codice.

### Fase 3 — Motore (traguardo: si gioca contro Stockfish a qualsiasi livello)

- **Nuova crate `crates/engine`**: dialogo UCI generico, cioè un "trait" `Engine` e un'unica
  implementazione UCI. Comandi:
  - `uci` / `isready` / `ucinewgame`;
  - `position fen … moves …`;
  - `go depth|movetime|wtime/btime`;
  - lettura delle righe `info … score cp|mate … pv …` e di `bestmove`.
    Tutto asincrono (tokio, già incluso da Tauri) e cancellabile (`stop`).
- **Stockfish come sidecar Tauri:**
  - un binario per piattaforma in `src-tauri/binaries/`, con il nome che indica la piattaforma:
    - `stockfish-x86_64-pc-windows-msvc.exe`;
    - `stockfish-aarch64-apple-darwin`;
    - `stockfish-x86_64-apple-darwin`;
    - `stockfish-x86_64-unknown-linux-gnu`;
  - configurato in `tauri.conf.json` → `bundle.externalBin`;
  - avviato con `tauri-plugin-shell` (sidecar), con il permesso in `capabilities/default.json`;
  - **licenza GPL-3.0**: annotarla, come già fatto per chessground;
  - va deciso come procurarsi i binari: uno script npm multipiattaforma che li scarica, oppure
    binari inseriti a mano. Niente bash o PowerShell (`PROGETTO.md` §5-bis).
- **Eventi in tempo reale:** nuova cartella `src-tauri/src/events/`. L'analisi live viene
  inviata all'interfaccia come eventi Tauri (`emit`), mai come attesa bloccante.
- **Livelli di gioco:**
  - Stockfish `Skill Level` 0–20, oppure `UCI_LimitStrength` + `UCI_Elo` (1320–3190);
  - tempo per mossa;
  - va concordata una scala comprensibile (es. "Principiante … Maestro … Sovrumano").
- **Interfaccia:**
  - `components/eval-bar/` (barra di valutazione);
  - frecce della mossa migliore (`drawable.autoShapes` di chessground);
  - pannello analisi con la variante principale tradotta nella notazione scelta;
  - impostazione per scegliere il colore con cui si gioca;
  - **orologio**: tempo per partita e incremento, con il conteggio in Rust per precisione.
- **Partita contro il computer:** `PlayPage` passa da "due giocatori" a una scelta della
  modalità. Il computer muove con un evento e la scacchiera resta bloccata durante il suo turno.

### Fase 4 — Database (traguardo: nessuna partita va persa)

- **Nuova crate `crates/storage`:** SQLite tramite `sqlx`, con migrazioni versionate. Il file
  sta nella `app_data_dir` di Tauri.
- **Schema, da progettare prima delle schermate** (`PROGETTO.md` §4.3):
  - partite (giocatori, livello motore, tempo, risultato, PGN, apertura);
  - mosse, con: valutazione prima e dopo, perdita rispetto alla mossa migliore (centipedoni),
    tempo impiegato, fase di gioco, temi tattici presenti, minacce viste o mancate.
- Salvataggio **automatico a ogni mossa** e ripresa della partita all'avvio.
- **Archivio partite** (nuova sezione) con ricerca e filtri.
- Salvataggio e apertura di file `.pgn` con il dialogo di sistema (`tauri-plugin-dialog`),
  come promesso nella Fase 2.

### Fase 5 — Analisi (traguardo: l'app "capisce" la posizione)

- **Nuova crate `crates/analysis`**, con un rilevatore per file:
  - pezzi indifesi e attaccati;
  - minacce (cosa vuole fare l'avversario);
  - tattiche: forchette, inchiodature, infilate, attacchi di scoperta, sovraccarico;
  - struttura pedonale: isolati, doppiati, passati, arretrati, catene;
  - sicurezza del re;
  - attività dei pezzi;
  - fase di gioco.
- Ogni rilevatore deve avere test con posizioni FEN note. Sono il **livello 1 del tutor**
  (fatti verificati, `PROGETTO.md` §4.2).

### Fase 6 — Allenamento calcolo (traguardo: allenamento al calcolo completo)

- **Nuova crate `crates/training`.**
- **Import del database di puzzle di Lichess**: CSV con licenza CC0, in `data/`, filtrabile per
  tema e rating.
- **Modalità di allenamento:**
  - puzzle per **profondità** (2, 3, 4, … mosse);
  - **visualizzazione**: la posizione sparisce dopo N secondi e si gioca "a memoria";
  - **"cosa minaccia?"**: indovinare la mossa dell'avversario;
  - **ripasso degli errori**, con ripetizione dilazionata.
- Nuova sezione "Allenamento" (già presente nella barra laterale, disattivata).

### Fase 7 — Tutor (traguardo: il tutor spiega ogni posizione)

- **Nuova crate `crates/tutor`** e cartella `knowledge/` con testi curati in italiano:
  - strutture pedonali e piani tipici;
  - idee delle aperture **col bianco e col nero**;
  - concetti di mediogioco;
  - finali.
- **Pannello richiamabile ovunque**, con una scorciatoia unica per tutta l'app.
- Il tutor spiega: minacce di entrambi, piani possibili, scenari "e se…" (calcolati con il
  motore), perché una mossa è buona o cattiva.
- **Livello linguistico:** prima a regole/modelli di frase; più avanti (Fase 10) un'AI locale
  che **racconta soltanto** i fatti verificati, senza mai decidere.

### Fase 8 — Analisi partite (traguardo: si impara dalle proprie partite)

- Revisione automatica delle partite salvate:
  - momenti critici;
  - mosse mancate, spiegate dal tutor;
  - grafico della valutazione.
- **Varianti nella lista mosse**: albero delle mosse invece della sostituzione del seguito.
  Richiede di estendere `Game` in chess-core.

### Fase 9 — Statistiche (traguardo: progressi misurabili)

- **Nuova crate `crates/stats`.** Metriche:
  - precisione;
  - perdita media in centipedoni per fase;
  - errori per tema tattico;
  - profondità di calcolo nel tempo;
  - livello stimato;
  - aperture col bianco e col nero.
- Grafici: la libreria è **da scegliere** (vedi §8).

### Fase 10 — Estensioni (traguardo: app completa)

- **Maia via lc0** (sidecar, con i pesi della rete) come avversario "umano" a livelli di rating.
- **AI locale** (es. Ollama) per spiegazioni più naturali, senza mai prendere decisioni.
- Conoscenza avanzata.
- Installatori firmati e aggiornamenti automatici (`tauri-plugin-updater`).
- **Attivare la CSP restrittiva** (Content Security Policy).

---

## 8. Decisioni ancora aperte

| Tema                         | Opzioni / note                                                                  | Quando decidere      |
| ---------------------------- | ------------------------------------------------------------------------------- | -------------------- |
| Git e GitHub                 | `git init` subito; repository privato su GitHub per far partire il CI           | **Subito**           |
| Come procurarsi Stockfish    | Script npm che scarica i binari ufficiali per piattaforma, oppure binari a mano | Fase 3               |
| Scala dei livelli del motore | Skill Level, `UCI_Elo`, oppure livelli con nomi                                 | Fase 3               |
| Libreria grafici statistiche | Da valutare (deve funzionare offline e rispettare i temi)                       | Fase 9               |
| Linguaggio del tutor         | Prima a regole; poi AI locale (Ollama)                                          | Fasi 7 / 10          |
| CSP                          | Da attivare quando l'interfaccia è stabile                                      | Fase 10              |
| Licenza                      | chessground e Stockfish sono GPL-3.0: per uso personale nessun problema         | Prima di distribuire |

---

## 9. Trappole note e lezioni imparate

- **shakmaty:** `fen.parse()` non basta, serve `fen.parse::<Fen>()`, altrimenti compare
  l'errore E0282 (successo nella Fase 1).
- **Clippy pedantic:** `assert!(v.is_empty())` sui `Vec` nei test ha creato problemi; si usa
  `assert_eq!(v, Vec::new())`.
- **ESLint strict:**
  - niente cast `as HTMLInputElement`: si usa `getByRole<HTMLInputElement>(…)`;
  - niente `invoke<void>`: si scrive `async/await`;
  - `FormEvent` è deprecato: si usa `SyntheticEvent`;
  - `Array.prototype.at` non è disponibile (la libreria TS è ES2021).
- **CSS Modules:** con `noUncheckedIndexedAccess` le classi sono `string | undefined`; si
  passano sempre da `classNames()`.
- **Testing Library:** la pulizia tra un test e l'altro non è automatica; ci pensa
  `src/test/setup.ts`, che simula anche `matchMedia` e `scrollIntoView`.
- **npm audit:** **mai** `npm audit fix --force`. Le vulnerabilità di Vitest 3 si sono risolte
  passando a Vitest 5 con Node ≥ 22.12.
- **`tauri-specta`** è stato scartato perché instabile; non va riproposto.
- **Il pacchetto chessground** si chiama `@lichess-org/chessground`, non `chessground`.
- **Lato grafico**, ad Alessandro piacciono:
  - le cornici attorno alla scacchiera;
  - pezzi puliti e minimal;
  - tanta scelta di stili.
    Vuole che ogni scelta valga in **tutta** l'app. Ha chiesto di schiarire le case scure del
    noce nel tema Sera (i pezzi neri si vedevano poco).
