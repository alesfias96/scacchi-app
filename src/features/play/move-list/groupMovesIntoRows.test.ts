/**
 * Test del raggruppamento delle mosse in righe.
 */
import { describe, expect, it } from 'vitest';
import type { MoveRecord } from '../../../bindings/MoveRecord';
import type { Side } from '../../../bindings/Side';
import { groupMovesIntoRows } from './groupMovesIntoRows';

/** Crea una mossa di prova (solo i campi che contano qui sono realistici). */
function move(ply: number, moveNumber: number, side: Side, san: string): MoveRecord {
  return { ply, moveNumber, side, san, uci: '', from: '', to: '' };
}

describe('groupMovesIntoRows', () => {
  it('mette bianco e nero sulla stessa riga', () => {
    const rows = groupMovesIntoRows([
      move(1, 1, 'white', 'e4'),
      move(2, 1, 'black', 'e5'),
      move(3, 2, 'white', 'Nf3'),
    ]);

    expect(rows.map((row) => [row.moveNumber, row.white?.san, row.black?.san])).toEqual([
      [1, 'e4', 'e5'],
      [2, 'Nf3', undefined],
    ]);
  });

  it('se comincia il nero, la prima riga non ha la mossa del bianco', () => {
    const rows = groupMovesIntoRows([move(1, 1, 'black', 'e5'), move(2, 2, 'white', 'Nf3')]);

    expect(rows.map((row) => [row.moveNumber, row.white?.san, row.black?.san])).toEqual([
      [1, undefined, 'e5'],
      [2, 'Nf3', undefined],
    ]);
  });

  it('nessuna mossa, nessuna riga', () => {
    expect(groupMovesIntoRows([])).toEqual([]);
  });
});
