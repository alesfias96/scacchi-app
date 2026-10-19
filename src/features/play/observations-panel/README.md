# src/features/play/observations-panel — La scheda "Osservazioni"

Un interruttore accende i **rilevatori** di Rust (`crates/analysis`), che guardano la posizione
mostrata e trovano i fatti verificabili, divisi per gruppo: **Minacce**, **Tattiche**, **Pedoni**,
**Re**, **Pezzi**, **Materiale**. Il pallino bianco o nero dice a quale giocatore si riferisce il fatto.

Passando il mouse su un fatto (o raggiungendolo con il tasto Tab) la scacchiera mostra frecce e
cerchi blu sulle case coinvolte. La scheda si aggiorna da sola a ogni mossa e quando si scorre
la partita. Contro il computer parte spenta: vederla mentre si gioca sarebbe un aiuto.

| File                           | Cosa fa                                                                   |
| ------------------------------ | ------------------------------------------------------------------------- |
| `ObservationsPanel.tsx`        | La scheda                                                                 |
| `ObservationsPanel.module.css` | I suoi stili                                                              |
| `formatFact.ts`                | La frase completa di un fatto, con le mosse citate nella notazione scelta |
| `*.test.ts(x)`                 | I test                                                                    |
