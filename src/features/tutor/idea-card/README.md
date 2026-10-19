# src/features/tutor/idea-card — Un'idea del tutor

Mostra l'idea di una variante di Stockfish: la frase principale («L'idea: … — si vede tra 3 mosse»)
con la mossa da cui si vede e la valutazione, i **momenti chiave** (ognuno con la sua mossa, nella
notazione scelta: un clic lo mostra sulla scacchiera), le schede della base di conoscenza citate e
la variante intera. **Guarda sulla scacchiera** fa vedere la variante sulla scacchiera della pagina:
qui compaiono i pulsanti per andare avanti e indietro, la mossa mostrata, il commento del momento
chiave e **Torna alla partita**.

| File                  | Cosa fa                                                               |
| --------------------- | --------------------------------------------------------------------- |
| `IdeaCard.tsx`        | Il componente                                                         |
| `IdeaCard.module.css` | I suoi stili                                                          |
| `IdeaCard.test.tsx`   | I test: idea e momenti, variante sulla scacchiera, clic su un momento |
