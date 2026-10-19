# src/components/eval-graph — Il grafico della partita

Una striscia che mostra la probabilità di vittoria mossa per mossa: la parte chiara (sotto) è del
Bianco, quella scura (sopra) del Nero, la linea tratteggiata è la parità. Le mosse sbagliate sono
pallini colorati (giallo imprecisione, arancio errore, rosso errore grave). Un clic, o le frecce
con il grafico selezionato, porta alla mossa in quel punto. È disegnato con SVG, senza librerie:
la libreria dei grafici si sceglierà nella Fase 9.

| File                   | Cosa fa                              |
| ---------------------- | ------------------------------------ |
| `EvalGraph.tsx`        | Il componente                        |
| `EvalGraph.module.css` | I suoi stili (colori presi dal tema) |
| `EvalGraph.test.tsx`   | I suoi test                          |
