# crates/maia — Il computer "umano"

**Maia** è una rete neurale addestrata su milioni di partite di persone vere: data una posizione,
dice quanto è probabile ogni mossa per una persona di un certo livello, da 1100 a 1900 Elo. Usata
come avversario, gioca come una persona di quel livello, con gli errori tipici di quel livello.
Stockfish ai livelli bassi, invece, si indebolisce "a caso": gioca benissimo e poi, ogni tanto,
regala un pezzo in un modo in cui nessuna persona lo farebbe.

Le nove reti sono quelle originali del progetto Maia (in `data/maia/`, licenza GPL-3.0) e vengono
incluse nell'app. Il loro calcolo lo fa questa crate, in Rust semplice: niente programmi esterni,
uguale su Windows, macOS e Linux.

## Come sceglie una mossa

1. **Codifica la posizione** come la vuole la rete: 112 "piani" di 8×8 numeri. Ci sono dove sono i
   pezzi nelle ultime 8 posizioni, gli arrocchi possibili, il turno e il conteggio delle 50 mosse.
   Tutto è visto da chi deve muovere: se muove il Nero, la scacchiera è ribaltata.
2. **Calcola la rete**: una rete "convoluzionale" (come quelle che riconoscono le immagini) con 6
   blocchi da 64 canali. Dà un valore a ciascuna delle 1858 mosse possibili negli scacchi. Basta un
   solo calcolo, senza ricerca: circa 60 millesimi di secondo.
3. **Trasforma i valori in probabilità** (con la "softmax"), contando solo le mosse legali.
4. **Sceglie a sorte** tra le mosse con almeno un decimo della probabilità della migliore,
   ciascuna con la sua probabilità. Così non gioca sempre la stessa partita, ma non sceglie mai una
   mossa che quasi nessuno giocherebbe.

Il calcolo segue esattamente quello di Lc0 (Leela Chess Zero), il programma per cui le reti sono
state create. Le due tabelle delle mosse di Lc0 sono copiate così come sono in `policy_tables.rs`.

## I livelli

| Livello | Rete        | Gioca come una persona da |
| ------- | ----------- | ------------------------- |
| 1       | `maia-1100` | 1100 Elo                  |
| 2       | `maia-1200` | 1200 Elo                  |
| …       | …           | …                         |
| 9       | `maia-1900` | 1900 Elo                  |

## Contenuto

| Elemento     | Cosa contiene                                                         |
| ------------ | --------------------------------------------------------------------- |
| `Cargo.toml` | Descrizione della crate e librerie usate                              |
| `src/`       | Il codice, un file per responsabilità (vedi il README della cartella) |

## Test

`npm run test:rust` esegue anche i test di questa crate. Verificano tra l'altro che:

- in partenza le mosse più probabili siano 1.e4 e 1.d4, e che contro 1.e4 il livello 1100 risponda 1…e5;
- l'arrocco e la promozione a donna siano riconosciuti;
- le probabilità di tutte le mosse legali sommino a 1;
- la codifica della posizione (scacchiera ribaltata per il Nero, posizioni ripetute) sia giusta;
- le richieste sbagliate (livello inesistente, mossa illegale, nessuna mossa possibile) diano un errore.
