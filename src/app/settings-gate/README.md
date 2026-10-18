# src/app/settings-gate — Caricamento delle impostazioni all'avvio

All'apertura dell'app legge le impostazioni salvate (tema, scacchiera, pezzi) e mostra
l'interfaccia solo quando sono pronte. Così l'app compare subito con l'aspetto giusto,
senza "lampeggiare" da un tema all'altro. Se la lettura fallisce, spiega il problema.

## Contenuto

| File                      | Cosa fa                                                      |
| ------------------------- | ------------------------------------------------------------ |
| `SettingsGate.tsx`        | Il componente che aspetta le impostazioni e poi mostra l'app |
| `SettingsGate.module.css` | Lo stile del messaggio d'errore                              |
| `SettingsGate.test.tsx`   | I suoi test: impostazioni lette, oppure errore spiegato      |
