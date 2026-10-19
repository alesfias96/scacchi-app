# crates/training — L'allenamento con i puzzle

Un **puzzle** è una posizione presa da una partita vera in cui esiste una continuazione forte
(un matto, una forchetta, un pezzo vinto…). Il puzzle comincia dopo la mossa dell'avversario: chi
risolve deve trovare la mossa giusta, poi l'avversario risponde, e così via fino alla fine della
soluzione.

I puzzle (circa 100.000) vengono dal database di Lichess, con licenza CC0 (vedi
`data/puzzles/README.md`), e sono **inclusi nell'app**: funzionano offline e si scelgono
all'istante, perché restano in memoria (circa 15 MB) invece di stare nel database.

## Cosa sa fare

- **Scegliere un puzzle** adatto: numero di mosse da trovare (da 1 a 6), tema (forchetta,
  inchiodatura, matto in 2…), difficoltà rispetto al proprio livello (più facile, normale, più
  difficile). I puzzle già affrontati non vengono riproposti come nuovi.
- **Verificare le mosse**: è giusta la mossa della soluzione, oppure qualsiasi mossa che dà scacco
  matto. Una mossa sbagliata si può riprovare, ma il puzzle conta come sbagliato (come su Lichess);
  lo stesso vale per "Aiuto" e "Mostra soluzione".
- **Le modalità**: calcolo (tutta la soluzione), visualizzazione (la stessa cosa, ma i pezzi
  spariscono: l'interfaccia li nasconde), "cosa minaccia?" (si guarda dalla parte di chi difende
  e si indovina la mossa forte dell'avversario), ripasso.
- **Il rating dei puzzle**: un numero che misura il proprio livello, con la formula Elo (quella dei
  tornei). Parte da 1500; il ripasso non lo cambia.
- **Il ripasso** dei puzzle sbagliati (anche quelli nati dagli errori gravi delle proprie
  partite, tema «Dalle tue partite», che l'analisi della Fase 8 mette subito in ripasso), con il metodo delle "scatole" (Leitner): un puzzle
  sbagliato torna dopo 1 giorno, poi dopo 3, 7, 14 e 30 giorni se lo si risolve ogni volta; un
  nuovo errore lo riporta all'inizio.
- **Spiegare la soluzione**: a puzzle finito, i rilevatori (crate `analysis`) dicono cosa c'era
  nella posizione (forchette, pezzi in presa, inchiodature…).

Salvare tentativi, rating e ripasso è compito della crate `storage`; questa crate non sa nulla
del database né dell'interfaccia.

## Contenuto

| Elemento     | Cosa contiene                                                         |
| ------------ | --------------------------------------------------------------------- |
| `Cargo.toml` | Descrizione della crate e librerie usate                              |
| `src/`       | Il codice, un file per responsabilità (vedi il README della cartella) |

## L'opzione `bindings`

Con l'opzione `bindings` (attivata da `src-tauri`) i test generano i tipi TypeScript in
`src/bindings/`: `AttemptView`, `Feedback`, `Reveal`, `ThemeInfo`, `TrainingOverview`.

## Test

`npm run test:rust` esegue anche i test di questa crate. Verificano tra l'altro che: il file dei
puzzle si legga tutto; la scelta rispetti profondità, tema e livello e non ripeta i puzzle già
fatti; una mossa giusta faccia rispondere l'avversario e una sbagliata si possa riprovare; aiuto e
soluzione contino come errore; il rating salga e scenda come previsto; le scadenze del ripasso
siano quelle giuste.
