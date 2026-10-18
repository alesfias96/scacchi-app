# crates/app-settings/src — Il codice delle impostazioni

Ogni file ha un solo compito. I test di ogni file stanno in fondo al file stesso.

## Contenuto

| File             | Cosa fa                                                                  |
| ---------------- | ------------------------------------------------------------------------ |
| `lib.rs`         | La "porta d'ingresso": elenca i moduli e decide cosa è visibile da fuori |
| `app_theme.rs`   | I temi dell'app (Studio, Sera, Giorno, Automatico)                       |
| `board_style.rs` | Gli stili della scacchiera (Rovere, Noce, Acero, Marmo, Ardesia, Torneo) |
| `piece_style.rs` | Gli stili dei pezzi (Icona, Morbido, Legno, Metallo, Gettoni, Classico)  |
| `notation.rs`    | La notazione delle mosse (Figurine, Italiana, Internazionale)            |
| `settings.rs`    | L'insieme delle quattro scelte e i valori predefiniti                    |
| `store.rs`       | Lettura e scrittura del file `settings.json`                             |
| `store_error.rs` | Gli errori possibili nella lettura o scrittura, con messaggi chiari      |
