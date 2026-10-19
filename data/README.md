# data — Dati inclusi nell'app

Dati presi da fonti esterne, con una licenza che ne permette l'uso. Vengono **inclusi nell'app**
durante la compilazione, così l'app funziona offline.

## Contenuto

| Cartella    | Cosa contiene                                                                                          |
| ----------- | ------------------------------------------------------------------------------------------------------ |
| `openings/` | L'elenco delle aperture di Lichess: codice ECO, nome e mosse (usato dalla crate `crates/openings`)     |
| `puzzles/`  | Circa 100.000 puzzle scelti dal database di Lichess, licenza CC0 (usati dalla crate `crates/training`) |
