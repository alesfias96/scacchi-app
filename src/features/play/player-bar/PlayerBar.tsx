/**
 * PlayerBar — la striscia di un giocatore, sopra o sotto la scacchiera.
 *
 * Cosa fa: mostra il colore e il nome del giocatore ("Tu", "Stockfish"…),
 * un dettaglio (es. il livello del computer), l'avviso "sta pensando" mentre
 * il computer cerca la mossa e, se la partita ha l'orologio, il suo tempo.
 *
 * Riceve:
 * - `side`: il colore del giocatore;
 * - `name` e `detail`: il nome e, facoltativo, un dettaglio;
 * - `clock`: il tempo rimasto (in millesimi) e se sta scorrendo, oppure `null`
 *   per una partita senza orologio;
 * - `thinking`: `true` mentre il computer sta pensando.
 * Restituisce: la striscia.
 */
import type { Side } from '../../../bindings/Side';
import { classNames } from '../../../lib/classNames';
import { ClockFace, type ClockReading } from '../clock-face/ClockFace';
import styles from './PlayerBar.module.css';

export interface PlayerBarProps {
  /** Il colore del giocatore. */
  side: Side;
  /** Il nome mostrato. */
  name: string;
  /** Un dettaglio facoltativo (es. "Circolo · 1400 Elo"). */
  detail?: string;
  /** Il tempo del giocatore, oppure `null` se la partita non ha l'orologio. */
  clock: ClockReading | null;
  /** `true` mentre il computer sta pensando. */
  thinking: boolean;
}

export function PlayerBar({ side, name, detail, clock, thinking }: PlayerBarProps) {
  return (
    <div className={styles.bar}>
      <span
        className={classNames(
          styles.swatch,
          side === 'white' ? styles.swatchWhite : styles.swatchBlack,
        )}
        aria-hidden="true"
      />
      <div className={styles.identity}>
        <span className={styles.name}>{name}</span>
        {detail !== undefined && <span className={styles.detail}>{detail}</span>}
      </div>
      {thinking && (
        <span className={styles.thinking} role="status">
          <span className={styles.dots} aria-hidden="true">
            <span />
            <span />
            <span />
          </span>
          sta pensando
        </span>
      )}
      {clock !== null && (
        <span className={styles.clock}>
          <ClockFace reading={clock} side={side} />
        </span>
      )}
    </div>
  );
}
