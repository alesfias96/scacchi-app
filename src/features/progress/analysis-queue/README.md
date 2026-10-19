# src/features/progress/analysis-queue — Le partite analizzate e la coda

Dice su quante partite si basano le statistiche e quante sono analizzate. Se mancano partite da analizzare, il pulsante «Analizza le partite mancanti» le mette in coda: si analizzano in sottofondo, una alla volta. Durante la coda si vede a che punto è, con il pulsante «Ferma».

| File                       | Cosa fa                                                            |
| -------------------------- | ------------------------------------------------------------------ |
| `AnalysisQueue.tsx`        | Il componente (usa lo stato condiviso dell'analisi, `reviewStore`) |
| `AnalysisQueue.module.css` | I suoi stili                                                       |
