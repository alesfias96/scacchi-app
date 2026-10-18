# crates/storage — Il database delle partite

Ogni partita giocata viene salvata, **mossa per mossa**, in un file **SQLite**: un database
completo contenuto in un solo file, che sta nella cartella dati dell'app (la sceglie Tauri per ogni
sistema; su Windows è dentro `AppData\Roaming`). Così nessuna partita va persa, nemmeno se l'app
si chiude all'improvviso.

## Cosa si salva

- **Partite** (tabella `games`): data, avversario e livello, colore, tempo e tempi rimasti,
  posizione di partenza, risultato e motivo, apertura, PGN completo.
- **Mosse** (tabella `moves`): numero, notazione, chi l'ha giocata, se è una presa, **quanto ha
  pensato** chi l'ha giocata e quanto tempo gli restava.
- **Colonne per le statistiche**, vuote per ora: valutazione prima e dopo, mossa migliore, perdita
  in centipedoni, fase di gioco, temi tattici, minacce non viste. Le riempirà l'analisi automatica
  della Fase 8; se una mossa viene salvata di nuovo uguale, questi dati restano.

## Le migrazioni

Lo schema (cioè com'è fatto il database) è descritto dai file della cartella `migrations/`, applicati
in ordine all'apertura. Un file già rilasciato non si modifica mai: per cambiare lo schema se ne
aggiunge uno nuovo (`0002_…`), e i database esistenti vengono aggiornati senza perdere nulla.

## Contenuto

| Elemento      | Cosa contiene                                                         |
| ------------- | --------------------------------------------------------------------- |
| `Cargo.toml`  | Descrizione della crate e librerie usate (`sqlx` per SQLite)          |
| `migrations/` | Lo schema del database, un file per versione                          |
| `src/`        | Il codice, un file per responsabilità (vedi il README della cartella) |

## Test

I test usano un database **in memoria** (sparisce alla fine del test): salvano partite vere, le
rileggono e controllano che tornino identiche, che i filtri funzionino e che l'eliminazione tolga
anche le mosse.
