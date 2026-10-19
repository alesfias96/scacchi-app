/**
 * GameResultBanner — il riquadro che annuncia la fine della partita.
 *
 * Cosa fa: mostra il motivo (scacco matto, stallo, ...), chi ha vinto e il
 * punteggio come nei libri (1-0, 0-1, ½-½). Viene letto automaticamente dai
 * lettori di schermo quando compare.
 *
 * Riceve:
 * - `result`: il risultato calcolato da Rust.
 * Restituisce: il riquadro.
 */
import type { ReactNode } from 'react';
import type { GameResult } from '../../../bindings/GameResult';
import { describeResult } from './describeResult';
import styles from './GameResultBanner.module.css';

export interface GameResultBannerProps {
  /** Il risultato della partita. */
  result: GameResult;
  /** Facoltativo: un comando sotto il risultato (es. «Analizza la partita»). */
  action?: ReactNode;
}

export function GameResultBanner({ result, action }: GameResultBannerProps) {
  const { title, detail } = describeResult(result);
  // "1/2-1/2" è più leggibile con il simbolo del mezzo punto.
  const score = result.score.replaceAll('1/2', '½');

  return (
    <div className={styles.banner} role="status">
      <div className={styles.text}>
        <p className={styles.title}>{title}</p>
        <p className={styles.detail}>{detail}</p>
        {action}
      </div>
      <p className={styles.score}>{score}</p>
    </div>
  );
}
