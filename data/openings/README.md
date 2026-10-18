# data/openings — L'elenco delle aperture

| File                   | Cosa contiene                                                                                  |
| ---------------------- | ---------------------------------------------------------------------------------------------- |
| `lichess-openings.tsv` | Circa 3.900 aperture e varianti: codice ECO (es. `B90`), nome inglese e mosse in notazione PGN |

**Fonte:** [lichess-org/chess-openings](https://github.com/lichess-org/chess-openings), i file
`a.tsv` … `e.tsv` uniti in uno solo (una riga di intestazione, poi una riga per apertura, colonne
separate da tabulazione: `eco`, `name`, `pgn`).

**Licenza:** CC0 1.0 (pubblico dominio): si può usare e includere liberamente.

## Come si usa

La crate `crates/openings` include questo file nell'app e, per ogni partita, riconosce l'apertura
giocata: segue le mosse e tiene l'ultima posizione che compare nell'elenco (la variante più precisa).
Per aggiornare l'elenco basta sostituire il file con una versione più recente, nello stesso formato.
