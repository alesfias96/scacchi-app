/**
 * Test della descrizione dell'esito di una partita dell'archivio.
 */
import { describe, expect, it } from 'vitest';
import type { GameSummary } from '../../../bindings/GameSummary';
import { describeOutcome } from './describeOutcome';

/** Una partita di prova contro il computer (che ha il Nero). */
function game(overrides: Partial<GameSummary>): GameSummary {
  return {
    id: 'a',
    startedAtMs: 0,
    opponent: 'computer',
    computerSide: 'black',
    level: 4,
    timeControl: null,
    result: null,
    opening: null,
    plyCount: 10,
    analyzed: false,
    ...overrides,
  };
}

describe('describeOutcome', () => {
  it('riconosce le partite non finite e le patte', () => {
    expect(describeOutcome(game({})).label).toBe('In corso');
    expect(
      describeOutcome(game({ result: { winner: null, reason: 'stalemate', score: '1/2-1/2' } })),
    ).toEqual({ label: 'Patta', tone: 'drawn' });
  });

  it('contro il computer dice se hai vinto o perso', () => {
    expect(
      describeOutcome(game({ result: { winner: 'white', reason: 'checkmate', score: '1-0' } })),
    ).toEqual({ label: 'Vinta', tone: 'won' });
    expect(
      describeOutcome(game({ result: { winner: 'black', reason: 'resignation', score: '0-1' } })),
    ).toEqual({ label: 'Persa', tone: 'lost' });
  });

  it('a due giocatori dice chi ha vinto', () => {
    const twoPlayers = game({
      opponent: 'twoPlayers',
      computerSide: null,
      result: { winner: 'black', reason: 'checkmate', score: '0-1' },
    });

    expect(describeOutcome(twoPlayers).label).toBe('Vince il Nero');
  });
});
