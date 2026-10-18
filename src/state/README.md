# src/state — Lo stato condiviso dell'interfaccia

Qui stanno i dati che più parti dell'interfaccia devono leggere insieme mentre l'app è aperta.
Quando uno di questi dati cambia, tutti i componenti che lo usano si aggiornano da soli.
Usa la libreria **Zustand**.

Le regole e la logica di gioco **non** stanno qui (vivono in Rust): qui c'è solo ciò che
serve all'interfaccia per mostrare le cose.

## Contenuto

| File                    | Cosa fa                                                                                                                        |
| ----------------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| `settingsStore.ts`      | Le impostazioni dell'utente (tema, scacchiera, pezzi, notazione): le legge all'avvio, le cambia e le salva                     |
| `settingsStore.test.ts` | I suoi test: caricamento, modifica, annullamento se il salvataggio fallisce                                                    |
| `gameStore.ts`          | La partita in corso, così come l'ha descritta Rust, e le azioni per cambiarla (mossa, annulla, scorri, nuova partita, incolla) |
| `gameStore.test.ts`     | I suoi test: caricamento, mosse accettate e rifiutate, navigazione, importazione non valida                                    |
