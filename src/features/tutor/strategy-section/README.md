# src/features/tutor/strategy-section — La strategia

In cima alla spiegazione del tutor: com'è il centro, il bilancio **«Chi ha cosa»** (le voci del
Bianco e del Nero: materiale, pedoni, spazio, alfieri, re…), uno o due **piani consigliati** a chi
muove e il **piano dell'avversario**. Ogni piano dice _perché_ vale proprio qui e, aperto, _come_
si fa, _quando_ conviene, _quando no_ e i segnali che sta funzionando. Se Stockfish, guardando
avanti, segue davvero il piano, compare «✓ Stockfish conferma». Tutto il contenuto lo prepara Rust
(crate `tutor`, file `strategy.rs`).

| File                         | Cosa fa                                                  |
| ---------------------------- | -------------------------------------------------------- |
| `StrategySection.tsx`        | La sezione intera (centro, chi ha cosa, piani)           |
| `StrategySection.module.css` | I suoi stili                                             |
| `StrategyPlan.tsx`           | Un piano (usato anche dalla scheda «Il piano» del coach) |
| `StrategyPlan.module.css`    | I suoi stili                                             |
| `StrategySection.test.tsx`   | I test                                                   |
