/**
 * Test dei collegamenti degli appunti.
 */
import { describe, expect, it } from 'vitest';
import type { MoveRecord } from '../../bindings/MoveRecord';
import type { SessionView } from '../../bindings/SessionView';
import { sampleTree } from '../../test/moveTrees';
import { fenBoard, noteLabel, playLink } from './noteLinks';

/** Una mossa qualsiasi (contano solo quante sono). */
const MOVE: MoveRecord = {
  ply: 1,
  moveNumber: 1,
  side: 'white',
  san: 'e4',
  uci: 'e2e4',
  from: 'e2',
  to: 'e4',
  capture: false,
};

/** Una partita salvata con tre mosse, che mostra la mossa `current` dell'albero di prova. */
function session(current: number | null, moves: MoveRecord[] = [MOVE, MOVE, MOVE]): SessionView {
  return {
    id: 'partita-1',
    opening: null,
    game: {
      position: {
        fen: 'FEN mostrata',
        boardFen: '',
        turn: 'white',
        inCheck: false,
        legalMoves: [],
      },
      moves,
      cursor: 2,
      lastMove: null,
      result: null,
      startingFen: '',
      tree: sampleTree(current),
    },
    opponent: 'twoPlayers',
    computer: null,
    timeControl: null,
    clock: null,
    canMove: true,
    canUndo: true,
    canResign: true,
    computerThinking: false,
    canEditVariations: true,
  };
}

describe('noteLinks', () => {
  it('in Gioca collega la partita solo sulla linea principale', () => {
    expect(playLink(session(1))).toEqual({ fen: 'FEN mostrata', gameId: 'partita-1', ply: 2 });
    // Nella variante 1... c5 la partita no: si ricorda solo la posizione.
    expect(playLink(session(3))).toEqual({ fen: 'FEN mostrata', gameId: null, ply: null });
    // Una partita senza mosse non è ancora nell'archivio.
    expect(playLink(session(null, [])).gameId).toBeNull();
  });

  it('legge dalla FEN i pezzi e chi muove', () => {
    expect(fenBoard('8/8/8/8/8/8/8/K6k b - - 0 1')).toEqual({
      boardFen: '8/8/8/8/8/8/8/K6k',
      turn: 'black',
    });
  });

  it('usa il titolo, oppure la prima riga del testo', () => {
    expect(noteLabel('Idea', 'testo')).toBe('Idea');
    expect(noteLabel('  ', '\n  Spingere f5  \naltro')).toBe('Spingere f5');
    expect(noteLabel('', '')).toBe('Appunto senza titolo');
  });
});
