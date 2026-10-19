/**
 * Test dei suoni della partita. I suoni veri sono simulati: si verifica
 * quale suono parte e quando.
 */
import { render } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import type { GameResult } from '../bindings/GameResult';
import type { GameView } from '../bindings/GameView';
import type { MoveRecord } from '../bindings/MoveRecord';
import { useMoveSound } from './useMoveSound';

const soundMock = vi.hoisted(() => ({
  playMoveSound: vi.fn(),
  playCaptureSound: vi.fn(),
  playGameEndSound: vi.fn(),
}));

vi.mock('../sounds/playMoveSound', () => ({ playMoveSound: soundMock.playMoveSound }));
vi.mock('../sounds/playCaptureSound', () => ({ playCaptureSound: soundMock.playCaptureSound }));
vi.mock('../sounds/playGameEndSound', () => ({ playGameEndSound: soundMock.playGameEndSound }));

/** Un componente di prova che usa l'hook. */
function SoundProbe({ game, enabled }: { game: GameView | null; enabled: boolean }) {
  useMoveSound(game, enabled);
  return null;
}

/** Una mossa di prova. */
function record(ply: number, uci: string, capture = false): MoveRecord {
  return { ply, moveNumber: 1, side: 'white', san: uci, uci, from: '', to: '', capture };
}

/** Una partita di prova con le mosse indicate, mostrata alla fine. */
function game(moves: MoveRecord[], result: GameResult | null = null): GameView {
  return {
    position: { fen: '', boardFen: '', turn: 'white', inCheck: false, legalMoves: [] },
    moves,
    cursor: moves.length,
    lastMove: null,
    result,
    startingFen: '',
    tree: { nodes: [], roots: [], current: null },
  };
}

const E4 = record(1, 'e2e4');
const D5 = record(2, 'd7d5');
const EXD5 = record(3, 'e4d5', true);

describe('useMoveSound', () => {
  beforeEach(() => {
    vi.resetAllMocks();
  });

  it('suona la mossa e la presa quando vengono giocate', () => {
    const { rerender } = render(<SoundProbe game={game([E4])} enabled />);
    expect(soundMock.playMoveSound).not.toHaveBeenCalled();

    rerender(<SoundProbe game={game([E4, D5])} enabled />);
    expect(soundMock.playMoveSound).toHaveBeenCalledOnce();

    rerender(<SoundProbe game={game([E4, D5, EXD5])} enabled />);
    expect(soundMock.playCaptureSound).toHaveBeenCalledOnce();
    expect(soundMock.playMoveSound).toHaveBeenCalledOnce();
  });

  it('dopo il colpo della mossa suona la fine partita', () => {
    const mate: GameResult = { winner: 'black', reason: 'checkmate', score: '0-1' };
    const { rerender } = render(<SoundProbe game={game([E4])} enabled />);

    rerender(<SoundProbe game={game([E4, D5], mate)} enabled />);

    expect(soundMock.playMoveSound).toHaveBeenCalledOnce();
    expect(soundMock.playGameEndSound).toHaveBeenCalledWith('decisive', expect.any(Number));
  });

  it('non suona con i suoni disattivati, né quando si riattivano', () => {
    const { rerender } = render(<SoundProbe game={game([E4])} enabled={false} />);
    const current = game([E4, D5]);

    rerender(<SoundProbe game={current} enabled={false} />);
    rerender(<SoundProbe game={current} enabled />);

    expect(soundMock.playMoveSound).not.toHaveBeenCalled();
  });
});
