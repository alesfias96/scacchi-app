/**
 * Prepara le mosse legali nel formato che chessground usa per sapere dove
 * può andare ogni pezzo.
 *
 * Rust invia un elenco di mosse (`e2` → `e4`, `e2` → `e3`, ...); chessground
 * vuole invece, per ogni casa di partenza, l'elenco delle case di arrivo
 * (`e2` → [`e3`, `e4`]). Le promozioni compaiono quattro volte in Rust (una
 * per pezzo) ma qui una volta sola: la scelta del pezzo avviene dopo.
 *
 * @param legalMoves - le mosse legali calcolate da Rust.
 * @returns per ogni casa di partenza, le case di arrivo possibili.
 */
import type { Dests, Key } from '@lichess-org/chessground/types';
import type { LegalMove } from '../../bindings/LegalMove';
import { isSquareKey } from './isSquareKey';

export function buildDests(legalMoves: readonly LegalMove[]): Dests {
  const dests = new Map<Key, Key[]>();
  for (const { from, to } of legalMoves) {
    if (!isSquareKey(from) || !isSquareKey(to)) {
      continue;
    }
    const targets = dests.get(from) ?? [];
    if (!targets.includes(to)) {
      targets.push(to);
    }
    dests.set(from, targets);
  }
  return dests;
}
