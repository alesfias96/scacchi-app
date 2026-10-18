# crates/openings — Il riconoscimento delle aperture

Le prime mosse di una partita hanno un nome, raccolto nei libri: per esempio dopo
1.e4 c5 2.Cf3 d6 3.d4 cxd4 4.Cxd4 Cf6 5.Cc3 a6 si gioca la "Difesa Siciliana, variante Najdorf",
codice **ECO** `B90` (ECO = _Encyclopaedia of Chess Openings_, la classificazione usata da tutti).

Questa crate contiene l'elenco delle aperture di **Lichess** (circa 3.900, licenza CC0, nella cartella
`data/openings/` della radice) e, data una partita, dice quale apertura è stata giocata. Segue le
mosse e tiene l'ultima posizione che compare nell'elenco: la variante più precisa. Riconosce anche
le **trasposizioni** (stessa posizione raggiunta con un altro ordine di mosse), perché cerca per
posizione e non per sequenza di mosse.

I nomi sono in **inglese**, come nell'elenco di Lichess (es. "Sicilian Defense: Najdorf Variation").

## Contenuto

| Elemento     | Cosa contiene                                                                                                                                    |
| ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| `Cargo.toml` | Descrizione della crate e librerie usate                                                                                                         |
| `src/`       | `lib.rs` (porta d'ingresso), `opening.rs` (codice ECO e nome), `catalog.rs` (l'elenco, pronto per la ricerca), `identify.rs` (il riconoscimento) |

L'elenco viene incluso nell'app durante la compilazione e preparato la prima volta che serve
(qualche centesimo di secondo).
