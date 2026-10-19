# crates/tutor — Le spiegazioni delle posizioni

Il **tutor** spiega una posizione in italiano, a tre livelli (`PROGETTO.md` §4.2):

1. **I fatti verificati**: le minacce e le tattiche dei rilevatori (crate `analysis`) e, se
   richiesta, la valutazione di Stockfish con la mossa migliore.
2. **La conoscenza**: le schede della cartella `knowledge/` (principi, strutture pedonali,
   aperture, mediogioco, finali). Una scheda compare solo se una regola verificabile dice che
   vale: questa crate contiene i **riconoscitori** che guardano la scacchiera (dove sono i
   pedoni, quali pezzi restano, dove sono i re…).
3. **Il linguaggio**: frasi composte da regole. Non decide mai nulla: racconta solo ciò che i
   primi due livelli hanno verificato. (Più avanti, Fase 10, un'AI locale potrà raccontarlo meglio.)

## Cosa sa fare

- **Spiegare una posizione** (`explain`): la situazione (chi muove, fase, apertura, materiale,
  valutazione, tipo di finale), cosa succede adesso (minacce e tattiche), il piano (strutture e
  temi di mediogioco), i principi da ricordare, e le schede che approfondiscono.
- **Gli scenari "e se…"** (`what_if`): prova una mossa e dice cosa si concede all'avversario,
  cosa si crea, quali minacce si risolvono e, con Stockfish, il giudizio sulla mossa
  ("Imprecisione", "Errore"…) con la risposta migliore dell'avversario.

Il motore lo avvia il ponte `src-tauri` (un terzo Stockfish, solo per il tutor): questa crate
riceve già le valutazioni e non sa nulla di Tauri né dell'interfaccia.

## Lo sguardo lungo (Fase 12-A)

Per spiegare il **perché** di una mossa il tutor guarda dove porta: rigioca le varianti di
Stockfish (12–20 mezze mosse) e confronta la posizione di oggi con quella lungo il percorso e alla
fine. Ne ricava l'**idea**: un pezzo che con una manovra arriva su un avamposto o in settima
traversa, una rottura di pedone, una colonna che si apre, materiale vinto (anche con una
combinazione) o sacrificato, debolezze nuove dell'avversario, una struttura o un finale nuovi; nelle
posizioni tranquille, il piano (arroccare, sviluppare, portare le torri al centro). La frase
principale dice l'idea e **quando** si vede; i momenti chiave la mostrano passo per passo.

Le frasi non contengono mosse: l'interfaccia le aggiunge nella notazione scelta. Un pezzo
semplicemente "in presa" a metà di uno scambio o un'inchiodatura su un pedone non contano come tattica.

## La strategia (Fase 12-B)

Ogni spiegazione ha anche **«La strategia»**: il bilancio **«Chi ha cosa»** per ciascun lato
(materiale, coppia degli alfieri, pedoni deboli e passati, maggioranze sulle ali, spazio, pezzo
peggiore, alfiere cattivo, colonne, avamposti, case deboli vicino al re, re e sviluppo), com'è il
centro (chiuso, aperto, mobile) e da lì **uno o due piani** per chi muove, più il piano
dell'avversario. Se l'avversario ha un piano pericoloso, il primo piano diventa la **profilassi**.
Ogni piano dice il **perché** concreto in questa posizione e, dalla sua scheda, **come** si fa,
**quando usarlo**, **quando no** e i **segnali** che funziona. Dopo «Guarda avanti», se la variante
migliore di Stockfish va nella stessa direzione (es. una rottura di pedone per il piano «rottura
centrale»), il piano è **confermato** (`confirmed_plans` delle idee della posizione).

## La tattica e il metodo di pensiero (Fase 12-C)

- Lungo le varianti il tutor riconosce i **motivi tattici** (`motifs.rs`) e racconta una
  combinazione passo per passo: «una combinazione: prima una deviazione…, poi una forchetta…, e alla
  fine il Bianco vince una torre». Ogni motivo ha la sua scheda in `knowledge/tactics/` e, se c'è,
  il tema dei puzzle per allenarsi.
- **Le strade dell'avversario** (`opponent_paths.rs`): non «l'avversario vuole fare questo», ma le
  strade possibili se toccasse a lui, ognuna spiegata al condizionale («si vedrebbe tra 3 mosse»).
- **Prima di muovere** (`thinking.rs`): il metodo di pensiero con le mosse della posizione; dopo un
  errore, `move_lesson.rs` dice quale passo l'avrebbe evitato.
- `solution_idea` spiega la soluzione di un puzzle come una combinazione (la usa la crate `training`).

## Contenuto

| Elemento     | Cosa contiene                                                                           |
| ------------ | --------------------------------------------------------------------------------------- |
| `Cargo.toml` | Descrizione della crate e librerie usate (`toml` per leggere le schede)                 |
| `build.rs`   | Durante la compilazione raccoglie tutte le schede di `knowledge/` e le include nell'app |
| `src/`       | Il codice, un file per responsabilità (vedi il README della cartella)                   |

## L'opzione `bindings`

Con l'opzione `bindings` (attivata da `src-tauri`) i test generano i tipi TypeScript in
`src/bindings/`: `TutorExplanation`, `TutorSection`, `TutorItem`, `CardView`, `PlanView`, `Topic`,
`WhatIfView`.

## Test

`npm run test:rust` esegue anche i test di questa crate. Verificano tra l'altro che: ogni scheda si
legga e abbia testo; ogni "quando vale" sia riconosciuto (e ogni nome di apertura esista
nell'elenco di Lichess); tutto ciò che i riconoscitori trovano abbia la sua scheda; strutture,
finali, temi e principi vengano riconosciuti in posizioni di prova; la valutazione sia detta a
parole in modo giusto; la spiegazione e gli scenari contengano le parti attese.
