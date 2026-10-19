# src-tauri/src/reactions — Ciò che succede da solo dopo ogni cambiamento

Dopo una mossa, una mossa ritirata, una nuova partita, uno spostamento nella lista delle mosse…
il ponte deve fare alcune cose **senza bloccare l'interfaccia**. Le decisioni ("tocca al
computer?", "quando scade il tempo?", "quale posizione analizzare?") le prende la crate
`game-session`; qui si eseguono soltanto, ognuna in un compito separato.

| File                 | Cosa fa                                                                                                                                                                                                                       |
| -------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `mod.rs`             | Elenca i file di questa cartella                                                                                                                                                                                              |
| `after_change.rs`    | Il punto unico da chiamare dopo ogni cambiamento: mette in coda il salvataggio (se la partita è cambiata davvero) e avvia le tre reazioni qui sotto. Se il computer sta già pensando alla stessa posizione, non lo interrompe |
| `computer_move.rs`   | Chiede la mossa al motore che gioca, la gioca nella partita e avvisa l'interfaccia (evento `game-updated`). Una mossa arrivata "in ritardo" viene scartata                                                                    |
| `flag_timer.rs`      | Programma il controllo dell'orologio nell'istante esatto in cui scadrebbe il tempo; ogni nuovo controllo annulla il precedente                                                                                                |
| `follow_analysis.rs` | Se l'analisi è accesa, fa analizzare la posizione mostrata (sempre l'ultima, anche quando si scorrono le mosse in fretta)                                                                                                     |
| `save_worker.rs`     | Il lavoratore che salva le partite nel database, una alla volta e nell'ordine giusto (se si accumulano più versioni della stessa partita, salva solo l'ultima)                                                                |
| `resume_game.rs`     | All'avvio riprende l'ultima partita non finita (se l'utente non ha già cominciato a giocarne una)                                                                                                                             |
| `review_worker.rs`   | Analizza una partita in sottofondo: Stockfish valuta ogni posizione, poi la revisione si salva e gli errori gravi diventano puzzle da ripassare                                                                               |
| `review_queue.rs`    | «Analizza le partite mancanti»: analizza in sottofondo una partita alla volta, dalla più vecchia; fermare l'analisi ferma anche la coda                                                                                       |
