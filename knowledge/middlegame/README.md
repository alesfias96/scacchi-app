# knowledge/middlegame — I temi del mediogioco

Situazioni tipiche del mediogioco, riconosciute dalla posizione dei pezzi.

Il formato delle schede è spiegato nel README della cartella `knowledge/`.

| File                     | Scheda               | Quando vale (riconosciuto da `crates/tutor/src/middlegame_themes.rs`) |
| ------------------------ | -------------------- | --------------------------------------------------------------------- |
| `opposite-castling.toml` | Gli arrocchi opposti | `opposite-castling`                                                   |
| `outpost.toml`           | L'avamposto          | `outpost`                                                             |
| `seventh-rank.toml`      | La torre in settima  | `seventh-rank`                                                        |

## Le schede di strategia (Fase 12-B)

Queste schede non le riconosce un tema della scacchiera: le propone il tutor come **piani**,
quando le regole di `crates/tutor/src/strategy_plans.rs` dicono che valgono (dagli squilibri della
posizione). Hanno anche «Quando usarlo», «Quando no» e «Segnali che funziona».

| File                          | Scheda                                      | Chiave                   |
| ----------------------------- | ------------------------------------------- | ------------------------ |
| `minority-attack.toml`        | L'attacco di minoranza                      | `minority-attack`        |
| `pawn-majority.toml`          | La maggioranza di pedoni su un'ala          | `pawn-majority`          |
| `central-break.toml`          | La rottura centrale                         | `central-break`          |
| `weak-squares.toml`           | Le case deboli (i buchi)                    | `weak-squares`           |
| `color-complex.toml`          | Il complesso di case di un colore           | `color-complex`          |
| `bad-bishop.toml`             | L'alfiere cattivo                           | `bad-bishop`             |
| `good-bishop-bad-knight.toml` | L'alfiere buono contro il cavallo           | `good-bishop-bad-knight` |
| `space-advantage.toml`        | Il vantaggio di spazio                      | `space-advantage`        |
| `worst-piece.toml`            | Il pezzo peggiore                           | `worst-piece`            |
| `prophylaxis.toml`            | La profilassi                               | `prophylaxis`            |
| `when-to-trade.toml`          | Quando cambiare i pezzi                     | `when-to-trade`          |
| `trades-against-space.toml`   | Cambiare per liberarsi dallo spazio         | `trades-against-space`   |
| `same-side-attack.toml`       | L'attacco al re, arrocchi dallo stesso lato | `same-side-attack`       |
| `greek-gift.toml`             | Il sacrificio greco                         | `greek-gift`             |
| `exchange-sacrifice.toml`     | Il sacrificio di qualità                    | `exchange-sacrifice`     |
| `pawn-sacrifice.toml`         | Il sacrificio posizionale di pedone         | `pawn-sacrifice`         |
| `initiative.toml`             | L'iniziativa e lo sviluppo                  | `initiative`             |
| `closed-center.toml`          | Il centro chiuso                            | `closed-center`          |
| `open-game.toml`              | Il centro aperto                            | `open-game`              |
| `pawn-chain-base.toml`        | La catena di pedoni: attaccare la base      | `pawn-chain-base`        |
| `two-weaknesses.toml`         | Il principio delle due debolezze            | `two-weaknesses`         |
| `transforming-advantage.toml` | Trasformare il vantaggio                    | `transforming-advantage` |
| `half-open-file.toml`         | La colonna semiaperta                       | `half-open-file`         |
