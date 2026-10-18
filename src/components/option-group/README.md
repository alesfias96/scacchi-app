# src/components/option-group — Gruppo di scelte a riquadri

Un gruppo di opzioni tra cui se ne sceglie una sola, mostrate come riquadri con anteprima,
nome e descrizione. È usato nelle Impostazioni (tema, scacchiera, pezzi), ma è generico:
può servire ovunque si debba scegliere tra poche opzioni visive.

Sotto l'aspetto grafico c'è un normale gruppo di pulsanti di scelta, quindi funziona anche
con la tastiera (frecce) e con i lettori di schermo.

## Contenuto

| File                     | Cosa fa                                                                     |
| ------------------------ | --------------------------------------------------------------------------- |
| `OptionGroup.tsx`        | Il componente                                                               |
| `OptionGroup.module.css` | I suoi stili: riquadri, bordo della scelta attiva, contorno per la tastiera |
| `OptionGroup.test.tsx`   | I suoi test                                                                 |
