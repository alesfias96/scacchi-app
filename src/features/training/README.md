# src/features/training — La sezione "Allenamento"

Puzzle per allenare il **calcolo**: si cerca la continuazione migliore in una posizione presa da una
partita vera. Come in "Gioca", a sinistra c'è la scacchiera e a destra la colonna con le schede,
sempre aperta e con il suo scorrimento interno:

- **Il tuo livello**: il rating dei puzzle (parte da 1500), quanti puzzle risolti, quanti da ripassare oggi.
- **Allenamento**: le scelte, che si ricordano.
  - _Calcolo_: trova tutte le mosse della soluzione (da 1 a 6).
  - _Visualizzazione_: la posizione si vede per alcuni secondi (da 2 a 15), poi i pezzi
    spariscono e la soluzione si gioca a memoria, con due clic sulle case; le mosse giocate
    restano scritte nella scheda "Puzzle".
  - _Cosa minaccia?_: la scacchiera è girata dalla parte di chi difende; bisogna indovinare la
    mossa forte dell'avversario.
  - _Ripasso_: i puzzle sbagliati tornano dopo 1, 3, 7, 14 e 30 giorni.
  - Il tema (forchetta, inchiodatura, matto in 2…) e la difficoltà (più facile, normale, più difficile).
- **Puzzle**: il messaggio (giusto, sbagliato, risolto), le mosse trovate, i pulsanti _Aiuto_ (cerchia
  il pezzo da muovere), _Mostra soluzione_ e _Prossimo puzzle_.
- **Soluzione**, a puzzle finito: rating e quanto è cambiato, difficoltà, temi, la soluzione scritta e
  **perché funziona** (i fatti dei rilevatori; passandoci sopra la scacchiera mostra le case).

Una mossa sbagliata torna indietro e si può riprovare, ma il puzzle conta come sbagliato (anche con
Aiuto o Mostra soluzione): così il rating misura davvero cosa si trova da soli. Tutte le regole
stanno in Rust (crate `crates/training`).

## Contenuto

| Elemento                   | Cosa fa                                                                                               |
| -------------------------- | ----------------------------------------------------------------------------------------------------- |
| `TrainingPage.tsx`         | La pagina: scacchiera (con promozione e pezzi nascosti) e colonna con le schede                       |
| `TrainingPage.module.css`  | La disposizione, uguale a "Gioca"                                                                     |
| `TrainingPage.test.tsx`    | I test: livello, nuovo puzzle, mossa e soluzione, aiuto, errore                                       |
| `usePiecesHidden.ts`       | Nella visualizzazione, dice quando nascondere i pezzi (dopo i secondi scelti)                         |
| `usePiecesHidden.test.tsx` | I suoi test                                                                                           |
| `useTrainTheme.ts`         | «Allenati su questo tema»: sceglie il tema, comincia un puzzle e apre la sezione                      |
| `train-theme-button/`      | Il pulsante «Allenati: …» usato dal tutor, dall'Analisi partite e dalle schede                        |
| `training-setup/`          | La scheda "Allenamento": modalità, mosse da trovare, tema, difficoltà, secondi                        |
| `puzzle-status/`           | La scheda "Puzzle": messaggio, avanzamento, mosse giocate, aiuto a gradini, pulsanti                  |
| `puzzle-reveal/`           | La scheda "Soluzione": rating, difficoltà, temi, soluzione, perché funziona, la combinazione spiegata |
