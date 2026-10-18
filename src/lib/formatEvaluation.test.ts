/**
 * Test della scrittura delle valutazioni.
 */
import { describe, expect, it } from 'vitest';
import { formatEvaluation } from './formatEvaluation';

describe('formatEvaluation', () => {
  it('scrive i centipedoni in pedoni, con il segno', () => {
    expect(formatEvaluation({ kind: 'centipawns', value: 35 })).toBe('+0.3');
    expect(formatEvaluation({ kind: 'centipawns', value: -120 })).toBe('−1.2');
    expect(formatEvaluation({ kind: 'centipawns', value: 0 })).toBe('0.0');
    expect(formatEvaluation({ kind: 'centipawns', value: 1250 })).toBe('+13');
  });

  it('scrive lo scacco matto in vista', () => {
    expect(formatEvaluation({ kind: 'mate', value: 3 })).toBe('M3');
    expect(formatEvaluation({ kind: 'mate', value: -2 })).toBe('−M2');
  });
});
