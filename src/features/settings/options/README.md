# src/features/settings/options — Le scelte disponibili

Gli elenchi delle scelte mostrate nelle Impostazioni, con nome, descrizione e anteprima.
I valori possibili sono decisi in Rust (`crates/app-settings/`); qui ci sono solo i testi
e le immagini. TypeScript controlla che ogni valore scritto qui esista davvero in Rust.
Quando in Rust si aggiunge uno stile, va aggiunta qui la sua voce.

## Contenuto

| File                  | Cosa contiene                                                           |
| --------------------- | ----------------------------------------------------------------------- |
| `themeOptions.tsx`    | I temi dell'app: Studio, Sera, Giorno, Automatico                       |
| `boardOptions.tsx`    | Gli stili della scacchiera: Rovere, Noce, Acero, Marmo, Ardesia, Torneo |
| `pieceOptions.tsx`    | Gli stili dei pezzi: Icona, Morbido, Legno, Metallo, Gettoni, Classico  |
| `notationOptions.tsx` | Le notazioni delle mosse: Figurine, Italiana, Internazionale            |
