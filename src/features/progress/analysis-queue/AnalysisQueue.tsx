/**
 * AnalysisQueue — su quante partite si basano le statistiche e «Analizza le partite mancanti».
 *
 * Cosa fa: dice quante partite ci sono nel periodo e quante sono analizzate.
 * Se alcune partite finite non sono ancora analizzate, offre il pulsante che
 * le mette in coda: si analizzano in sottofondo, una alla volta (anche
 * cambiando sezione). Durante la coda mostra a che punto è e il pulsante
 * «Ferma».
 *
 * Riceve:
 * - `games`: le partite nel periodo;
 * - `analyzed`: quante di queste sono analizzate;
 * - `missing`: quante partite finite non sono ancora analizzate.
 * Restituisce: la riga con i conteggi e i pulsanti.
 */
import { Button } from '../../../components/button/Button';
import { useReviewStore } from '../../../state/reviewStore';
import { countWords } from '../describeStats';
import styles from './AnalysisQueue.module.css';

export interface AnalysisQueueProps {
  games: number;
  analyzed: number;
  missing: number;
}

export function AnalysisQueue({ games, analyzed, missing }: AnalysisQueueProps) {
  const queue = useReviewStore((state) => state.queue);
  const error = useReviewStore((state) => state.error);
  const analyzeMissing = useReviewStore((state) => state.analyzeMissing);
  const cancel = useReviewStore((state) => state.cancel);
  const isRunning = queue !== null && !queue.finished;

  return (
    <div className={styles.queue}>
      <p className={styles.count} aria-live="polite">
        Basate su {countWords(games, 'partita', 'partite')} ({String(analyzed)}{' '}
        {analyzed === 1 ? 'analizzata' : 'analizzate'}).
        {isRunning &&
          ` Analisi in corso: partita ${String(Math.min(queue.done + 1, queue.total))} di ${String(queue.total)}.`}
      </p>
      {isRunning ? (
        <Button
          onClick={() => {
            void cancel();
          }}
        >
          Ferma
        </Button>
      ) : (
        missing > 0 && (
          <Button
            variant="primary"
            onClick={() => {
              void analyzeMissing();
            }}
          >
            Analizza le partite mancanti ({String(missing)})
          </Button>
        )
      )}
      {error !== null && (
        <p className={styles.error} role="alert">
          {error}
        </p>
      )}
    </div>
  );
}
