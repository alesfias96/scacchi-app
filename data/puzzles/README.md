# data/puzzles — I puzzle di Lichess

`lichess-puzzles.csv` contiene circa **100.000 puzzle** scelti dal database pubblico dei puzzle di
Lichess (<https://database.lichess.org/#puzzles>), distribuito con licenza
**CC0** (pubblico dominio: si può usare liberamente, anche senza citare la fonte; la citiamo
comunque per correttezza).

Il file viene **incluso nell'app** durante la compilazione (crate `crates/training`).

## Come è stato scelto

Il database completo ha più di 5 milioni di puzzle. `npm run puzzles` (script
`scripts/fetch-puzzles.js`) lo scarica e tiene solo quelli più affidabili:

- giocati almeno 300 volte, con un giudizio dei giocatori molto positivo (popolarità ≥ 85);
- con una difficoltà ben misurata (deviazione del rating ≤ 90), da 400 a 2999 punti;
- con una soluzione da 1 a 6 mosse.

Poi li distribuisce in modo equilibrato tra difficoltà e lunghezze, così ci sono puzzle per ogni
livello e ogni profondità.

## Le colonne

| Colonna    | Cosa contiene                                                                         |
| ---------- | ------------------------------------------------------------------------------------- |
| `id`       | Il codice del puzzle su Lichess (si apre con `https://lichess.org/training/<codice>`) |
| `fen`      | La posizione **prima** della mossa dell'avversario                                    |
| `moves`    | La mossa dell'avversario e poi la soluzione, in notazione UCI                         |
| `rating`   | La difficoltà, in punti                                                               |
| `themes`   | I temi, con i nomi di Lichess (es. `fork`, `mateIn2`)                                 |
| `openings` | L'apertura della partita da cui viene il puzzle (non usata per ora)                   |

## Aggiornare i puzzle

`npm run puzzles` riscarica il database completo (circa 300 MB compressi, letti senza salvarli) e
rifà la scelta. Il file risultante si salva nel repository: chi compila l'app non deve scaricare nulla.
