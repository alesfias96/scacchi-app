/**
 * Test dello stato della partita. Il servizio verso Rust è simulato.
 */
import { beforeEach, describe, expect, it, vi } from 'vitest';
import type { GameSetup } from '../bindings/GameSetup';
import type { SessionView } from '../bindings/SessionView';
import { useGameStore } from './gameStore';

const serviceMock = vi.hoisted(() => ({
  fetchGame: vi.fn(),
  startNewGame: vi.fn(),
  playMove: vi.fn(),
  undoMove: vi.fn(),
  goToPly: vi.fn(),
  resignGame: vi.fn(),
  importGame: vi.fn(),
  exportPgn: vi.fn(),
  savePgnFile: vi.fn(),
  openPgnFile: vi.fn(),
  listenToGameUpdates: vi.fn(),
}));

vi.mock('../services/gameService', () => serviceMock);

const archiveMock = vi.hoisted(() => ({
  openSavedGame: vi.fn(),
  listenToStorageErrors: vi.fn(),
}));

vi.mock('../services/archiveService', () => archiveMock);

/** Una partita appena cominciata, a due giocatori. */
const NEW_GAME: SessionView = {
  id: 'partita',
  opening: null,
  game: {
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
};

/** La stessa partita dopo una mossa. */
const AFTER_E4: SessionView = { ...NEW_GAME, game: { ...NEW_GAME.game, cursor: 1 } };

describe('gameStore', () => {
  beforeEach(() => {
    vi.resetAllMocks();
    serviceMock.listenToGameUpdates.mockResolvedValue(() => undefined);
    archiveMock.listenToStorageErrors.mockResolvedValue(() => undefined);
    useGameStore.setState({
      session: null,
      loadError: null,
      actionError: null,
      storageError: null,
    });
  });

  // Va per primo: l'ascolto degli eventi si attiva una volta sola, al primo caricamento.
  it('segue i cambiamenti annunciati da Rust (es. la mossa del computer)', async () => {
    let announce: ((view: SessionView) => void) | undefined;
    serviceMock.listenToGameUpdates.mockImplementation((onUpdate: (view: SessionView) => void) => {
      announce = onUpdate;
      return Promise.resolve(() => undefined);
    });
    serviceMock.fetchGame.mockResolvedValue(NEW_GAME);
    await useGameStore.getState().load();

    announce?.(AFTER_E4);

    expect(useGameStore.getState().session).toEqual(AFTER_E4);
  });

  it('legge la partita da Rust', async () => {
    serviceMock.fetchGame.mockResolvedValue(NEW_GAME);

    await useGameStore.getState().load();

    expect(useGameStore.getState().session).toEqual(NEW_GAME);
  });

  it('annota il motivo se la partita non si può leggere', async () => {
    serviceMock.fetchGame.mockRejectedValue('comando non trovato');

    await useGameStore.getState().load();

    expect(useGameStore.getState().loadError).toBe('comando non trovato');
  });

  it('gioca una mossa e salva la partita aggiornata', async () => {
    serviceMock.playMove.mockResolvedValue(AFTER_E4);

    await useGameStore.getState().play('e2e4');

    expect(serviceMock.playMove).toHaveBeenCalledWith('e2e4');
    expect(useGameStore.getState().session).toEqual(AFTER_E4);
  });

  it('se una mossa viene rifiutata tiene la partita e spiega il motivo', async () => {
    useGameStore.setState({ session: NEW_GAME });
    serviceMock.playMove.mockRejectedValue('tocca al computer: aspetta la sua mossa');

    await useGameStore.getState().play('e2e4');

    expect(useGameStore.getState().session).toEqual(NEW_GAME);
    expect(useGameStore.getState().actionError).toBe('tocca al computer: aspetta la sua mossa');
  });

  it('comincia la partita scelta', async () => {
    const setup: GameSetup = {
      opponent: 'computer',
      color: 'black',
      engine: 'stockfish',
      level: 7,
      maiaLevel: 5,
      timeControl: null,
    };
    serviceMock.startNewGame.mockResolvedValue(NEW_GAME);

    await useGameStore.getState().newGame(setup);

    expect(serviceMock.startNewGame).toHaveBeenCalledWith(setup);
  });

  it('va alla mossa richiesta e abbandona', async () => {
    serviceMock.goToPly.mockResolvedValue(NEW_GAME);
    serviceMock.resignGame.mockResolvedValue(NEW_GAME);

    await useGameStore.getState().goTo(0);
    await useGameStore.getState().resign();

    expect(serviceMock.goToPly).toHaveBeenCalledWith(0);
    expect(serviceMock.resignGame).toHaveBeenCalled();
  });

  it('rilancia l’errore di un testo da incollare non valido', async () => {
    serviceMock.importGame.mockRejectedValue('PGN non valido');

    await expect(useGameStore.getState().importText('ciao')).rejects.toBe('PGN non valido');
  });

  it('apre una partita dell’archivio', async () => {
    archiveMock.openSavedGame.mockResolvedValue(AFTER_E4);

    await useGameStore.getState().openSaved('partita');

    expect(archiveMock.openSavedGame).toHaveBeenCalledWith('partita');
    expect(useGameStore.getState().session).toEqual(AFTER_E4);
  });

  it('apre un file PGN, e se si annulla non cambia nulla', async () => {
    useGameStore.setState({ session: NEW_GAME });
    serviceMock.openPgnFile.mockResolvedValueOnce(null).mockResolvedValueOnce(AFTER_E4);

    await useGameStore.getState().openFile();
    expect(useGameStore.getState().session).toEqual(NEW_GAME);

    await useGameStore.getState().openFile();
    expect(useGameStore.getState().session).toEqual(AFTER_E4);
  });
});
