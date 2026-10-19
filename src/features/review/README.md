# src/features/review — La sezione «Analisi partite»

Qui si rivedono le proprie partite per imparare dagli errori. Si apre dall'**Archivio** (pulsante
«Analizza» o «Vedi l'analisi») oppure da **«Gioca»** a fine partita («Analizza la partita»).

- **Partita non ancora analizzata**: il pulsante «Analizza la partita» fa partire l'analisi in
  sottofondo. Stockfish guarda ogni posizione per circa un secondo, la barra mostra l'avanzamento
  e «Ferma l'analisi» la interrompe. L'analisi continua anche cambiando sezione.
- **Partita analizzata**:
  - a sinistra la scacchiera; sotto, il **grafico della partita** (chiaro = Bianco, scuro =
    Nero, pallini colorati per gli errori; un clic porta a quella mossa) e i comandi per
    scorrere, anche con ←/→, Home e Fine;
  - a destra, alta quanto la scacchiera e con lo scorrimento interno:
    - il **Riepilogo**: precisione ed errori di ciascun giocatore, e dove ha sbagliato di più;
    - i **Momenti critici**: la mossa giocata, il giudizio, la mossa migliore e il perché (cosa
      concedeva la mossa giocata, cosa otteneva quella migliore). «Mostra sulla scacchiera» porta
      alla posizione prima dell'errore: la freccia verde è la mossa migliore, quella blu la
      mossa giocata;
    - le **Mosse**, con i simboli ?! (imprecisione), ? (errore), ?? (errore grave).

Gli errori gravi diventano anche **puzzle** nel ripasso dell'Allenamento (tema «Dalle tue partite»).

## Contenuto

| Elemento                | Cosa fa                                                            |
| ----------------------- | ------------------------------------------------------------------ |
| `ReviewPage.tsx`        | La pagina: partita da analizzare, avanzamento, oppure la revisione |
| `ReviewPage.module.css` | La disposizione, come in «Gioca»                                   |
| `ReviewPage.test.tsx`   | I test: senza partita, da analizzare, revisione, momento critico   |
| `useGameReview.ts`      | Legge la revisione da Rust e la rilegge a fine analisi             |
| `describeQuality.ts`    | I simboli delle mosse (?!, ?, ??) e i nomi delle fasi di gioco     |
| `review-summary/`       | La scheda «Riepilogo»                                              |
| `critical-moments/`     | La scheda «Momenti critici»                                        |
| `review-moves/`         | La scheda «Mosse», con i simboli                                   |
