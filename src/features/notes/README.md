# src/features/notes — La sezione «Appunti»

Uno spazio per scrivere e ritrovare idee, pensieri e note di studio. A sinistra:

- **Nuovo appunto**;
- la **ricerca**: trova gli appunti che contengono tutte le parole scritte, nel titolo, nel testo o
  nelle etichette, senza badare a maiuscole e accenti;
- le **etichette**, per vedere solo gli appunti con una certa etichetta;
- l'**elenco**, dal più recente (♟ = collegato a una posizione).

A destra l'appunto aperto: titolo, testo ed etichette (separate da virgole) si scrivono
direttamente e **si salvano da soli** poco dopo che si smette di scrivere. Se l'appunto è collegato a
una posizione, c'è la scacchiera piccola con **Apri in Gioca** e, se l'appunto è nato da una partita
dell'archivio, **Apri la partita a questa mossa**. **Elimina** chiede conferma.

Gli appunti collegati si creano con **Aggiungi un appunto**, in «Gioca» (scheda «Appunti» nella
colonna di destra) e in «Analisi partite» (sotto la scacchiera). In «Gioca» la partita si collega
solo se è già nell'archivio e la posizione è sulla linea principale; altrimenti si ricorda solo la
posizione.

## Contenuto

| Elemento               | Cosa fa                                                                                            |
| ---------------------- | -------------------------------------------------------------------------------------------------- |
| `NotesPage.tsx`        | La pagina: titolo, ricerca, etichette, elenco e appunto aperto (quello dell'indirizzo `?appunto=`) |
| `NotesPage.module.css` | La disposizione della pagina                                                                       |
| `NotesPage.test.tsx`   | I test: elenco, ricerca e filtro, appunto collegato, salvataggio automatico, nuovo ed elimina      |
| `useNotes.ts`          | Chiede a Rust gli appunti a ogni cambio della ricerca, con le etichette                            |
| `noteLinks.ts`         | Il collegamento di un appunto nato in «Gioca», la posizione da disegnare, il nome nell'elenco      |
| `noteLinks.test.ts`    | I suoi test                                                                                        |
| `note-list/`           | L'elenco degli appunti                                                                             |
| `note-editor/`         | L'appunto aperto, con il salvataggio automatico                                                    |
| `add-note-button/`     | Il pulsante «Aggiungi un appunto» usato in «Gioca» e in «Analisi partite»                          |
