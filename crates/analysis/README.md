# crates/analysis — I rilevatori

Guarda una posizione e trova i **fatti verificabili**: le cose che un buon giocatore vede guardando
la scacchiera. È il "livello 1" del tutor (`PROGETTO.md` §4.2): **fatti, non opinioni**. Non dice chi
sta meglio (quello lo fa Stockfish), ma dice cosa c'è: "il cavallo in c7 attacca re e torre",
"il pedone in d4 è isolato".

Non usa il motore: sono regole precise, scritte in Rust, che danno sempre la stessa risposta e
si possono spiegare. Analizzare una posizione richiede pochi millesimi di secondo.

## Cosa trova

| Gruppo    | Fatti                                                                                                                        |
| --------- | ---------------------------------------------------------------------------------------------------------------------------- |
| Minacce   | Matto in una mossa (possibile per chi muove, o minacciato dall'avversario); pezzi in presa                                   |
| Tattiche  | Forchette (già sulla scacchiera e mosse che le creano), inchiodature, infilate, attacchi e scacchi di scoperta, sovraccarico |
| Pedoni    | Isolati, doppiati, passati, arretrati, catene (con la base), isole                                                           |
| Re        | Scacco, re rimasto al centro, scudo di pedoni debole, colonne senza pedoni vicino al re, pezzi che puntano il re             |
| Pezzi     | Cavalli sul bordo, alfieri "cattivi", torri su colonne aperte o semiaperte e in settima, mobilità                            |
| Materiale | Chi è avanti e di quanto, coppia degli alfieri; e la fase di gioco (apertura, mediogioco, finale)                            |

Ogni fatto ha una frase in italiano, le case da evidenziare e le frecce da disegnare sulla scacchiera.

## Come evita i falsi allarmi

- Un pezzo è "in presa" solo se, facendo tutte le catture possibili su quella casa (lo **scambio
  statico**), l'avversario ci guadagna davvero.
- Una forchetta conta solo se il pezzo che la fa non si può catturare guadagnando.
- Un'inchiodatura (o un'infilata) verso un pezzo conta solo se quel pezzo è indifeso o vale più di
  chi inchioda; quelle verso il re contano sempre.
- Nel finale non si segnalano scudo e arrocco (il re lì deve essere attivo); in apertura non si
  segnala l'alfiere "cattivo".

Limite noto: lo scambio statico non considera i pezzi inchiodati. È una semplificazione accettata:
il giudizio preciso resta a Stockfish.

## Contenuto

| Elemento     | Cosa contiene                                                     |
| ------------ | ----------------------------------------------------------------- |
| `Cargo.toml` | Descrizione della crate e librerie usate                          |
| `src/`       | Il codice, un file per rilevatore (vedi il README della cartella) |

## Test

Ogni rilevatore ha i suoi test su posizioni FEN scelte apposta (una forchetta di cavallo, il matto del
barbiere, un pedone arretrato, …), compresi i casi che **non** devono essere segnalati.
