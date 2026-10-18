/**
 * Test della parte di barra che spetta al Bianco.
 */
import { describe, expect, it } from 'vitest';
import { whiteShare } from './whiteShare';

describe('whiteShare', () => {
  it('a metà per la parità', () => {
    expect(whiteShare({ kind: 'centipawns', value: 0 })).toBe(0.5);
  });

  it('più del Bianco se il Bianco sta meglio, simmetrica per il Nero', () => {
    const white = whiteShare({ kind: 'centipawns', value: 300 });
    const black = whiteShare({ kind: 'centipawns', value: -300 });

    expect(white).toBeGreaterThan(0.7);
    expect(white + black).toBeCloseTo(1);
  });

  it('piena per chi dà scacco matto', () => {
    expect(whiteShare({ kind: 'mate', value: 3 })).toBe(1);
    expect(whiteShare({ kind: 'mate', value: -2 })).toBe(0);
  });
});
