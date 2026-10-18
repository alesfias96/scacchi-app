# src/test — Impostazioni comuni dei test

I test veri stanno accanto al file che verificano (es. `App.test.tsx` vicino a `App.tsx`).
Qui c'è solo ciò che vale per tutti i test.

## Contenuto

| File                   | Cosa fa                                                                                                                                                        |
| ---------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `setup.ts`             | Eseguito prima dei test: pulisce la pagina simulata dopo ogni test, simula la modalità chiara/scura del sistema e lo scorrimento automatico (`scrollIntoView`) |
| `renderWithRouter.tsx` | Disegna un componente dentro un router di prova, per i componenti che contengono collegamenti tra sezioni                                                      |
