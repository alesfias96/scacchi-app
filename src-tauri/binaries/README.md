# src-tauri/binaries — I programmi esterni (motori)

Qui stanno i programmi che l'app avvia accanto a sé: per ora **Stockfish**, il motore di scacchi.
Tauri li chiama _sidecar_ ("sidecar" = il sidecar della motocicletta: viaggia attaccato all'app).

Questi file **non sono nella cronologia git** (sono pesanti, circa 100 MB) e **non si copiano a mano**:
li scarica `npm run engines` (vedi `scripts/README.md`), che parte da solo con `npm run check` e
`npm run tauri dev`.

## Contenuto (dopo `npm run engines`)

| File                                      | Cosa contiene                                                              |
| ----------------------------------------- | -------------------------------------------------------------------------- |
| `stockfish-<sistema>` (`.exe` su Windows) | Il programma di Stockfish, es. `stockfish-x86_64-pc-windows-msvc.exe`      |
| `stockfish-<sistema>.version`             | Quale versione e quale archivio l'hanno prodotto (per non riscaricarlo)    |
| `STOCKFISH-COPYING.txt`                   | La licenza di Stockfish (GPL-3.0), che va distribuita insieme al programma |
| `README.md`                               | Questo file (l'unico salvato in git)                                       |

## Perché il nome contiene il sistema

Tauri sceglie da solo il file giusto per il computer su cui si compila, leggendolo dal nome:
`x86_64-pc-windows-msvc` (Windows), `aarch64-apple-darwin` (Mac M1/M2…), `x86_64-apple-darwin`
(Mac Intel), `x86_64-unknown-linux-gnu` (Linux). Nell'app installata il file si chiama solo
`stockfish` e sta accanto all'eseguibile dell'app.
