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

## Le schede di strategia (Fase 12-B)

Queste schede non le riconosce un tema della scacchiera: le propone il tutor come **piani**,
quando le regole di `crates/tutor/src/strategy_plans.rs` dicono che valgono (dagli squilibri della
posizione). Hanno anche «Quando usarlo», «Quando no» e «Segnali che funziona».

| File                       | Scheda                      | Chiave                |
| -------------------------- | --------------------------- | --------------------- |
| `outside-passed-pawn.toml` | Il pedone passato lontano   | `outside-passed-pawn` |
| `choosing-endgame.toml`    | Scegliere il finale giusto  | `choosing-endgame`    |
| `fortress.toml`            | La fortezza                 | `fortress`            |
| `zugzwang-opposition.toml` | Lo zugzwang e l'opposizione | `zugzwang-opposition` |
| `rule-of-square.toml`      | La regola del quadrato      | `rule-of-square`      |
