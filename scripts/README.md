# scripts — Comandi di servizio del progetto

Piccoli programmi che preparano il progetto. Sono scritti in **JavaScript** (Node.js) e non in
bash o PowerShell, così funzionano uguali su Windows, macOS e Linux (regola di `PROGETTO.md` §5-bis).
Si avviano con `npm run …`, mai a mano.

## Contenuto

| File               | Comando           | Cosa fa                                                                                                                                                                                                                               |
| ------------------ | ----------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `fetch-engines.js` | `npm run engines` | Scarica **Stockfish 19** ufficiale per il sistema in uso, ne controlla l'impronta SHA-256 (se il file non è quello ufficiale si ferma) e lo salva in `src-tauri/binaries/` con il nome che Tauri si aspetta, insieme alla sua licenza |

## Quando parte da solo

- all'inizio di `npm run check` (così Clippy e i test trovano il motore);
- prima di `npm run tauri dev` e `npm run tauri build` (`pretauri`).

Se il motore giusto è già presente non scarica nulla. Per riscaricarlo: `npm run engines -- --force`.
Per scaricare il motore di un altro sistema: `npm run engines -- --target aarch64-apple-darwin`.

## Aggiornare Stockfish

1. Nella pagina delle release di Stockfish su GitHub si legge il nome della nuova versione (es. `sf_20`)
   e l'impronta SHA-256 di ogni archivio.
2. In `fetch-engines.js` si cambiano `STOCKFISH_RELEASE` e le impronte in `ARCHIVES`.
3. `npm run engines`, poi `npm run check`.
