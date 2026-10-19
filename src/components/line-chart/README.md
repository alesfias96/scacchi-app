# src/components/line-chart — Il grafico a linea

Un grafico che mostra come cambia un valore nel tempo, ad esempio il rating dei puzzle dopo ogni
tentativo o la precisione di ogni partita. A sinistra ci sono il valore più alto e quello più
basso, sotto la data del primo e dell'ultimo risultato. I punti sono messi a distanza uguale: conta
l'ordine, non quanti giorni sono passati tra uno e l'altro. Con meno di due risultati compare un
messaggio al posto della linea. È disegnato con SVG, senza librerie, con i colori del tema.

| File                   | Cosa fa                              |
| ---------------------- | ------------------------------------ |
| `LineChart.tsx`        | Il componente                        |
| `LineChart.module.css` | I suoi stili (colori presi dal tema) |
| `LineChart.test.tsx`   | I suoi test                          |
