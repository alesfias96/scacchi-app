# crates/app-settings/src — Il codice delle impostazioni

Ogni file ha un solo compito. I test di ogni file stanno in fondo al file stesso.

## Contenuto

| File               | Cosa fa                                                                                   |
| ------------------ | ----------------------------------------------------------------------------------------- |
| `lib.rs`           | La "porta d'ingresso": elenca i moduli e decide cosa è visibile da fuori                  |
| `app_theme.rs`     | I temi dell'app (Studio, Sera, Giorno, Automatico)                                        |
| `board_style.rs`   | Gli stili della scacchiera (Rovere, Noce, Acero, Marmo, Ardesia, Torneo)                  |
| `piece_style.rs`   | Gli stili dei pezzi (Icona, Morbido, Legno, Metallo, Gettoni, Classico)                   |
| `notation.rs`      | La notazione delle mosse (Figurine, Italiana, Internazionale)                             |
| `sound_setting.rs` | I suoni del gioco (Attivi, Disattivati)                                                   |
| `game_setup.rs`    | L'ultima configurazione scelta per una nuova partita (avversario, colore, livello, tempo) |
| `opponent_kind.rs` | Contro chi si gioca: computer o due persone                                               |
| `color_choice.rs`  | Il colore contro il computer: bianco, nero o a sorte                                      |
| `time_control.rs`  | Il tempo di riflessione: minuti a testa più secondi a mossa                               |
| `settings.rs`      | L'insieme delle scelte e i valori predefiniti                                             |
| `store.rs`         | Lettura e scrittura del file `settings.json`                                              |
| `store_error.rs`   | Gli errori possibili nella lettura o scrittura, con messaggi chiari                       |
