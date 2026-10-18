/**
 * Test della descrizione del risultato.
 */
import { describe, expect, it } from 'vitest';
import { describeResult } from './describeResult';

describe('describeResult', () => {
  it('descrive una vittoria', () => {
    expect(describeResult({ winner: 'black', reason: 'checkmate', score: '0-1' })).toEqual({
      title: 'Scacco matto',
      detail: 'Vince il Nero',
    });
  });

  it('descrive una patta', () => {
    expect(
      describeResult({ winner: null, reason: 'threefoldRepetition', score: '1/2-1/2' }),
    ).toEqual({ title: 'Triplice ripetizione', detail: 'Patta' });
  });

  it('descrive tempo scaduto e abbandono', () => {
    expect(describeResult({ winner: 'white', reason: 'timeout', score: '1-0' }).title).toBe(
      'Tempo scaduto',
    );
    expect(describeResult({ winner: 'black', reason: 'resignation', score: '0-1' })).toEqual({
      title: 'Abbandono',
      detail: 'Vince il Nero',
    });
  });
});
