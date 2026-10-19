# src/features/settings/update-section — La scheda «Aggiornamenti»

Mostra la versione dell'app installata e il pulsante **Controlla aggiornamenti**. Se c'è una versione
nuova la propone con **Installa e riavvia**: l'app la scarica, ne controlla la firma, la installa e si
riavvia. L'interruttore **Controlla a ogni avvio** (spento di default) fa il controllo anche da solo
all'apertura dell'app. L'esito del controllo è scritto sotto i pulsanti.

| File                       | Cosa fa                                                                          |
| -------------------------- | -------------------------------------------------------------------------------- |
| `UpdateSection.tsx`        | Il componente                                                                    |
| `UpdateSection.module.css` | I suoi stili                                                                     |
| `UpdateSection.test.tsx`   | I test: versione nuova e installazione, «hai già l'ultima», errore, interruttore |
