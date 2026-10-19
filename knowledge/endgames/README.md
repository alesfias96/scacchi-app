# knowledge/endgames — I finali

I tipi di finale, decisi dai pezzi rimasti (i pedoni non contano). Nei finali dove uno ha più pedoni, «chi ha più pedoni» è chi può giocare per vincere.

Il formato delle schede è spiegato nel README della cartella `knowledge/`.

| File                      | Scheda                          | Quando vale (riconosciuto da `crates/tutor/src/endgame_kind.rs`) |
| ------------------------- | ------------------------------- | ---------------------------------------------------------------- |
| `pawn-endgame.toml`       | Il finale di pedoni             | `pawn-endgame`                                                   |
| `rook-endgame.toml`       | Il finale di torre              | `rook-endgame`                                                   |
| `opposite-bishops.toml`   | Gli alfieri di colore contrario | `opposite-bishops`                                               |
| `same-color-bishops.toml` | Gli alfieri dello stesso colore | `same-color-bishops`                                             |
| `knight-endgame.toml`     | Il finale di cavalli            | `knight-endgame`                                                 |
| `bishop-vs-knight.toml`   | Alfiere contro cavallo          | `bishop-vs-knight`                                               |
| `queen-endgame.toml`      | Il finale di donne              | `queen-endgame`                                                  |
| `rook-vs-minor.toml`      | Torre contro pezzo minore       | `rook-vs-minor`                                                  |
| `mate-with-queen.toml`    | Il matto di re e donna          | `mate-with-queen`                                                |
| `mate-with-rook.toml`     | Il matto di re e torre          | `mate-with-rook`                                                 |
