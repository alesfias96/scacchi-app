/**
 * Scrive una valutazione come la mostrano i siti di scacchi.
 *
 * - Centipedoni: in pedoni con un decimale e il segno (`+0.4`, `−1.2`, `0.0`);
 *   oltre i 10 pedoni senza decimali (`+12`).
 * - Scacco matto: `M3` se lo dà il Bianco, `−M3` se lo dà il Nero.
 *
 * @param evaluation - la valutazione dal punto di vista del Bianco.
 * @returns il testo da mostrare.
 */
import type { Evaluation } from '../bindings/Evaluation';

/** Il segno meno tipografico, più leggibile del trattino. */
const MINUS = '−';
/** Oltre questo vantaggio (in centipedoni) i decimali non servono più. */
const NO_DECIMALS_FROM = 1000;

export function formatEvaluation(evaluation: Evaluation): string {
  const { value } = evaluation;
  if (evaluation.kind === 'mate') {
    return `${value < 0 ? MINUS : ''}M${String(Math.abs(value))}`;
  }
  const pawns = Math.abs(value) / 100;
  const digits = Math.abs(value) >= NO_DECIMALS_FROM ? 0 : 1;
  const sign = value > 0 ? '+' : value < 0 ? MINUS : '';
  return `${sign}${pawns.toFixed(digits)}`;
}
