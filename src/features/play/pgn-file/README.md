# src/features/play/pgn-file — Salvare e aprire file .pgn

Due pulsanti che aprono la finestra del sistema (la stessa di "Salva con nome" e "Apri" di ogni
programma):

- **Salva file** salva la partita in un file `.pgn`, il formato letto da tutti i programmi e siti di scacchi;
- **Apri file** carica una partita da un file `.pgn` al posto di quella in corso (si gioca poi a due
  giocatori; anche questa partita finisce nell'archivio, alla prima mossa).

| File                        | Cosa fa       |
| --------------------------- | ------------- |
| `PgnFileButtons.tsx`        | Il componente |
| `PgnFileButtons.module.css` | I suoi stili  |
| `PgnFileButtons.test.tsx`   | I suoi test   |
