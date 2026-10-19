# data/maia — Le reti di Maia

Le nove reti neurali di **Maia**, il computer "umano" (crate `crates/maia`). Ognuna è stata
addestrata su milioni di partite di Lichess giocate da persone di un certo livello, da 1100 a 1900
Elo: data una posizione, dice quale mossa giocherebbe una persona di quel livello.

- **Fonte:** progetto Maia dell'Università di Toronto (CSSLab), versione 1.0:
  <https://github.com/CSSLab/maia-chess/releases/tag/v1.0>.
- **Licenza:** GPL-3.0 (il testo è in `LICENSE`). Come Stockfish e la scacchiera: per uso personale
  nessun problema; per distribuire l'app va rispettata.
- **Formato:** quello delle reti di Lc0 (Leela Chess Zero). I file originali erano compressi
  (`.pb.gz`); qui sono **già decompressi** (`.pb`), così l'app li legge senza bisogno di una
  libreria in più. Sono inclusi nell'app al momento della compilazione (circa 1,7 MB l'uno).

## Contenuto

| File           | Livello | Impronta SHA-256 del file originale `.pb.gz`                       |
| -------------- | ------- | ------------------------------------------------------------------ |
| `maia-1100.pb` | 1       | `e1cf1cd0c96b8a4fa6a275f4b9fd54ed1ffebf9fe44641b9fceded310e9619c4` |
| `maia-1200.pb` | 2       | `ead4ba953f233ae732999ebc1e2b675378148527ebcfad2f0acbc5e4c224d98e` |
| `maia-1300.pb` | 3       | `36195f87bf4761834baa0bf87472b18509a7261a9d7d6f1a8443261369a733f2` |
| `maia-1400.pb` | 4       | `d5353ea6766356dad2d28920c6692f37a5f30963767f1a3105d33b4d0af011e8` |
| `maia-1500.pb` | 5       | `35ab6f20421d59e1df3b17c5a5016947af4c6761368ef84044a9a9c7619a9a00` |
| `maia-1600.pb` | 6       | `d2c9e5948581acf4b9fc0b1e720c5dc0fe64ce80cfc4a239d3f8a42e1176c876` |
| `maia-1700.pb` | 7       | `d277eacd792d340a30abb464dc65127254e65cac57abca17facc469889b96478` |
| `maia-1800.pb` | 8       | `0031ad7c4256b1fd09fbebd28418d644d68b26cd2a45df4967ccf5c7ec9c4965` |
| `maia-1900.pb` | 9       | `e2f565f42d7cd9f122557e6dc4eb84e5bbaedceda1d404dc485d3611c7c97a12` |
| `LICENSE`      | —       | La licenza GPL-3.0 del progetto Maia                               |

Le impronte servono a verificare che i file vengano davvero dalla pagina ufficiale: si scarica il
`.pb.gz`, se ne calcola l'impronta e la si confronta con quella scritta qui.
