# .github/workflows — Le procedure automatiche

Ogni file `.yml` qui dentro descrive una procedura che GitHub esegue da solo.

## Contenuto

| File          | Cosa fa                                                                                                                                                                                                                                                                                                    |
| ------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ci.yml`      | A ogni modifica inviata su GitHub, su tre computer (Windows, macOS e Linux): installa tutto, esegue `npm run check` (tutti i controlli e i test), verifica che i tipi generati da Rust in `src/bindings/` siano aggiornati, compila l'app e salva l'installatore per 7 giorni                              |
| `release.yml` | A ogni etichetta `v…` (es. `v0.10.0`), su Windows, macOS e Linux: compila l'app, crea gli installatori **firmati** per gli aggiornamenti (segreti `TAURI_SIGNING_PRIVATE_KEY` e `…_PASSWORD`) e li mette in una **bozza** di Release con il file `latest.json` letto dall'app. La bozza si pubblica a mano |

## Dove vedere i risultati

Sulla pagina del progetto su GitHub, scheda **Actions**:

- ✅ verde: tutto funziona su quella piattaforma
- ❌ rossa: qualcosa si è rotto; cliccando si vede il passaggio che ha fallito

Gli installatori si scaricano in fondo alla pagina di ogni esecuzione, nella sezione **Artifacts**.
