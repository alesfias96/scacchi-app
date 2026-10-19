# crates/stats — Le statistiche dei progressi

Riceve i dati grezzi letti dal database (le partite, le mosse delle partite analizzate, i
tentativi dei puzzle) e calcola tutte le statistiche della sezione «Progressi». Non legge il
database (lo fa `storage`, e il ponte `src-tauri` unisce i due) e non sa nulla dell'interfaccia:
sono solo calcoli, facili da verificare con i test.

## Cosa calcola

- **Livello**
  - L'**Elo stimato** è un "Elo di rendimento": la media dell'Elo indicativo dei livelli del
    computer affrontati (Stockfish e Maia insieme), più 400 × (vittorie − sconfitte) ÷ partite. Compare solo con almeno
    5 partite finite contro il computer.
  - Il rating dei puzzle dopo ogni tentativo.
  - La precisione di ogni partita analizzata, e la media.
- **Partite**
  - Vinte, patte e perse contro il computer: in tutto, con ciascun colore, per livello (i livelli di
    Stockfish e di Maia in ordine di Elo).
  - La precisione e gli errori in apertura, nel mediogioco e nel finale.
  - Gli errori con poco tempo, cioè quando sull'orologio resta meno di un quinto del tempo
    iniziale, rispetto al resto della partita.
- **Aperture**: le famiglie più giocate (la parte del nome di Lichess prima dei due punti, es.
  "Sicilian Defense"), fino a 8 per colore, con risultati e precisione.
- **Calcolo e tattica**
  - I puzzle risolti per numero di mosse da trovare.
  - La profondità media dei puzzle risolti, giorno per giorno.
  - I temi in cui si sbaglia di più (con almeno 3 tentativi).
  - I puzzle da ripassare oggi.

Il giudizio delle mosse (imprecisione, errore, errore grave) e la precisione sono calcolati
esattamente come nell'analisi delle partite (crate `review`), così i numeri coincidono.
I ripassi non contano nella percentuale di puzzle risolti: sono puzzle già visti.

## Contenuto

| Elemento     | Cosa contiene                                                         |
| ------------ | --------------------------------------------------------------------- |
| `Cargo.toml` | Descrizione della crate e librerie usate                              |
| `src/`       | Il codice, un file per responsabilità (vedi il README della cartella) |

## L'opzione `bindings`

Con l'opzione `bindings` (attivata da `src-tauri`) i test generano i tipi TypeScript in
`src/bindings/`: `StatsReport`, `LevelStats`, `GameStats`, `OpeningStats`, `TacticsStats` e i
tipi più piccoli che contengono (`ChartPoint`, `Score`, …).

## Test

`npm run test:rust` esegue anche i test di questa crate, con partite e puzzle di prova. Verificano
tra l'altro l'Elo stimato (e che manchi con meno di 5 partite), i risultati per colore, gli errori
con poco tempo, le aperture raggruppate per famiglia e i temi più sbagliati.
