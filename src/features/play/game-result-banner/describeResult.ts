/**
 * Descrive in italiano come è finita una partita.
 *
 * @param result - il risultato calcolato da Rust.
 * @returns il titolo (es. "Scacco matto") e il dettaglio (es. "Vince il Bianco").
 */
import type { GameEndReason } from '../../../bindings/GameEndReason';
import type { GameResult } from '../../../bindings/GameResult';

/** Il nome di ciascun motivo di fine partita. */
const REASON_TITLES: Record<GameEndReason, string> = {
  checkmate: 'Scacco matto',
  stalemate: 'Stallo',
  insufficientMaterial: 'Materiale insufficiente',
  threefoldRepetition: 'Triplice ripetizione',
  fiftyMoveRule: 'Regola delle 50 mosse',
};

/** Il titolo e il dettaglio da mostrare. */
export interface ResultDescription {
  /** Il motivo della fine, es. "Scacco matto". */
  title: string;
  /** Chi ha vinto, oppure "Patta". */
  detail: string;
}

export function describeResult(result: GameResult): ResultDescription {
  const detail =
    result.winner === null ? 'Patta' : `Vince il ${result.winner === 'white' ? 'Bianco' : 'Nero'}`;
  return { title: REASON_TITLES[result.reason], detail };
}
