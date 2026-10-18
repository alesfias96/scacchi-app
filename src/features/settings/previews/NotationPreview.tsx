/**
 * NotationPreview — due mosse d'esempio scritte in una notazione.
 *
 * Cosa fa: mostra come appariranno le mosse nell'elenco della partita
 * (un cavallo che va in f3 e una donna che cattura in d8 dando scacco).
 *
 * Riceve:
 * - `notation`: la notazione da mostrare.
 * Restituisce: l'anteprima.
 */
import type { Notation } from '../../../bindings/Notation';
import { formatSan } from '../../../lib/formatSan';
import styles from './NotationPreview.module.css';

/** Le mosse d'esempio, in notazione internazionale. */
const SAMPLE_MOVES = ['Nf3', 'Qxd8+'] as const;

export interface NotationPreviewProps {
  /** La notazione da mostrare. */
  notation: Notation;
}

export function NotationPreview({ notation }: NotationPreviewProps) {
  return (
    <span className={styles.sample}>
      {SAMPLE_MOVES.map((san) => (
        <span key={san} className={styles.move}>
          {formatSan(san, notation)}
        </span>
      ))}
    </span>
  );
}
