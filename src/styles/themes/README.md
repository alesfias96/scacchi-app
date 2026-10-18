# src/styles/themes — I temi dell'app

Ogni file è un tema: definisce **le stesse variabili** (colori, font, ombra della scacchiera)
con valori diversi. Il tema attivo è scelto dall'attributo `data-theme` sull'elemento `<html>`,
impostato automaticamente in base alle impostazioni (vedi `src/app/theme-controller/`).

Per creare un nuovo tema: copia un file esistente, cambia il nome nel selettore
(`:root[data-theme='nuovo']`) e i valori. Va poi aggiunto all'elenco dei temi in Rust
(`crates/app-settings/src/app_theme.rs`) e alle opzioni della sezione Impostazioni.

## Contenuto

| File         | Tema                 | Aspetto                                                                      |
| ------------ | -------------------- | ---------------------------------------------------------------------------- |
| `studio.css` | Studio (predefinito) | Barra laterale blu inchiostro, piano chiaro, dettagli arancioni, font Outfit |
| `sera.css`   | Sera                 | Tutto scuro, dettagli ambra, font Manrope                                    |
| `giorno.css` | Giorno               | Chiaro e arioso, dettagli verdi, font Plus Jakarta Sans                      |

Il tema **Automatico** non ha un file suo: usa Giorno o Sera a seconda che Windows
(o macOS, o Linux) sia in modalità chiara o scura.
