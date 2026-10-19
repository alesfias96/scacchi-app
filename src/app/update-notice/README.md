# src/app/update-notice — L'avviso di una versione nuova

Se nelle Impostazioni è acceso **Controlla a ogni avvio**, all'apertura dell'app si controlla una
volta se c'è una versione nuova. Se c'è, compare un piccolo riquadro in basso a destra con
**Installa e riavvia** e **Più tardi** (che lo nasconde fino alla prossima apertura). Se il controllo
non riesce, ad esempio senza Internet, non compare nulla: l'esito si vede nelle Impostazioni.

| File                      | Cosa fa                                                            |
| ------------------------- | ------------------------------------------------------------------ |
| `UpdateNotice.tsx`        | Il componente                                                      |
| `UpdateNotice.module.css` | I suoi stili (riquadro fisso in basso a destra)                    |
| `UpdateNotice.test.tsx`   | I test: nessun collegamento se non richiesto, avviso e «Più tardi» |
