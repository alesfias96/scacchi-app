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
