/**
 * ClockFace — il quadrante dell'orologio di un giocatore.
 *
 * Cosa fa: mostra il tempo rimasto (`4:07`, oppure `9.4` negli ultimi
 * secondi) e lo fa scorrere mentre l'orologio è in moto. L'orologio di chi
 * deve muovere è pieno nel colore d'accento; negli ultimi 10 secondi diventa rosso.
 *
 * Riceve:
 * - `reading`: il tempo ricevuto da Rust e se sta scorrendo;
 * - `side`: di chi è l'orologio (per il nome letto dai lettori di schermo).
 * Restituisce: il quadrante.
 */
import type { Side } from '../../../bindings/Side';
import { useCountdown } from '../../../hooks/useCountdown';
import { classNames } from '../../../lib/classNames';
import { formatClock } from '../../../lib/formatClock';
import styles from './ClockFace.module.css';

/** Sotto questo tempo (in millesimi) l'orologio diventa rosso. */
const LOW_TIME_MS = 10_000;

/** Il tempo di un giocatore, come ricevuto da Rust. */
export interface ClockReading {
  /** Il tempo rimasto, in millesimi di secondo. */
  ms: number;
  /** `true` se sta scorrendo. */
  running: boolean;
}

export interface ClockFaceProps {
  /** Il tempo ricevuto. */
  reading: ClockReading;
  /** Di chi è l'orologio. */
  side: Side;
}

export function ClockFace({ reading, side }: ClockFaceProps) {
  const remaining = useCountdown(reading.ms, reading.running);
  return (
    <span
      className={classNames(
        styles.clock,
        reading.running && styles.running,
        remaining < LOW_TIME_MS && styles.low,
      )}
      aria-label={`Tempo del ${side === 'white' ? 'Bianco' : 'Nero'}`}
    >
      {formatClock(remaining)}
    </span>
  );
}
