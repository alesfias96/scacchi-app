# src-tauri/capabilities — I permessi

Per sicurezza, l'interfaccia di un'app Tauri non può fare nulla sul sistema se non le viene
concesso esplicitamente. Questa cartella elenca i permessi concessi.

Regola: si concede solo ciò che serve davvero, e ogni nuovo permesso va annotato in `PROGETTO.md`.

## Contenuto

| File           | Cosa fa                                                                                   |
| -------------- | ----------------------------------------------------------------------------------------- |
| `default.json` | Permessi della finestra principale. Per ora solo quelli di base di Tauri (`core:default`) |
