# crates/review — L'analisi delle partite giocate

Riceve una partita e il giudizio di Stockfish su ogni sua posizione (il ponte `src-tauri` lo
chiede a un motore dedicato, circa un secondo per posizione) e ne ricava la **revisione**.

## Cosa sa fare

- **Giudicare ogni mossa** con la **probabilità di vittoria persa**, come fa Lichess, invece dei
  soli centipedoni: perdere due pedoni in una posizione già vinta conta poco, perderli in una
  posizione pari conta moltissimo.
  - _Ottima_: è la mossa migliore, o quasi uguale.
  - _Buona_: perde meno di 10 punti.
  - _Imprecisione_ (?!): almeno 10 punti.
  - _Errore_ (?): almeno 20 punti.
  - _Errore grave_ (??): almeno 30 punti.
- **La precisione** di ciascun giocatore (da 0 a 100, con la formula di Lichess), il conteggio
  degli errori e la fase di gioco in cui ha sbagliato di più.
- **I momenti critici**: le cinque mosse che hanno perso di più. Per ognuna dice la mossa migliore
  e il perché, con i fatti dei rilevatori: cosa concedeva la mossa giocata e cosa otteneva quella
  migliore (gli scenari "e se…" della crate `tutor`).
- **Il grafico**: la probabilità di vittoria del Bianco in ogni posizione.
- **I dati da salvare** per le statistiche della Fase 9 (valutazioni, mossa migliore, centipedoni
  persi, fase, temi tattici, minacce concesse).
- **I puzzle dagli errori gravi**: la posizione prima della mossa dell'avversario, da risolvere
  trovando la mossa migliore che nella partita non si è giocata. Contro il computer si usano solo
  i propri errori.

Una posizione finita sulla scacchiera (matto, stallo) non ha valutazione del motore: vale il
risultato (100% o 0% per il matto, 50% per la patta). Le mosse senza valutazione restano senza
giudizio.

## Contenuto

| Elemento     | Cosa contiene                                                         |
| ------------ | --------------------------------------------------------------------- |
| `Cargo.toml` | Descrizione della crate e librerie usate                              |
| `src/`       | Il codice, un file per responsabilità (vedi il README della cartella) |

## L'opzione `bindings`

Con l'opzione `bindings` (attivata da `src-tauri`) i test generano i tipi TypeScript in
`src/bindings/`: `GameReview`, `MoveReview`, `MoveQuality`, `PlayerSummary`, `CriticalMoment`,
`ReviewProgress`, `ReviewFinished`.

## Test

`npm run test:rust` esegue anche i test di questa crate, con una partita di prova (il "matto del
barbiere" sbagliato dal Nero). Verificano tra l'altro il giudizio delle mosse, l'errore grave
trovato e spiegato, il grafico dopo il matto, i dati da salvare e il puzzle creato.
