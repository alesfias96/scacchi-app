# src/app/sidebar — La barra laterale

Il menu principale: nome dell'app, sezioni e, in fondo, le Impostazioni. Sta nella barra a comparsa
sul bordo sinistro della finestra (vedi `src/app/app-layout/`): si apre col mouse sul bordo o con il
tasto M.
I colori arrivano dal tema scelto. La sezione aperta ha il fondo in rilievo e un pallino
nel colore d'accento del tema.

## Contenuto

| File                 | Cosa fa                                                                                      |
| -------------------- | -------------------------------------------------------------------------------------------- |
| `Sidebar.tsx`        | Il componente che disegna la barra laterale, con i collegamenti alle sezioni                 |
| `Sidebar.module.css` | I suoi stili (colori dal tema attivo)                                                        |
| `Sidebar.test.tsx`   | I suoi test: sezione attiva, Archivio, Tutor, Progressi, Appunti, Impostazioni               |
| `navigation.ts`      | L'elenco delle sezioni: nome e indirizzo. Per aggiungere una sezione si modifica questo file |
