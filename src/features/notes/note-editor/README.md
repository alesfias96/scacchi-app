# src/features/notes/note-editor — L'appunto aperto

Titolo, testo ed etichette da scrivere, lo stato del salvataggio ("Salvato", "Salvataggio…"), la
posizione collegata con i pulsanti per aprirla in «Gioca», ed «Elimina» con la conferma.

Il salvataggio è automatico: poco dopo l'ultimo tasto (`AUTOSAVE_DELAY_MS`) l'appunto si salva;
se si chiude l'appunto o si cambia sezione prima, la modifica si salva subito.

| File                    | Cosa fa                              |
| ----------------------- | ------------------------------------ |
| `NoteEditor.tsx`        | Il componente                        |
| `NoteEditor.module.css` | I suoi stili                         |
| `useNoteAutosave.ts`    | La bozza e il salvataggio automatico |
