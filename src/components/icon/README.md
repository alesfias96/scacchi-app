# components/icon — Le icone dell'app

Piccole icone a tratto (frecce, copia, incolla, altoparlante, bandierina, dischetto, cartella, …) disegnate direttamente nel
codice: niente file esterni, funzionano offline e prendono automaticamente il
colore del testo, quindi si adattano a ogni tema.

| File              | Cosa contiene                                          |
| ----------------- | ------------------------------------------------------ |
| `iconPaths.ts`    | I disegni di tutte le icone (per aggiungerne una: qui) |
| `Icon.tsx`        | Il componente che disegna un'icona dato il suo nome    |
| `Icon.module.css` | Dimensione (normale o grande) e stile del tratto       |
| `Icon.test.tsx`   | I test                                                 |
