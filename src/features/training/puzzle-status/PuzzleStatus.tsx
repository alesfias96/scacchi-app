/**
 * PuzzleStatus — la scheda "Puzzle": a che punto si è e cosa si può fare.
 *
 * Cosa fa: mostra il messaggio (consegna, mossa giusta o sbagliata, risolto),
 * quante mosse sono state trovate, le mosse giocate dall'inizio del puzzle
 * (nella visualizzazione sono l'unico modo di seguire la partita) e i
 * pulsanti: "Aiuto" (prima il tema a parole, poi con "Altro aiuto" il pezzo
 * da muovere), "Mostra soluzione" e,
 * a puzzle finito, "Prossimo puzzle".
 *
 * Riceve:
 * - `attempt`: il puzzle in corso;
 * - `memorizing`: `true` nella visualizzazione finché i pezzi si vedono;
 * - `notation`: la notazione scelta nelle Impostazioni;
 * - `onHint`, `onShowSolution`, `onNext`: le azioni dei pulsanti.
 * Restituisce: la scheda.
 */
import type { AttemptView } from '../../../bindings/AttemptView';
import type { Notation } from '../../../bindings/Notation';
import { Button } from '../../../components/button/Button';
import { PanelCard } from '../../../components/panel-card/PanelCard';
import { classNames } from '../../../lib/classNames';
import { formatLine } from '../../../lib/formatLine';
import { describeFeedback } from './describeFeedback';
import styles from './PuzzleStatus.module.css';

export interface PuzzleStatusProps {
  attempt: AttemptView;
  memorizing: boolean;
  notation: Notation;
  onHint: () => void;
  onShowSolution: () => void;
  onNext: () => void;
}

/** Quante mezze mosse mostrare al massimo nell'elenco delle mosse giocate. */
const MAX_SHOWN_MOVES = 24;

export function PuzzleStatus({
  attempt,
  memorizing,
  notation,
  onHint,
  onShowSolution,
  onNext,
}: PuzzleStatusProps) {
  const message = describeFeedback(attempt, memorizing);
  // La prima mossa è quella dell'avversario: da lì comincia il puzzle.
  const played = attempt.game.moves;
  const [first] = played;
  const moves =
    first === undefined
      ? ''
      : formatLine(
          { moveNumber: first.moveNumber, turn: first.side, san: played.map((move) => move.san) },
          notation,
          MAX_SHOWN_MOVES,
        );

  return (
    <PanelCard title={attempt.isReview ? 'Ripasso' : 'Puzzle'}>
      <p
        className={classNames(
          styles.message,
          message.tone === 'good' && styles.good,
          message.tone === 'bad' && styles.bad,
        )}
        role="status"
      >
        {message.text}
      </p>
      <p className={styles.progress}>
        Mosse trovate: {attempt.found} di {attempt.depth}
      </p>
      <p className={styles.moves} aria-label="Mosse giocate">
        {moves}
      </p>
      {attempt.hintText !== null && !attempt.finished && (
        <p className={styles.hint} role="note">
          Aiuto: {attempt.hintText}
        </p>
      )}
      <div className={styles.actions}>
        {attempt.finished ? (
          <Button variant="primary" onClick={onNext}>
            Prossimo puzzle
          </Button>
        ) : (
          <>
            <Button onClick={onHint} disabled={!attempt.canHint}>
              {attempt.hintText === null ? 'Aiuto' : 'Altro aiuto'}
            </Button>
            <Button onClick={onShowSolution}>Mostra soluzione</Button>
          </>
        )}
      </div>
    </PanelCard>
  );
}
