# crates/maia/src — Il codice di Maia

Ogni file ha un solo compito. I test di ogni file stanno in fondo al file stesso.

## Contenuto

| File               | Cosa fa                                                                                  |
| ------------------ | ---------------------------------------------------------------------------------------- |
| `lib.rs`           | La "porta d'ingresso": elenca i moduli e decide cosa è visibile da fuori                 |
| `proto_reader.rs`  | Legge il formato dei file delle reti ("protobuf"): numeri, testi e blocchi annidati      |
| `net_weights.rs`   | I pesi della rete letti dal file, con la "normalizzazione" già incorporata (come fa Lc0) |
| `network.rs`       | Il calcolo della rete: convoluzioni, blocchi residui, testa delle mosse                  |
| `input_planes.rs`  | La posizione codificata in 112 piani, vista da chi deve muovere                          |
| `move_index.rs`    | Il posto di una mossa legale tra le 1858 mosse della rete                                |
| `policy_tables.rs` | Le due tabelle delle mosse di Lc0, copiate così come sono (dati, non logica)             |
| `move_choice.rs`   | Dalle uscite della rete alle probabilità, e la scelta a sorte tra le mosse più probabili |
| `maia_level.rs`    | I nove livelli, da 1100 a 1900 Elo                                                       |
| `maia_player.rs`   | Tutto insieme: le reti incluse nell'app, le probabilità delle mosse e la mossa scelta    |
| `maia_error.rs`    | Gli errori possibili                                                                     |
