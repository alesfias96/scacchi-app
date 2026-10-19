# src/features/tutor — Il tutor

Il tutor spiega la posizione mostrata dalla sezione aperta. In «Gioca», «Tutor» e «Allenamento»
in cima alla colonna di destra c'è un **selettore** («Partita | Tutor», «Allenamento | Tutor»):
la voce «Tutor», o il tasto **T**, mostra questo pannello al posto delle schede della sezione.

Nel pannello:

- una **piccola scacchiera** con la posizione spiegata: passando il mouse su una frase si vedono
  le case coinvolte;
- l'interruttore **Giudizio di Stockfish**: aggiunge valutazione a parole e mossa migliore
  (circa mezzo secondo di attesa);
- la **spiegazione**, divisa in parti: _La situazione_, _Cosa succede adesso_, _Il piano_,
  _Da ricordare_. Le frasi con una scheda hanno il pulsante «Approfondisci»;
- gli **Approfondimenti**: le schede della base di conoscenza (`knowledge/`) che riguardano la
  posizione, con idee, piani per ciascun giocatore ed errori da evitare;
- **Prova una mossa** ("e se…"): sulla piccola scacchiera si giocano mosse per conto proprio
  (anche per l'avversario); per ognuna il tutor dice cosa cambia. La partita vera non cambia.
  La promozione è sempre a donna.

Quali posizioni spiega: in «Gioca» quella sulla scacchiera (anche scorrendo le mosse); contro il
computer, a partita in corso, è spento (sarebbe un aiuto) ma si può accendere comunque. In
«Allenamento» si accende solo a puzzle finito. Lavora solo quando la voce «Tutor» è scelta.

La sezione **Tutor** del menu principale, con i commenti dopo ogni mossa sotto la scacchiera, sta
in `src/features/play/coach/`; le schede da aprire stanno in `src/components/knowledge-cards/`.

**Guarda avanti** (circa 3 secondi) e **Pensa più a lungo** (15 secondi): Stockfish calcola le sue
tre varianti migliori e il tutor spiega l'idea di ciascuna, guardando lontano (vedi
`crates/tutor`, "Lo sguardo lungo"). Ogni idea si può guardare sulla scacchiera.

## Contenuto

| Elemento                | Cosa fa                                                                                                                  |
| ----------------------- | ------------------------------------------------------------------------------------------------------------------------ |
| `TutorPanel.tsx`        | Il contenuto della voce «Tutor»: scacchiera, interruttore, spiegazione, prove "e se…"                                    |
| `TutorPanel.module.css` | La sua disposizione                                                                                                      |
| `TutorPanel.test.tsx`   | I test: senza posizione, contro il computer, puzzle, schede, mosse provate                                               |
| `tutor-sections/`       | Le parti della spiegazione, con le frasi che si illuminano                                                               |
| `what-if-results/`      | Le mosse provate e cosa cambia con ognuna                                                                                |
| `idea-card/`            | Un'idea: frase principale, momenti chiave con la loro mossa, schede, variante e i comandi per guardarla sulla scacchiera |
| `strategy-section/`     | «La strategia» in cima alla spiegazione: centro, «Chi ha cosa», piani con perché, come, quando sì e quando no            |
| `thinking-section/`     | «Prima di muovere»: il metodo di pensiero con le mosse della posizione                                                   |
| `opponent-paths/`       | Le strade dell'avversario: cosa potrebbe fare se toccasse a lui, al condizionale                                         |
| `position-ideas/`       | Le idee della posizione, dalla preferita da Stockfish, con il confronto                                                  |
| `usePositionIdeas.ts`   | Chiede le idee della posizione a richiesta («Guarda avanti», «Pensa più a lungo»)                                        |
| `ideaMoves.ts`          | La mossa di un momento chiave scritta come nei libri ("12. ♞c6+"), nella notazione scelta                                |
| `ideaMoves.test.ts`     | I suoi test                                                                                                              |
