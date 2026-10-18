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
});
