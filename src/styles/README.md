# src/styles — Gli stili globali

Qui sta tutto ciò che riguarda l'aspetto **dell'intera app**. Gli stili dei singoli componenti
stanno invece accanto a ogni componente, nel suo file `.module.css`.

Regola fondamentale: **i valori grezzi (colori, misure, font) esistono solo in `tokens.css`
e nei file di `themes/`**. Tutti gli altri file usano le variabili, ad esempio `var(--color-text)`.

## Contenuto

| Elemento     | Cosa fa                                                                                                                                |
| ------------ | -------------------------------------------------------------------------------------------------------------------------------------- |
| `index.css`  | Raccoglie gli altri file nell'ordine giusto; è l'unico importato dall'app (in `src/main.tsx`)                                          |
| `tokens.css` | I token comuni a tutti i temi: dimensioni del testo, spaziature, forme, colori delle cornici delle scacchiere, durate delle animazioni |
| `themes/`    | I temi dell'app (Studio, Sera, Giorno): colori e font di ciascuno                                                                      |
| `reset.css`  | Azzera le differenze tra i motori web di Windows, macOS e Linux                                                                        |
| `base.css`   | Stili minimi della pagina: sfondo, testo, contorno di focus per la tastiera                                                            |

## I font

Ogni tema ha il suo font, incluso nell'app (pacchetti `@fontsource`): funzionano anche senza
internet e appaiono identici su tutti i sistemi.

| Tema   | Font              |
| ------ | ----------------- |
| Studio | Outfit            |
| Sera   | Manrope           |
| Giorno | Plus Jakarta Sans |
