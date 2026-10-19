# knowledge/tactics — I motivi tattici e il metodo di pensiero

Le schede della tattica (Fase 12-C). Il tutor le cita quando trova il motivo lungo una variante di
Stockfish (vedi `crates/tutor/src/motifs.rs`) o tra i fatti dei rilevatori. Il campo facoltativo
`puzzle_theme` è il tema dei puzzle di Lichess con quel motivo: accanto alla scheda compare
«Allenati su questo tema», che apre l'Allenamento con quei puzzle.

Il formato delle schede è spiegato nel README della cartella `knowledge/`.

| File                     | Scheda                              | Chiave              | Tema dei puzzle     |
| ------------------------ | ----------------------------------- | ------------------- | ------------------- |
| `fork.toml`              | La forchetta                        | `fork`              | `fork`              |
| `pin.toml`               | L'inchiodatura                      | `pin`               | `pin`               |
| `skewer.toml`            | L'infilata                          | `skewer`            | `skewer`            |
| `discovered-attack.toml` | L'attacco (e lo scacco) di scoperta | `discovered-attack` | `discoveredAttack`  |
| `double-check.toml`      | Lo scacco doppio                    | `double-check`      | `doubleCheck`       |
| `overload.toml`          | Il sovraccarico                     | `overload`          | —                   |
| `attraction.toml`        | L'attrazione (adescamento)          | `attraction`        | `attraction`        |
| `deflection.toml`        | La deviazione                       | `deflection`        | `deflection`        |
| `removing-defender.toml` | L'eliminazione del difensore        | `removing-defender` | `capturingDefender` |
| `intermezzo.toml`        | La mossa intermedia                 | `intermezzo`        | `intermezzo`        |
| `back-rank.toml`         | Il matto sull'ultima traversa       | `back-rank`         | `backRankMate`      |
| `smothered-mate.toml`    | Il matto affogato                   | `smothered-mate`    | `smotheredMate`     |
| `clearance.toml`         | La liberazione di casa o di linea   | `clearance`         | `clearance`         |
| `promotion.toml`         | La promozione                       | `promotion`         | `promotion`         |
| `thinking-method.toml`   | Il metodo «Prima di muovere»        | `thinking-method`   | —                   |
