# components/piece-icon — I pezzi fuori dalla scacchiera

Serve a mostrare un pezzo **fuori** dalla scacchiera (ad esempio nella finestra
di scelta della promozione, o nelle anteprime delle Impostazioni), sempre nello
stile scelto dall'utente.

| File                   | Cosa contiene                                                              |
| ---------------------- | -------------------------------------------------------------------------- |
| `pieceCode.ts`         | I "nomi" dei pezzi: colore (`w`/`b`) + pezzo (`K Q R B N P`), es. `wQ`     |
| `pieceImageUrl.ts`     | Trova l'immagine di un pezzo in uno stile (cartella `src/assets/pieces`)   |
| `PieceIcon.tsx`        | Il componente che mostra l'immagine                                        |
| `PieceIcon.module.css` | L'immagine riempie lo spazio disponibile senza deformarsi                  |
| `*.test.ts(x)`         | Verificano che ogni stile abbia tutti i 12 pezzi e che lo stile sia giusto |

Per aggiungere uno stile di pezzi basta creare la sua cartella in
`src/assets/pieces/` con i 12 file (`wK.svg` … `bP.svg`): `pieceImageUrl` la
trova da sola.
