/**
 * Test del controllo dei nomi delle case.
 */
import { describe, expect, it } from 'vitest';
import { isSquareKey } from './isSquareKey';

describe('isSquareKey', () => {
  it('accetta le case da a1 a h8', () => {
    expect(isSquareKey('a1')).toBe(true);
    expect(isSquareKey('h8')).toBe(true);
  });

  it('rifiuta tutto il resto', () => {
    expect(isSquareKey('i1')).toBe(false);
    expect(isSquareKey('a9')).toBe(false);
    expect(isSquareKey('e2e4')).toBe(false);
    expect(isSquareKey('')).toBe(false);
  });
});
