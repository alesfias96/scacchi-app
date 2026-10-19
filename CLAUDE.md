# CLAUDE.md — Da leggere a ogni nuova sessione

> Questo file viene caricato **automaticamente** da Claude Code all'apertura del progetto.
> Serve a riprendere il lavoro esattamente dove si era rimasti, senza che Alessandro debba
> rispiegare nulla. La memoria tecnica completa (architettura, struttura, registro delle decisioni,
> trappole) è in **`PROGETTO.md`**: va letto per intero prima di qualsiasi modifica.
>
> **Ultimo aggiornamento: 2026-10-19 — Fase 8-bis (Varianti) implementata, in attesa dei test di Alessandro.**

---

## 1. Il progetto in due righe

App desktop **offline** per giocare, imparare e allenarsi a scacchi (Windows, macOS, Linux):
**Tauri 2 + Rust** (tutta la logica) e **React 19 + TypeScript** (solo interfaccia). Motore:
**Stockfish 19** come sidecar. Cartella: `C:\Users\afias\Desktop\Scacchi_app`. Git sul ramo `main`.

## 2. Con chi parli

- **Alessandro**, italiano. **Rispondere sempre in italiano**, in modo semplice: sta imparando
  Rust, quindi le scelte tecniche vanno spiegate con parole semplici (cosa fa, perché così).
- Gusti: cornici attorno alla scacchiera, pezzi puliti e minimal, tanta scelta di stili, ogni scelta
  valida in tutta l'app; a ogni fase chiede ritocchi estetici. Vuole sempre vedere **tutta la
  scacchiera** (niente che la copra) e **nessuno scroll della pagina** (le colonne scorrono dentro).
- Dirà man mano quali frasi delle analisi e del tutor non sono chiare: vanno migliorate.

## 3. Regole vincolanti (dalle sue istruzioni e da `PROGETTO.md` §2)

1. **Prima analisi completa, poi piano con le strade possibili (A, B… con quella consigliata),
   poi si aspetta la sua autorizzazione esplicita.** Nessuna modifica ai file prima del suo
   «va bene / vai / sì». L'approvazione vale solo per ciò che è stato proposto.
2. **Una fase della roadmap per passo di lavoro**; alla fine di ogni fase gli si dà l'**elenco dei
   test da fare a mano** (in italiano, numerati, concreti).
3. **Un file = una responsabilità.** Un **README in ogni cartella**, comprensibile anche a chi
   non programma. **Commenti esaustivi in italiano**; i **nomi nel codice in inglese**.
4. **Il codice morto si cancella**, non si commenta.
5. **Una sola metodologia CSS**: design token + CSS Modules. Valori "grezzi" (colori, misure…)
   **solo** in `src/styles/tokens.css` e `src/styles/themes/*.css`. Niente stili inline, niente
   `!important`; classi condizionali con `classNames()`.
6. **Le regole degli scacchi vivono solo in Rust.** I tipi in `src/bindings/` li genera **ts-rs**
   (con `npm run test:rust`): **non si scrivono a mano**. I nomi dei comandi Rust compaiono solo
   in `src/services/`. Le frasi in italiano dei fatti le compone Rust.
7. **Multipiattaforma sempre**: niente script bash/PowerShell nel progetto (script in Node.js sì).
8. **`npm run check` deve essere tutto verde** prima di chiudere qualsiasi lavoro.
9. **Aggiornare `PROGETTO.md`** (stato §9, struttura §6, registro delle decisioni §10, appunti §11,
   trappole §12) a ogni decisione, e **aggiornare questo file** a fine fase (§5 e §6 qui sotto).
10. Commit con messaggio in italiano e la riga finale
    `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`.

## 4. Come si lavora una fase (procedura collaudata)

1. Analisi (cosa c'è già, cosa manca, rischi) e piano a punti numerati con opzioni **A (consigliata)** / B.
2. Si aspetta la risposta (es. «Va bene tutto», «1A, 2B…»).
3. Implementazione: Rust prima (crate pura + test), poi `storage` (migrazione nuova, mai modificare
   quelle già rilasciate), poi ponte `src-tauri` (comandi senza logica), poi interfaccia (servizio,
   store Zustand, componenti, test Vitest).
4. README di tutte le cartelle toccate, `PROGETTO.md`, `npm run format`, `npm run check`.
5. Commit, poi messaggio finale: cosa c'è, eventuali scostamenti dal piano (spiegati), **test da
   fare a mano**. La fase in `PROGETTO.md` §9 passa a 🔵 _in test_.
6. Quando lui approva («va tutto bene, andiamo avanti»): fase ✅ in `PROGETTO.md`, riga
   «Fase N approvata» nel registro, commit, aggiornare questo file, e si propone la fase successiva.

## 5. Stato: cosa è fatto

| Fase            | Cosa ha portato                                                                                                                                                                                           |
| --------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1, 1-bis        | Fondamenta, controlli di qualità, scacchiera (chessground), 4 temi, 6 scacchiere, 6 set di pezzi, impostazioni salvate                                                                                    |
| 2               | Regole complete, lista mosse navigabile, fine partita, import/export FEN e PGN, suoni                                                                                                                     |
| 3               | Stockfish, analisi in tempo reale con barra e freccia, computer a 10 livelli, orologio                                                                                                                    |
| 4               | Database SQLite con migrazioni, salvataggio a ogni mossa, Archivio, ripresa della partita                                                                                                                 |
| 5               | Rilevatori (minacce, tattiche, pedoni, re, attività), scheda «Osservazioni»                                                                                                                               |
| 6               | Allenamento: ~100.000 puzzle Lichess, calcolo, visualizzazione, «cosa minaccia?», ripasso Leitner, rating Elo                                                                                             |
| 7               | Tutor: crate `tutor` + 57 schede in `knowledge/`; voce «Tutor» in cima alla colonna destra (tasto T) con "e se…"; sezione **Tutor** del menu (commenti dopo ogni mossa in una fascia sotto la scacchiera) |
| 8               | Analisi partite: crate `review`, quarto Stockfish, giudizio mosse (probabilità di vittoria persa), precisione, momenti critici, grafico, puzzle dagli errori gravi                                        |
| 8-bis (in test) | Varianti: albero di mosse, lista mosse come Lichess, PGN con varianti, Copia FEN/PGN funzionanti                                                                                                          |

Disposizione attuale delle sezioni con scacchiera: scacchiera a sinistra, colonna a destra **alta
quanto la scacchiera** con scorrimento interno; menu principale a comparsa sul bordo sinistro (tasto M).

## 6. Dove eravamo rimasti — prossimo passo

**Fase 8-bis (Varianti) implementata e in test** (🔵 in `PROGETTO.md`): commit fatto, ad
Alessandro è stato dato l'elenco dei test da fare a mano. Si aspetta il suo esito:

- se va tutto bene: segnare la 8-bis ✅ (procedura del §4, punto 6), aggiornare questo file e
  proporre il **piano della Fase 9 (Statistiche)**, con analisi e strade possibili;
- se chiede ritocchi: analisi e piano dei ritocchi, poi autorizzazione.

Cosa fa la 8-bis: la partita è un **albero di mosse** (`Game` in `crates/chess-core`: la prima
continuazione è la linea principale); una mossa da una posizione passata crea una variante (a due
giocatori senza orologio e in ogni partita finita); lista mosse come Lichess con «Promuovi a linea
principale» (solo a due giocatori senza orologio, partita in corso) ed «Elimina da qui»; PGN con
varianti; nel database la tabella `moves` è la linea principale e l'albero sta nel PGN della
partita. «Copia FEN»/«Copia PGN» ora passano da Rust (`tauri-plugin-clipboard-manager`).

## 7. Cosa resta da fare

- **9. Statistiche** — sezione «Progressi»: precisione ed errori nel tempo, per fase e per apertura,
  livello stimato, rating dei puzzle, temi tattici più sbagliati; crate `crates/stats`; libreria
  dei grafici da scegliere (offline, rispettosa dei temi). I dati sono già nella tabella `moves`
  (riempita dall'analisi delle partite) e in `puzzle_attempts`.
- **10. Estensioni** — Maia via lc0 (avversario "umano"), AI locale facoltativa (es. Ollama) che
  racconta senza decidere, installatori firmati e aggiornamenti, CSP restrittiva.
- **11. Appunti personali** — richiesta di Alessandro, alla fine: note libere con ricerca,
  collegabili a partite e posizioni, salvate nel database.
- Prima di distribuire: chessground e Stockfish sono GPL-3.0 (per uso personale nessun problema).

## 8. Mappa veloce

- `crates/`: `chess-core` (regole, partita, PGN/FEN), `app-settings`, `engine` (UCI; analisi,
  mossa, valutazione breve), `game-session` (partita in corso, orologio), `openings`, `storage`
  (SQLite, migrazioni in `migrations/`), `analysis` (rilevatori), `training` (puzzle), `tutor`
  (spiegazioni + `knowledge/`), `review` (analisi partite).
- `src-tauri/src/`: `commands/` (un file per argomento), `state/`, `events/`, `reactions/`.
  Quattro Stockfish: gioca, analizza, tutor, analisi partite.
- `src/`: `app/` (router, menu, cornice), `features/` (`play` con `coach/`, `archive`,
  `training`, `tutor`, `review`, `settings`), `components/`, `services/`, `state/`, `hooks/`,
  `lib/`, `styles/`, `bindings/` (generati), `test/` (dati di prova).
- `data/`: aperture e puzzle di Lichess (CC0). `knowledge/`: schede TOML del tutor.

## 9. Comandi utili

- `npm run tauri dev` — avvia l'app (scarica Stockfish se manca).
- `npm run check` — tutti i controlli (Rust: formato, Clippy pedantic, test; poi Prettier, ESLint, tsc, Vitest).
- `npm run format` / `npm run rust:format` — sistemano la formattazione.
- `npm run puzzles` — riscarica e riseleziona i puzzle di Lichess (il file è già nel repository).

## 10. Trappole da ricordare (le più frequenti; l'elenco completo è in `PROGETTO.md` §12)

- Shell: heredoc lunghi con apostrofi si rompono → scrivere i file con lo strumento di scrittura
  o con script `.cjs` nella cartella scratchpad; Python non è installato, si usa Node.
- sqlx 0.9: niente query con `format!`; ts-rs: `u64` diventa `bigint` (usare `u32`).
- Clippy pedantic: `assert_eq!(x, [])` invece di `assert!(x.is_empty())`; funzioni sotto le 100 righe.
- ESLint strict: niente `Array.prototype.at`, catene opzionali dove possibile.
- Colonne alte quanto la scacchiera: `contain: size` sulla colonna, `align-content: start` sulla griglia.
- Le schede di `knowledge/` le include `build.rs` della crate `tutor`; i test controllano che
  ogni scheda sia valida e collegata.
