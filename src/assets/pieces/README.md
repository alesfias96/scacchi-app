# src/assets/pieces — I pezzi

Una cartella per ogni stile. Ogni cartella contiene 12 disegni (formato SVG),
uno per pezzo e colore, sempre con gli stessi nomi:

- prima lettera: colore — `w` bianco, `b` nero;
- seconda lettera: pezzo — `K` re, `Q` regina (donna), `R` torre, `B` alfiere, `N` cavallo, `P` pedone.

Esempio: `wK.svg` è il re bianco, `bN.svg` il cavallo nero.

Il collegamento tra stile scelto e disegni è in `src/components/chessboard/themes/pieces-*.css`.

## Contenuto

| Cartella    | Stile                                              | Origine                                                                 |
| ----------- | -------------------------------------------------- | ----------------------------------------------------------------------- |
| `icona/`    | Forme geometriche con contorno netto (predefinito) | Disegnato per questa app                                                |
| `morbido/`  | Le stesse forme, con volume e ombra                | Disegnato per questa app                                                |
| `legno/`    | Bosso chiaro ed ebano                              | Disegnato per questa app                                                |
| `metallo/`  | Argento e ottone                                   | Disegnato per questa app                                                |
| `gettoni/`  | Dischi con l'iniziale italiana (R, D, T, A, C, P)  | Disegnato per questa app                                                |
| `classico/` | Il set tradizionale "cburnett"                     | Colin M. L. Burnett, distribuito con chessground (licenza GPL/BSD/GFDL) |
