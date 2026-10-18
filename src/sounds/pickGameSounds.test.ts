/**
 * Test della scelta dei suoni quando la partita mostrata cambia.
 */
import { describe, expect, it } from 'vitest';
import type { GameResult } from '../bindings/GameResult';
import type { GameView } from '../bindings/GameView';
import type { MoveRecord } from '../bindings/MoveRecord';
import { pickGameSounds } from './pickGameSounds';

/** Una mossa di prova: contano solo la notazione UCI e se è una presa. */
function record(ply: number, uci: string, capture = false): MoveRecord {
  return {
    ply,
    moveNumber: Math.ceil(ply / 2),
    side: ply % 2 === 1 ? 'white' : 'black',
    san: uci,
    uci,
    from: uci.slice(0, 2),
    to: uci.slice(2, 4),
    capture,
  };
}

/** Una partita di prova: contano solo le mosse, il cursore e il risultato. */
function game(
  moves: MoveRecord[],
  cursor = moves.length,
  result: GameResult | null = null,
): GameView {
  return {
    position: { fen: '', boardFen: '', turn: 'white', inCheck: false, legalMoves: [] },
    moves,
    cursor,
    lastMove: null,
    result,
    startingFen: '',
  };
}

const E4 = record(1, 'e2e4');
const D5 = record(2, 'd7d5');
const EXD5 = record(3, 'e4d5', true);

describe('pickGameSounds', () => {
  it('suona la mossa appena giocata', () => {
    expect(pickGameSounds(game([E4]), game([E4, D5]))).toEqual({ move: 'move', end: null });
  });

  it('suona la presa', () => {
    expect(pickGameSounds(game([E4, D5]), game([E4, D5, EXD5]))).toEqual({
      move: 'capture',
      end: null,
    });
  });

  it('suona anche facendo un passo avanti nella partita', () => {
    expect(pickGameSounds(game([E4, D5, EXD5], 1), game([E4, D5, EXD5], 2))).toEqual({
      move: 'move',
      end: null,
    });
  });

  it('non suona tornando indietro, saltando o al primo caricamento', () => {
    const silent = { move: null, end: null };
    const full = game([E4, D5, EXD5]);

    expect(pickGameSounds(full, game([E4, D5, EXD5], 2))).toEqual(silent);
    expect(pickGameSounds(game([E4, D5, EXD5], 0), full)).toEqual(silent);
    expect(pickGameSounds(null, full)).toEqual(silent);
  });

  it('non suona se arriva un’altra partita (ad esempio incollata)', () => {
    const other = record(1, 'd2d4');

    expect(pickGameSounds(game([E4]), game([other, D5]))).toEqual({ move: null, end: null });
  });

  it('aggiunge il suono di fine partita: deciso per il matto, patta altrimenti', () => {
    const mate: GameResult = { winner: 'black', reason: 'checkmate', score: '0-1' };
    const draw: GameResult = { winner: null, reason: 'stalemate', score: '1/2-1/2' };

    expect(pickGameSounds(game([E4]), game([E4, D5], 2, mate)).end).toBe('decisive');
    expect(pickGameSounds(game([E4]), game([E4, D5], 2, draw)).end).toBe('draw');
  });
});
