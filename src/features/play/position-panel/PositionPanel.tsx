/**
 * PositionPanel — la scheda "Situazione" accanto alla scacchiera nella sezione Gioca.
 *
 * Cosa fa: descrive la posizione in parole semplici (chi deve muovere,
 * quante mosse sono possibili, se il re è sotto scacco).
 *
 * Riceve:
 * - `position`: la posizione calcolata da Rust.
 * Restituisce: il pannello.
 */
import type { PositionView } from '../../../bindings/PositionView';
import type { Side } from '../../../bindings/Side';
import { PanelCard } from '../../../components/panel-card/PanelCard';
import { classNames } from '../../../lib/classNames';
import styles from './PositionPanel.module.css';

export interface PositionPanelProps {
  /** La posizione da descrivere. */
  position: PositionView;
}

/** Il nome italiano di ciascun colore. */
const SIDE_LABELS: Record<Side, string> = {
  white: 'Bianco',
  black: 'Nero',
};

export function PositionPanel({ position }: PositionPanelProps) {
  return (
    <PanelCard title="Situazione">
      <p className={styles.turn}>
        <span
          className={classNames(
            styles.swatch,
            position.turn === 'white' ? styles.swatchWhite : styles.swatchBlack,
          )}
          aria-hidden="true"
        />
        Tocca al {SIDE_LABELS[position.turn]}
      </p>

      <div className={styles.stats}>
        <p className={styles.stat}>
          <span className={styles.figure}>{position.legalMoves.length}</span>
          <span className={styles.caption}>mosse possibili</span>
        </p>
        <p className={styles.stat}>
          <span className={styles.figure}>{position.inCheck ? 'Sì' : 'No'}</span>
          <span className={styles.caption}>re sotto scacco</span>
        </p>
      </div>
    </PanelCard>
  );
}
