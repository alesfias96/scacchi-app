# crates/analysis/src — Il codice dei rilevatori

Ogni file ha un solo compito. I test di ogni file stanno in fondo al file stesso.

## Le basi

| File               | Cosa fa                                                                                   |
| ------------------ | ----------------------------------------------------------------------------------------- |
| `lib.rs`           | La "porta d'ingresso": elenca i moduli e decide cosa è visibile da fuori                  |
| `report.rs`        | `analyze`: chiama tutti i rilevatori e raccoglie i fatti in un unico rapporto, per gruppo |
| `fact.rs`          | Un fatto: gruppo, giocatore a cui si riferisce, frase, case, frecce, mosse citate         |
| `piece_value.rs`   | Il valore dei pezzi (pedone 1, cavallo e alfiere 3, torre 5, donna 9)                     |
| `exchange.rs`      | Lo scambio statico: chi guadagna se si comincia a catturare su una casa                   |
| `wording.rs`       | Le parole italiane: "il cavallo bianco in c3", "della torre nera in a8", elenchi          |
| `test_position.rs` | Aiuti per i test (posizioni da FEN)                                                       |

## I rilevatori

| File                | Cosa trova                                                                           |
| ------------------- | ------------------------------------------------------------------------------------ |
| `mate_threats.rs`   | Matti in una mossa: quelli possibili e quelli che l'avversario minaccia              |
| `hanging.rs`        | I pezzi in presa                                                                     |
| `forks.rs`          | Le forchette sulla scacchiera e le mosse che le creano                               |
| `lines.rs`          | Inchiodature, infilate, attacchi e scacchi di scoperta (le tattiche "di linea")      |
| `overload.rs`       | Il sovraccarico: l'unico difensore di due o più pezzi attaccati                      |
| `pawn_structure.rs` | Pedoni isolati, doppiati, passati, arretrati, catene e isole                         |
| `king_safety.rs`    | Scacco, re al centro, scudo di pedoni, colonne aperte vicino al re, pressione sul re |
| `activity.rs`       | Cavalli sul bordo, alfieri "cattivi", torri attive, mobilità                         |
| `material.rs`       | Il conto del materiale, chi è avanti, la coppia degli alfieri                        |
| `game_phase.rs`     | La fase di gioco, calcolata dal materiale rimasto                                    |
