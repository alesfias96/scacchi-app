# components/eval-bar — La barra di valutazione

La colonna accanto alla scacchiera che mostra chi sta meglio secondo Stockfish: più il Bianco
è in vantaggio, più cresce la parte bianca (e viceversa). Il valore (`+0.4` = quasi mezzo
pedone in più per il Bianco, `M3` = matto in 3) è scritto dalla parte di chi è in vantaggio.
La divisione scorre dolcemente a ogni nuovo giudizio del motore.

Il giudizio lo dà Rust; qui c'è solo il modo di disegnarlo.

| File                 | Cosa contiene                                                                                                   |
| -------------------- | --------------------------------------------------------------------------------------------------------------- |
| `EvalBar.tsx`        | Il componente: un disegno SVG con le due parti e il valore                                                      |
| `EvalBar.module.css` | L'aspetto: colori del bianco e del nero, scorrimento dolce                                                      |
| `whiteShare.ts`      | Quanta parte della barra spetta al Bianco (la stessa curva usata da Lichess: si muove molto vicino alla parità) |
| `*.test.ts(x)`       | I test                                                                                                          |
