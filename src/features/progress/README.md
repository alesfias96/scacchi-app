# src/features/progress — La sezione «Progressi»

Le statistiche su come si gioca e come cambia nel tempo, dalle partite giocate e dai puzzle
dell'Allenamento. Tutti i calcoli li fa Rust (`crates/stats`): questa cartella li mostra soltanto.

In cima ci sono i **filtri**: il periodo (ultimi 30 giorni, ultimi 90 giorni, sempre) e le partite
(tutte, contro il computer, a due giocatori). Valgono per tutte le schede e restano salvati nelle
impostazioni. Sotto, una riga dice **su quante partite** si basano le statistiche e quante sono
analizzate; se ci sono partite finite non ancora analizzate, il pulsante **«Analizza le partite
mancanti»** le analizza in sottofondo, una alla volta (anche cambiando sezione), con l'avanzamento
e il pulsante «Ferma». Finita ogni partita, le statistiche si aggiornano da sole.

Le quattro schede:

- **Livello**: l'Elo stimato (dai risultati contro i livelli del computer, con almeno 5 partite),
  il rating dei puzzle e la precisione media, con i grafici nel tempo.
- **Partite**: vinte, patte e perse contro il computer (in tutto, per colore, per livello), la
  precisione e gli errori in apertura, mediogioco e finale, gli errori con poco tempo.
- **Aperture**: le più giocate con il Bianco e con il Nero, con risultati e precisione.
- **Calcolo e tattica**: i puzzle risolti per numero di mosse da trovare, quanto lontano si calcola
  nel tempo, i temi da allenare, i puzzle da ripassare oggi.

## Contenuto

| Elemento                  | Cosa fa                                                                     |
| ------------------------- | --------------------------------------------------------------------------- |
| `ProgressPage.tsx`        | La pagina: titolo, filtri, conteggi, selettore delle schede                 |
| `ProgressPage.module.css` | La disposizione della pagina e delle schede (riquadri, numeri, note)        |
| `ProgressPage.test.tsx`   | I test: livello, cambio di scheda, filtro salvato, coda di partite          |
| `useStats.ts`             | Chiede a Rust le statistiche a ogni cambio dei filtri (conta l'ultima)      |
| `describeStats.ts`        | Le parole e le barre comuni: risultati, percentuali, colori, fasi           |
| `describeStats.test.ts`   | I suoi test                                                                 |
| `progress-filters/`       | I filtri: periodo e partite                                                 |
| `analysis-queue/`         | Su quante partite si basano le statistiche e «Analizza le partite mancanti» |
| `stat-figure/`            | Un numero in evidenza con il suo nome e una nota (es. «Elo stimato»)        |
| `level-tab/`              | La scheda «Livello»                                                         |
| `games-tab/`              | La scheda «Partite»                                                         |
| `openings-tab/`           | La scheda «Aperture»                                                        |
| `tactics-tab/`            | La scheda «Calcolo e tattica»                                               |
