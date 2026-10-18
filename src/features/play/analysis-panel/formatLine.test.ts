/**
 * Test della scrittura della variante del motore.
 */
import { describe, expect, it } from 'vitest';
import type { LineView } from '../../../bindings/LineView';
import { formatLine } from './formatLine';

/** Una variante di prova. */
function line(turn: LineView['turn'], moveNumber: number, san: string[]): LineView {
  return { fen: '', turn, moveNumber, san, firstMove: null };
}

describe('formatLine', () => {
  it('numera le mosse a partire dal Bianco', () => {
    expect(formatLine(line('white', 12, ['Nf3', 'e5', 'Bc4']), 'internazionale', 10)).toBe(
      '12. Nf3 e5 13. Bc4',
    );
  });

  it('se comincia il Nero mette i puntini dopo il numero', () => {
    expect(formatLine(line('black', 12, ['e5', 'Nf3']), 'italiana', 10)).toBe('12… e5 13. Cf3');
  });

  it('accorcia le varianti lunghe', () => {
    expect(formatLine(line('white', 1, ['e4', 'e5', 'Nf3', 'Nc6']), 'figurine', 2)).toBe(
      '1. e4 e5',
    );
  });
});
