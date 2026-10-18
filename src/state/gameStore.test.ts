/**
 * Test dello stato della partita. Il servizio verso Rust è simulato.
 */
import { beforeEach, describe, expect, it, vi } from 'vitest';
import type { GameView } from '../bindings/GameView';
import { useGameStore } from './gameStore';

const serviceMock = vi.hoisted(() => ({
  fetchGame: vi.fn(),
  startNewGame: vi.fn(),
  playMove: vi.fn(),
  undoMove: vi.fn(),
  goToPly: vi.fn(),
  importGame: vi.fn(),
  exportPgn: vi.fn(),
}));

vi.mock('../services/gameService', () => serviceMock);

/** Una partita appena cominciata. */
const NEW_GAME: GameView = {
  position: {
    fen: 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1',
    boardFen: 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR',
    turn: 'white',
    inCheck: false,
    legalMoves: [],
  },
  moves: [],
  cursor: 0,
  lastMove: null,
  result: null,
  startingFen: 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1',
};

describe('gameStore', () => {
  beforeEach(() => {
    vi.resetAllMocks();
    useGameStore.setState({ game: null, loadError: null, actionError: null });
  });

  it('legge la partita da Rust', async () => {
    serviceMock.fetchGame.mockResolvedValue(NEW_GAME);

    await useGameStore.getState().load();

    expect(useGameStore.getState().game).toEqual(NEW_GAME);
  });

  it('annota il motivo se la partita non si può leggere', async () => {
    serviceMock.fetchGame.mockRejectedValue('comando non trovato');

    await useGameStore.getState().load();

    expect(useGameStore.getState().loadError).toBe('comando non trovato');
  });

  it('gioca una mossa e salva la partita aggiornata', async () => {
    const afterE4 = { ...NEW_GAME, cursor: 1 };
    serviceMock.playMove.mockResolvedValue(afterE4);

    await useGameStore.getState().play('e2e4');

    expect(serviceMock.playMove).toHaveBeenCalledWith('e2e4');
    expect(useGameStore.getState().game).toEqual(afterE4);
  });

  it('se una mossa viene rifiutata tiene la partita e spiega il motivo', async () => {
    useGameStore.setState({ game: NEW_GAME });
    serviceMock.playMove.mockRejectedValue('Mossa non legale: e2e5');

    await useGameStore.getState().play('e2e5');

    expect(useGameStore.getState().game).toEqual(NEW_GAME);
    expect(useGameStore.getState().actionError).toBe('Mossa non legale: e2e5');
  });

  it('va alla mossa richiesta', async () => {
    serviceMock.goToPly.mockResolvedValue(NEW_GAME);

    await useGameStore.getState().goTo(3);

    expect(serviceMock.goToPly).toHaveBeenCalledWith(3);
  });

  it("rilancia l'errore di un'importazione non valida", async () => {
    serviceMock.importGame.mockRejectedValue('Testo vuoto');

    await expect(useGameStore.getState().importText('')).rejects.toBe('Testo vuoto');
  });
});
