/**
 * Test della scrittura del tempo dell'orologio.
 */
import { describe, expect, it } from 'vitest';
import { formatClock } from './formatClock';

describe('formatClock', () => {
  it('scrive minuti e secondi', () => {
    expect(formatClock(247_000)).toBe('4:07');
    expect(formatClock(600_000)).toBe('10:00');
  });

  it('aggiunge le ore quando servono', () => {
    expect(formatClock(3_909_000)).toBe('1:05:09');
  });

  it('negli ultimi secondi mostra i decimi', () => {
    expect(formatClock(9_480)).toBe('9.4');
    expect(formatClock(0)).toBe('0.0');
    expect(formatClock(-50)).toBe('0.0');
  });
});
