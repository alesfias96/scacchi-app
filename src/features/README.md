# src/features — Le sezioni dell'app

Ogni cartella è una sezione dell'app (quelle elencate nella barra laterale), autonoma:
contiene le sue pagine, i suoi componenti specifici e la sua logica di interfaccia.
I componenti usati da più sezioni stanno invece in `src/components/`.

## Contenuto

| Cartella    | Sezione                             | Disponibile                                                                                                                                              |
| ----------- | ----------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `play/`     | Gioca                               | Dalla Fase 1 (posizione), dalla Fase 2 (mosse), dalla Fase 3 (contro il computer)                                                                        |
| `archive/`  | Archivio                            | Dalla Fase 4: tutte le partite salvate, con filtri; rivedi, continua, elimina                                                                            |
| `training/` | Allenamento                         | Dalla Fase 6: puzzle di calcolo, visualizzazione, "cosa minaccia?", ripasso                                                                              |
| `tutor/`    | Barra del tutor (a destra, tasto T) | Dalla Fase 7: spiega la posizione, schede della base di conoscenza, "e se…". La sezione «Tutor» del menu (commenti dopo ogni mossa) sta in `play/coach/` |
| `settings/` | Impostazioni                        | Tema dell'app, scacchiera, pezzi, notazione, suoni                                                                                                       |

## Sezioni che verranno aggiunte

| Cartella    | Sezione                 | Fase |
| ----------- | ----------------------- | ---- |
| `review/`   | Analisi partite         | 8    |
| `progress/` | Progressi e statistiche | 9    |
