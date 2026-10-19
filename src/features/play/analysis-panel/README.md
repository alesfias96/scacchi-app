# src/features/play/analysis-panel — La scheda "Analisi"

Un interruttore accende o spegne Stockfish sulla posizione mostrata. Quando è acceso la scheda
mostra:

- la **valutazione** (`+0.4` = il Bianco ha quasi mezzo pedone in più; `M3` = matto in 3);
- la **profondità** raggiunta (quante mezze mosse avanti ha guardato il motore): cresce mentre pensa;
- la **variante principale**, cioè le mosse migliori secondo il motore, nella notazione scelta
  nelle Impostazioni (es. `12. ♞f3 e5 13. ♝c4`).

Intanto accanto alla scacchiera compaiono la barra di valutazione e, sulla scacchiera, la freccia
della mossa migliore. Contro il computer l'analisi parte spenta: vederla mentre si gioca toglierebbe
l'esercizio di calcolo.

| File                       | Cosa fa      |
| -------------------------- | ------------ |
| `AnalysisPanel.tsx`        | La scheda    |
| `AnalysisPanel.module.css` | I suoi stili |
| `*.test.ts(x)`             | I test       |
