/**
 * Test della preparazione delle mosse legali per chessground.
 */
import { describe, expect, it } from 'vitest';
import { buildDests } from './buildDests';

describe('buildDests', () => {
  it('raggruppa le case di arrivo per casa di partenza', () => {
    const dests = buildDests([
      { from: 'e2', to: 'e3', uci: 'e2e3' },
      { from: 'e2', to: 'e4', uci: 'e2e4' },
      { from: 'g1', to: 'f3', uci: 'g1f3' },
    ]);

    expect(dests.get('e2')).toEqual(['e3', 'e4']);
    expect(dests.get('g1')).toEqual(['f3']);
  });

  it('mostra una sola volta le quattro promozioni sulla stessa casa', () => {
    const dests = buildDests(
      ['q', 'r', 'b', 'n'].map((piece) => ({ from: 'e7', to: 'e8', uci: `e7e8${piece}` })),
    );

    expect(dests.get('e7')).toEqual(['e8']);
  });

  it('scarta case non valide', () => {
    const dests = buildDests([{ from: 'z9', to: 'e4', uci: 'z9e4' }]);

    expect(dests.size).toBe(0);
  });
});
