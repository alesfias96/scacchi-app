# src/features/play/opening-label — Il nome dell'apertura

Sopra l'elenco delle mosse compare l'apertura giocata, riconosciuta da Rust (`crates/openings`): il
codice ECO in evidenza e il nome, in inglese come nell'elenco di Lichess (es. "B90 · Sicilian
Defense: Najdorf Variation"). Se l'apertura non è riconosciuta non compare nulla.

| File                      | Cosa fa       |
| ------------------------- | ------------- |
| `OpeningLabel.tsx`        | Il componente |
| `OpeningLabel.module.css` | I suoi stili  |
| `OpeningLabel.test.tsx`   | I suoi test   |
