/**
 * Test della scrittura dei tempi di riflessione.
 */
import { describe, expect, it } from 'vitest';
import { describeTimeControl } from './describeTimeControl';

describe('describeTimeControl', () => {
  it('scrive minuti e incremento', () => {
    expect(describeTimeControl({ initialSeconds: 300, incrementSeconds: 3 })).toBe('5+3');
    expect(describeTimeControl({ initialSeconds: 30, incrementSeconds: 0 })).toBe('½+0');
    expect(describeTimeControl({ initialSeconds: 90, incrementSeconds: 1 })).toBe('1.5+1');
  });

  it('riconosce la partita senza orologio', () => {
    expect(describeTimeControl(null)).toBe('Senza orologio');
  });
});
