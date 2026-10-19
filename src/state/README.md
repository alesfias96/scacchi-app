# src/state — Lo stato condiviso dell'interfaccia

Qui stanno i dati che più parti dell'interfaccia devono leggere insieme mentre l'app è aperta.
Quando uno di questi dati cambia, tutti i componenti che lo usano si aggiornano da soli.
Usa la libreria **Zustand**.

Le regole e la logica di gioco **non** stanno qui (vivono in Rust): qui c'è solo ciò che
serve all'interfaccia per mostrare le cose.

## Contenuto

| File                    | Cosa fa                                                                                                                                                                                                                                                                        |
| ----------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `settingsStore.ts`      | Le impostazioni dell'utente (tema, scacchiera, pezzi, notazione, suoni, ultima partita scelta): le legge all'avvio, le cambia e le salva                                                                                                                                       |
| `settingsStore.test.ts` | I suoi test: caricamento, modifica, annullamento se il salvataggio fallisce                                                                                                                                                                                                    |
| `gameStore.ts`          | La partita in corso (con avversario e orologio), così come l'ha descritta Rust, e le azioni per cambiarla; segue anche i cambiamenti che arrivano da soli (mossa del computer, tempo scaduto, partita ripresa) e i problemi del database; apre partite dall'archivio e da file |
| `gameStore.test.ts`     | I suoi test: caricamento, eventi, mosse accettate e rifiutate, nuova partita, navigazione, abbandono, importazione non valida                                                                                                                                                  |
| `engineStore.ts`        | Il motore: analisi accesa o spenta, ultimo giudizio ricevuto, eventuale problema da mostrare                                                                                                                                                                                   |
| `observationsStore.ts`  | Se la scheda "Osservazioni" è accesa (contro il computer parte spenta)                                                                                                                                                                                                         |
