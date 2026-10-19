/**
 * Test della descrizione dei giocatori.
 */
import { describe, expect, it } from 'vitest';
import type { SessionView } from '../../../bindings/SessionView';
import { describePlayer } from './describePlayer';

/** Una partita di prova: contano solo avversario, orologio e "sta pensando". */
function session(overrides: Partial<SessionView>): SessionView {
  return {
    id: 'partita',
    opening: null,
    game: {
      position: { fen: '', boardFen: '', turn: 'white', inCheck: false, legalMoves: [] },
      moves: [],
      cursor: 0,
      lastMove: null,
      result: null,
      startingFen: '',
      tree: { nodes: [], roots: [], current: null },
    },
    opponent: 'twoPlayers',
    computer: null,
    timeControl: null,
    clock: null,
    canMove: true,
    canUndo: false,
    canResign: true,
    computerThinking: false,
    canEditVariations: false,
    ...overrides,
  };
}

describe('describePlayer', () => {
  it('a due giocatori usa i nomi dei colori, senza orologio', () => {
    expect(describePlayer(session({}), 'black')).toEqual({
      side: 'black',
      name: 'Nero',
      detail: undefined,
      clock: null,
      thinking: false,
    });
  });

  it('contro il computer distingue Stockfish (con il livello) da te', () => {
    const playing = session({
      opponent: 'computer',
      computer: {
        side: 'black',
        engine: 'stockfish',
        level: { level: 4, name: 'Circolo', elo: 1400 },
      },
      computerThinking: true,
      canEditVariations: false,
    });

    expect(describePlayer(playing, 'black')).toEqual(
      expect.objectContaining({ name: 'Stockfish', detail: 'Circolo · ~1400 Elo', thinking: true }),
    );
    expect(describePlayer(playing, 'white')).toEqual(
      expect.objectContaining({ name: 'Tu', thinking: false }),
    );
  });

  it('contro Maia il computer si chiama Maia', () => {
    const playing = session({
      opponent: 'computer',
      computer: {
        side: 'white',
        engine: 'maia',
        level: { level: 5, name: 'Maia 1500', elo: 1500 },
      },
      canEditVariations: false,
    });

    expect(describePlayer(playing, 'white')).toEqual(
      expect.objectContaining({ name: 'Maia', detail: 'Maia 1500 · ~1500 Elo' }),
    );
  });

  it('dà a ciascuno il suo tempo e dice quale sta scorrendo', () => {
    const timed = session({ clock: { whiteMs: 60_000, blackMs: 55_000, running: 'black' } });

    expect(describePlayer(timed, 'white').clock).toEqual({ ms: 60_000, running: false });
    expect(describePlayer(timed, 'black').clock).toEqual({ ms: 55_000, running: true });
  });
});
