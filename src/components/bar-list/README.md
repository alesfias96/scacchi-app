# src/components/bar-list — L'elenco di barre

Un elenco di barre orizzontali, una per riga, con il nome a sinistra e un dettaglio in parole a
destra (ad esempio "3 vinte · 1 patta · 2 perse"). Una barra può avere più tratti colorati uno
dopo l'altro: verde per le vittorie, grigio per le patte, rosso per le sconfitte; oppure un solo
tratto, ad esempio la percentuale di puzzle risolti. Le barre di tutte le righe sono allineate.
È disegnato con SVG, senza librerie, con i colori del tema.

| File                 | Cosa fa                              |
| -------------------- | ------------------------------------ |
| `BarList.tsx`        | Il componente                        |
| `BarList.module.css` | I suoi stili (colori presi dal tema) |
| `BarList.test.tsx`   | I suoi test                          |
