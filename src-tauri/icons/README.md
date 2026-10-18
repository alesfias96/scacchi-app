# src-tauri/icons — Le icone dell'app

Ogni sistema operativo vuole l'icona in un formato diverso. Sono tutte la stessa immagine
(un pedone dorato su fondo scuro) in dimensioni e formati diversi.

## Contenuto

| File             | Usato da                                                          |
| ---------------- | ----------------------------------------------------------------- |
| `icon.png`       | Immagine originale a 1024×1024, da cui si ricavano tutte le altre |
| `32x32.png`      | Icone piccole (Linux, barra delle applicazioni)                   |
| `128x128.png`    | Icona media (Linux)                                               |
| `128x128@2x.png` | Icona media per schermi ad alta densità                           |
| `icon.ico`       | Windows                                                           |
| `icon.icns`      | macOS                                                             |

## Come cambiare l'icona

Sostituisci `icon.png` con una nuova immagine quadrata da 1024×1024 e lancia
`npm run tauri icon src-tauri/icons/icon.png`: Tauri rigenera tutti gli altri formati.
