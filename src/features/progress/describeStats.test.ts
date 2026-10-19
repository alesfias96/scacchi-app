/**
 * Test delle parole e delle barre comuni alle schede di «Progressi».
 */
import { describe, expect, it } from 'vitest';
import { formatPercent, percent, scoreDetail, scoreSegments } from './describeStats';

describe('describeStats', () => {
  it('scrive i risultati al singolare e al plurale', () => {
    expect(scoreDetail({ wins: 3, draws: 1, losses: 0 })).toBe('3 vinte · 1 patta · 0 perse');
  });

  it('divide la barra dei risultati in parti proporzionali', () => {
    expect(scoreSegments({ wins: 1, draws: 1, losses: 2 })).toEqual([
      { percent: 25, tone: 'good' },
      { percent: 25, tone: 'neutral' },
      { percent: 50, tone: 'bad' },
    ]);
    expect(scoreSegments({ wins: 0, draws: 0, losses: 0 })).toEqual([]);
  });

  it('calcola e scrive le percentuali', () => {
    expect(percent(2, 3)).toBe(67);
    expect(percent(1, 0)).toBeNull();
    expect(formatPercent(null)).toBe('—');
    expect(formatPercent(67)).toBe('67%');
  });
});
