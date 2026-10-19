# src/features/tutor — Il tutor

Il tutor spiega la posizione mostrata dalla sezione aperta. Sta in una **barra a comparsa sul
bordo destro**, uguale in ogni sezione: si apre portando il mouse sul bordo, con la linguetta
«Tutor» o con il tasto **T**, e si può fissare aperta.

Dentro la barra:

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
«Allenamento» si accende solo a puzzle finito. Lavora solo quando la barra è aperta.

La sezione **Tutor** del menu principale, con i commenti dopo ogni mossa sopra la scacchiera, sta
in `src/features/play/coach/`; le schede da aprire stanno in `src/components/knowledge-cards/`.

## Contenuto

| Elemento                | Cosa fa                                                                        |
| ----------------------- | ------------------------------------------------------------------------------ |
| `TutorPanel.tsx`        | Il contenuto della barra: scacchiera, interruttore, spiegazione, prove "e se…" |
| `TutorPanel.module.css` | La sua disposizione                                                            |
| `TutorPanel.test.tsx`   | I test: senza posizione, contro il computer, puzzle, schede, mosse provate     |
| `tutor-sections/`       | Le parti della spiegazione, con le frasi che si illuminano                     |
| `what-if-results/`      | Le mosse provate e cosa cambia con ognuna                                      |
