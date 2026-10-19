/**
 * Test della preparazione dell'albero delle mosse per la lista.
 */
import { describe, expect, it } from 'vitest';
import { sampleTree } from '../../../test/moveTrees';
import { buildMoveRows, variationTokens } from './buildMoveRows';

describe('buildMoveRows', () => {
  it('mette la linea principale in righe, con le varianti sotto la riga giusta', () => {
    const rows = buildMoveRows(sampleTree(null));

    expect(
      rows.map((row) => [row.moveNumber, row.white?.record.san, row.black?.record.san]),
    ).toEqual([
      [1, 'e4', 'e5'],
      [2, 'Nf3', undefined],
    ]);
    expect(rows[0]?.variations.map((first) => first.record.san)).toEqual(['c5']);
    expect(rows[1]?.variations).toEqual([]);
  });

  it('scrive una variante in fila, con le sotto-varianti tra parentesi', () => {
    const tree = sampleTree(null);
    const c5 = tree.nodes[3];
    if (c5 === undefined) {
      throw new Error('mossa di prova mancante');
    }

    const text = variationTokens(tree, c5)
      .map((token) =>
        token.kind === 'move'
          ? `${token.number === null ? '' : `${token.number} `}${token.node.record.san}`
          : token.kind === 'open'
            ? '('
            : ')',
      )
      .join(' ');

    expect(text).toBe('1… c5 2. Nf3 ( 2. c3 )');
  });
});
