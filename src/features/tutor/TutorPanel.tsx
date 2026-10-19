/**
 * TutorPanel — il tutor: spiega la posizione mostrata dalla sezione aperta.
 *
 * Cosa fa: è la voce «Tutor» in cima alla colonna di destra di «Gioca»,
 * «Tutor» e «Allenamento» (il tasto T ci passa). Mostra:
 * - una piccola scacchiera con la posizione spiegata: passando il mouse su
 *   una frase, si vedono le case coinvolte;
 * - la spiegazione divisa in parti (situazione, cosa succede adesso, piano,
 *   da ricordare) e le schede della base di conoscenza, da aprire;
 * - «Prova una mossa»: sulla piccola scacchiera si giocano mosse per conto
 *   proprio (anche per l'avversario) e per ognuna il tutor dice cosa cambia.
 *   La partita vera non cambia; «Torna alla posizione» chiude la prova.
 * Durante una partita contro il computer il tutor è spento (sarebbe un
 * aiuto), ma si può accendere comunque; durante un puzzle si accende solo a
 * puzzle finito. L'interruttore «Giudizio di Stockfish» aggiunge valutazione
 * e mossa migliore (circa mezzo secondo di attesa).
 *
 * Riceve: niente (legge tutto dallo stato del tutor).
 * Restituisce: il contenuto della voce «Tutor».
 */
import { useCallback, useMemo, useState } from 'react';
import type { LegalMove } from '../../bindings/LegalMove';
import type { TutorItem } from '../../bindings/TutorItem';
import { Button } from '../../components/button/Button';
import { Chessboard } from '../../components/chessboard/Chessboard';
import { Switch } from '../../components/switch/Switch';
import { useLoadedSettings } from '../../state/settingsStore';
import { isTutorLocked, useTutorStore } from '../../state/tutorStore';
import { KnowledgeCards } from '../../components/knowledge-cards/KnowledgeCards';
import { TutorSections } from './tutor-sections/TutorSections';
import styles from './TutorPanel.module.css';
import { WhatIfResults } from './what-if-results/WhatIfResults';

/** Nessuna mossa possibile (quando non si sta provando). */
const NO_MOVES: readonly LegalMove[] = [];

export function TutorPanel() {
  const { notation } = useLoadedSettings();
  const target = useTutorStore((state) => state.target);
  const withEngine = useTutorStore((state) => state.withEngine);
  const unlockedKey = useTutorStore((state) => state.unlockedKey);
  const explanation = useTutorStore((state) => state.explanation);
  const isLoading = useTutorStore((state) => state.isLoading);
  const error = useTutorStore((state) => state.error);
  const whatIf = useTutorStore((state) => state.whatIf);
  const setWithEngine = useTutorStore((state) => state.setWithEngine);
  const unlock = useTutorStore((state) => state.unlock);
  const startWhatIf = useTutorStore((state) => state.startWhatIf);
  const playWhatIf = useTutorStore((state) => state.playWhatIf);
  const undoWhatIf = useTutorStore((state) => state.undoWhatIf);
  const stopWhatIf = useTutorStore((state) => state.stopWhatIf);

  /** La frase indicata con il mouse, da evidenziare sulla scacchiera. */
  const [highlighted, setHighlighted] = useState<TutorItem | null>(null);
  /** La scheda aperta. */
  const [openCardId, setOpenCardId] = useState<string | null>(null);
  /** Cambiandolo, la scacchiera torna alla posizione ricevuta (mossa rifiutata). */
  const [resyncKey, setResyncKey] = useState(0);

  const marks = useMemo(
    () =>
      highlighted === null
        ? null
        : { arrows: highlighted.fact.arrows, circles: highlighted.fact.squares },
    [highlighted],
  );

  /** Apre una scheda e la porta in vista. */
  const openCard = useCallback((cardId: string) => {
    setOpenCardId(cardId);
    document.getElementById(`tutor-card-${cardId}`)?.scrollIntoView({ block: 'nearest' });
  }, []);

  /** La posizione sulla piccola scacchiera: l'ultima mossa provata, o quella spiegata. */
  const lastScenario = whatIf === null ? null : (whatIf[whatIf.length - 1] ?? null);
  const shown = lastScenario?.position ?? explanation?.position ?? null;

  /** Prova la mossa fatta sulla scacchiera (la promozione è sempre a donna). */
  const handleMove = useCallback(
    (from: string, to: string) => {
      const candidates = (shown?.legalMoves ?? []).filter(
        (move) => move.from === from && move.to === to,
      );
      const chosen = candidates.find((move) => move.uci.endsWith('q')) ?? candidates[0];
      if (chosen !== undefined) {
        void playWhatIf(chosen.uci);
      }
      setResyncKey((key) => key + 1);
    },
    [shown, playWhatIf],
  );

  if (target === null) {
    return (
      <p className={styles.message}>
        Comincia una partita o un puzzle: il tutor spiegherà la posizione sulla scacchiera.
      </p>
    );
  }

  if (isTutorLocked(target, unlockedKey)) {
    return target.lock === 'puzzle' ? (
      <p className={styles.message}>
        Il tutor si accende a puzzle finito: adesso rivelerebbe la soluzione.
      </p>
    ) : (
      <div className={styles.locked}>
        <p className={styles.message}>
          Durante la partita contro il computer il tutor è spento: le sue spiegazioni sarebbero un
          aiuto. Si accende da solo a partita finita.
        </p>
        <Button onClick={unlock}>Accendi comunque (è un aiuto)</Button>
      </div>
    );
  }

  const isTrying = whatIf !== null;

  return (
    <div className={styles.panel}>
      <Switch label="Giudizio di Stockfish" checked={withEngine} onChange={setWithEngine} />

      {shown !== null && (
        <div className={styles.board}>
          <Chessboard
            boardFen={shown.boardFen}
            turn={shown.turn}
            inCheck={shown.inCheck}
            orientation={target.orientation}
            lastMove={lastScenario?.lastMove ?? null}
            legalMoves={isTrying ? shown.legalMoves : NO_MOVES}
            onMove={isTrying && !isLoading ? handleMove : undefined}
            marks={marks}
            resyncKey={resyncKey}
          />
        </div>
      )}

      {explanation !== null && (
        <div className={styles.actions}>
          {isTrying ? (
            <>
              <Button onClick={undoWhatIf} disabled={whatIf.length === 0}>
                Annulla la mossa provata
              </Button>
              <Button variant="primary" onClick={stopWhatIf}>
                Torna alla posizione
              </Button>
            </>
          ) : (
            <Button variant="primary" onClick={startWhatIf}>
              Prova una mossa
            </Button>
          )}
        </div>
      )}

      {isLoading && <p className={styles.message}>Il tutor sta guardando la posizione…</p>}
      {error !== null && (
        <p className={styles.error} role="alert">
          {error}
        </p>
      )}

      {isTrying && (
        <section className={styles.block} aria-label="E se…">
          <h3 className={styles.blockTitle}>E se…</h3>
          {whatIf.length === 0 ? (
            <p className={styles.message}>
              Muovi un pezzo sulla scacchiera qui sopra (anche per l’avversario): il tutor ti dirà
              cosa cambia. La partita vera resta com’è.
            </p>
          ) : (
            <WhatIfResults scenarios={whatIf} notation={notation} onHighlight={setHighlighted} />
          )}
        </section>
      )}

      {explanation !== null && !isTrying && (
        <>
          <TutorSections
            sections={explanation.sections}
            notation={notation}
            onHighlight={setHighlighted}
            onOpenCard={openCard}
          />
          {explanation.cards.length > 0 && (
            <section className={styles.block} aria-label="Approfondimenti">
              <h3 className={styles.blockTitle}>Approfondimenti</h3>
              <KnowledgeCards
                cards={explanation.cards}
                openCardId={openCardId}
                onToggle={setOpenCardId}
              />
            </section>
          )}
        </>
      )}
    </div>
  );
}
