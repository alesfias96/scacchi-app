# src/lib — Piccole funzioni di supporto

Funzioni semplici usate in più punti dell'interfaccia. Quasi tutte sono "pure"
(stesso input, stesso risultato, nessun effetto collaterale); l'unica eccezione è
`copyToClipboard`, che scrive negli appunti del sistema. Non contengono regole degli scacchi:
quelle stanno tutte in Rust.

## Contenuto

| File                      | Cosa fa                                                                                                |
| ------------------------- | ------------------------------------------------------------------------------------------------------ |
| `classNames.ts`           | Unisce più classi CSS in una sola stringa, ignorando quelle assenti (utile per le classi condizionali) |
| `classNames.test.ts`      | I suoi test                                                                                            |
| `describeError.ts`        | Trasforma un errore di qualsiasi tipo in un messaggio leggibile da mostrare all'utente                 |
| `describeError.test.ts`   | I suoi test                                                                                            |
| `formatSan.ts`            | Scrive una mossa nella notazione scelta: figurine (♞f3), italiana (Cf3) o internazionale (Nf3)         |
| `formatSan.test.ts`       | I suoi test                                                                                            |
| `copyToClipboard.ts`      | Copia un testo negli appunti del sistema                                                               |
| `copyToClipboard.test.ts` | I suoi test                                                                                            |
