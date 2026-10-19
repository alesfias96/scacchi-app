# knowledge — La base di conoscenza del tutor

Qui c'è il "perché strategico" che il tutor aggiunge ai fatti verificati: principi, strutture
pedonali, aperture, temi di mediogioco e finali. Ogni **scheda** è un piccolo file di testo
(formato TOML), scritto in italiano e leggibile anche da chi non programma. Le schede vengono
incluse nell'app durante la compilazione: funzionano offline.

Il tutor non inventa nulla: mostra una scheda solo quando una regola verificabile dice che vale
per la posizione (ad esempio "c'è un pedone isolato in d4", oppure "l'apertura è la Partita
Italiana").

## Contenuto

| Cartella      | Cosa contiene                                                                                      |
| ------------- | -------------------------------------------------------------------------------------------------- |
| `principles/` | I principi generali (sviluppo, re al sicuro, centro, torri, …)                                     |
| `tactics/`    | I motivi tattici (forchetta, deviazione, mossa intermedia, …) e il metodo «Prima di muovere»       |
| `structures/` | Le strutture pedonali (isolano, Carlsbad, catene, …)                                               |
| `openings/`   | Le aperture più giocate, con le idee per il Bianco e per il Nero                                   |
| `middlegame/` | I temi e i piani del mediogioco (arrocchi opposti, avamposti, attacco di minoranza, profilassi, …) |
| `endgames/`   | I tipi di finale (pedoni, torre, …) e i loro piani (regola del quadrato, fortezza, …)              |

## Com'è fatta una scheda

```toml
title = "Il pedone isolato di donna (isolano)"
when = ["structure:isolated-queen-pawn"]      # quando vale
summary = "Una o due frasi: di cosa si tratta."
ideas = ["Prima idea.", "Seconda idea."]
mistakes = ["Un errore tipico da evitare."]
use_when = ["Quando conviene."]                # facoltativo: «Quando usarlo»
avoid_when = ["Quando no."]                    # facoltativo: «Quando no»
signs = ["Un segnale che sta funzionando."]    # facoltativo: «Segnali che funziona»
puzzle_theme = "fork"                          # facoltativo: il tema dei puzzle di Lichess («Allenati su questo tema»)

[[plans]]                                    # un piano (se ne possono scrivere più di uno)
for = "owner"                                # per chi: white, black, owner, opponent, both
label = "Chi ha l'isolano"                   # come chiamarlo (il tutor aggiunge il colore)
steps = ["Primo passo.", "Secondo passo."]
```

- `when`: `argomento:chiave`. L'argomento deve essere quello della cartella. Per le aperture la
  chiave è il nome di Lichess (in inglese) della famiglia, ad esempio `opening:Ruy Lopez`; per gli
  altri argomenti è una chiave che il tutor sa riconoscere (vedi il README di ogni cartella).
- `owner` / `opponent`: chi "ha" la cosa riconosciuta e il suo avversario. Il tutor scrive il
  colore giusto, ad esempio "Chi ha l'isolano (il Bianco)".
- `use_when`, `avoid_when`, `signs`: le usano soprattutto le schede di strategia (Fase 12-B), che
  il tutor propone come piani; si possono lasciare fuori.
- Le virgolette dentro i testi si scrivono «così».

## Aggiungere o modificare una scheda

1. Si crea o si modifica il file `.toml` nella cartella giusta (il nome del file non ha spazi).
2. `npm run check`: i test controllano che ogni scheda si legga, che ogni "quando vale" sia
   riconosciuto dal tutor (o, per le aperture, che esista nell'elenco di Lichess) e che tutto
   ciò che il tutor riconosce abbia la sua scheda.
3. Si aggiorna il README della cartella.

Per una chiave nuova (una struttura che il tutor non riconosce ancora) bisogna anche scrivere la
regola che la riconosce, in `crates/tutor/src/`.
