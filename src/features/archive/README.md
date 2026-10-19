# src/features/archive — La sezione "Archivio"

Tutte le partite giocate, salvate da sole mossa dopo mossa (nel database di Rust, `crates/storage`).
Si vedono dalla più recente e si filtrano per avversario (computer o due giocatori), esito (vinte,
perse, patte, in corso) e apertura (nome inglese o codice ECO, ad esempio "Sicilian" o "B90").

Da ogni partita:

- **Rivedi** (partita finita) o **Continua** (non finita): la apre in "Gioca", con il proprio colore
  in basso. Una partita con l'orologio riparte con l'orologio fermo fino alla mossa successiva.
- **Analizza** (o **Vedi l'analisi**, se è già stata analizzata): la apre in «Analisi partite»;
  l'analisi parte da sola se serve.
- **Elimina**: la toglie dall'archivio, dopo una conferma.

## Contenuto

| Elemento                 | Cosa fa                                                                        |
| ------------------------ | ------------------------------------------------------------------------------ |
| `ArchivePage.tsx`        | La pagina: titolo, filtri, elenco, messaggi (archivio vuoto, errori)           |
| `ArchivePage.module.css` | La disposizione della pagina                                                   |
| `ArchivePage.test.tsx`   | I test: elenco, archivio vuoto, filtri, eliminazione, errore                   |
| `useArchiveGames.ts`     | Chiede a Rust l'elenco a ogni cambio dei filtri (conta solo l'ultima risposta) |
| `archive-filters/`       | I filtri                                                                       |
| `game-row/`              | Una partita nell'elenco, con i suoi pulsanti                                   |
