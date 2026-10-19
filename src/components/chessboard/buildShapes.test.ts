/**
 * Test della preparazione dei segni sulla scacchiera.
 */
import { describe, expect, it } from 'vitest';
import { buildShapes } from './buildShapes';

describe('buildShapes', () => {
  it('senza nulla da indicare non disegna nulla', () => {
    expect(buildShapes(null, null)).toEqual([]);
  });

  it('disegna la mossa migliore in verde e i segni delle osservazioni in blu', () => {
    const shapes = buildShapes(
      { from: 'g1', to: 'f3', uci: 'g1f3' },
      { arrows: [{ from: 'c7', to: 'e8' }], circles: ['a8'] },
    );

    expect(shapes).toEqual([
      { orig: 'g1', dest: 'f3', brush: 'paleGreen' },
      { orig: 'c7', dest: 'e8', brush: 'blue' },
      { orig: 'a8', brush: 'blue' },
    ]);
  });

  it('ignora le case non valide', () => {
    expect(buildShapes(null, { arrows: [{ from: 'z9', to: 'e4' }], circles: ['k0'] })).toEqual([]);
  });
});
