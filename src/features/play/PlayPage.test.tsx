/**
 * Test della sezione "Gioca".
 *
 * I servizi verso Rust e chessground sono simulati: si verifica che la
 * pagina mostri la partita ricevuta, invii a Rust le mosse fatte sulla
 * scacchiera (promozione compresa), blocchi la scacchiera quando non si può
 * muovere, annunci la fine della partita, apra la finestra "Nuova partita",
 * accenda l'analisi e risponda a tastiera e comandi.
 */
import type { Config } from '@lichess-org/chessground/config';
import type { Key } from '@lichess-org/chessground/types';
import { fireEvent, render, screen, waitFor } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import type { GameView } from '../../bindings/GameView';
import type { SessionView } from '../../bindings/SessionView';
import { useEngineStore } from '../../state/engineStore';
import { useGameStore } from '../../state/gameStore';
import { useObservationsStore } from '../../state/observationsStore';
import { useSettingsStore } from '../../state/settingsStore';
import { AFTER_E4_SCENARIO, START_EXPLANATION } from '../../test/tutorExplanations';
import { CoachedPlayPage, PlayPage } from './PlayPage';

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

vi.mock('../../services/gameService', () => serviceMock);

const archiveMock = vi.hoisted(() => ({
  openSavedGame: vi.fn(),
  listenToStorageErrors: vi.fn(),
}));

vi.mock('../../services/archiveService', () => archiveMock);

const insightMock = vi.hoisted(() => ({ fetchPositionReport: vi.fn() }));

vi.mock('../../services/insightService', () => insightMock);

const tutorMock = vi.hoisted(() => ({ explainPosition: vi.fn(), tryWhatIf: vi.fn() }));

vi.mock('../../services/tutorService', () => tutorMock);

const engineServiceMock = vi.hoisted(() => ({
  fetchEngineLevels: vi.fn(),
  fetchMaiaLevels: () => Promise.resolve([]),
  setAnalysis: vi.fn(),
  listenToAnalysis: vi.fn(),
  listenToEngineErrors: vi.fn(),
}));

vi.mock('../../services/engineService', () => engineServiceMock);

const settingsServiceMock = vi.hoisted(() => ({
  fetchSettings: vi.fn(),
  saveSettings: vi.fn(),
}));

vi.mock('../../services/settingsService', () => settingsServiceMock);

/** chessground simulato: ricorda l'ultima configurazione ricevuta (vedi beforeEach). */
const chessgroundMock = vi.hoisted(() => ({
  state: { config: undefined as Config | undefined },
  create: vi.fn(),
}));

vi.mock('@lichess-org/chessground', () => ({ Chessground: chessgroundMock.create }));

/** Simula l'utente che sposta un pezzo sulla scacchiera. */
function dragPiece(from: Key, to: Key) {
  chessgroundMock.state.config?.movable?.events?.after?.(from, to, { premove: false });
}

/** Partita dopo 1.e4: muove il nero. */
const AFTER_E4_GAME: GameView = {
  position: {
    fen: 'rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1',
    boardFen: 'rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR',
    turn: 'black',
    inCheck: false,
    legalMoves: [{ from: 'e7', to: 'e5', uci: 'e7e5' }],
  },
  moves: [
    {
      ply: 1,
      moveNumber: 1,
      side: 'white',
      san: 'e4',
      uci: 'e2e4',
      from: 'e2',
      to: 'e4',
      capture: false,
    },
  ],
  cursor: 1,
  lastMove: { from: 'e2', to: 'e4', uci: 'e2e4' },
  result: null,
  startingFen: 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1',
  tree: {
    nodes: [
      {
        id: 0,
        record: {
          ply: 1,
          moveNumber: 1,
          side: 'white',
          san: 'e4',
          uci: 'e2e4',
          from: 'e2',
          to: 'e4',
          capture: false,
        },
        parent: null,
        children: [],
        mainline: true,
      },
    ],
    roots: [0],
    current: 0,
  },
};

/** Una partita a due giocatori, senza orologio. */
function twoPlayers(game: GameView): SessionView {
  return {
    id: 'partita-a-due',
    opening: null,
    game,
    opponent: 'twoPlayers',
    computer: null,
    timeControl: null,
    clock: null,
    canMove: game.result === null,
    canUndo: game.moves.length > 0,
    canResign: game.result === null,
    computerThinking: false,
    canEditVariations: false,
  };
}

const AFTER_E4 = twoPlayers(AFTER_E4_GAME);

/** Un pedone bianco in e7 pronto a promuovere. */
const PROMOTION = twoPlayers({
  ...AFTER_E4_GAME,
  position: {
    fen: '8/4P3/8/7k/8/8/8/4K3 w - - 0 1',
    boardFen: '8/4P3/8/7k/8/8/8/4K3',
    turn: 'white',
    inCheck: false,
    legalMoves: ['q', 'r', 'b', 'n'].map((piece) => ({
      from: 'e7',
      to: 'e8',
      uci: `e7e8${piece}`,
    })),
  },
});

/** Contro il computer (con il nero), mentre il computer pensa. */
const COMPUTER_THINKING: SessionView = {
  ...AFTER_E4,
  id: 'partita-col-computer',
  opponent: 'computer',
  computer: { side: 'black', engine: 'stockfish', level: { level: 4, name: 'Circolo', elo: 1400 } },
  timeControl: { initialSeconds: 300, incrementSeconds: 0 },
  clock: { whiteMs: 300_000, blackMs: 299_000, running: 'black' },
  canMove: false,
  canUndo: true,
  computerThinking: true,
  canEditVariations: false,
};

/** Aspetta che la pagina abbia ricevuto la partita. */
async function pageReady() {
  await screen.findByRole('region', { name: 'Situazione' });
}

describe('PlayPage', () => {
  beforeEach(() => {
    vi.resetAllMocks();
    serviceMock.listenToGameUpdates.mockResolvedValue(() => undefined);
    archiveMock.listenToStorageErrors.mockResolvedValue(() => undefined);
    engineServiceMock.listenToAnalysis.mockResolvedValue(() => undefined);
    engineServiceMock.listenToEngineErrors.mockResolvedValue(() => undefined);
    engineServiceMock.setAnalysis.mockResolvedValue(undefined);
    engineServiceMock.fetchEngineLevels.mockResolvedValue([
      { level: 4, name: 'Circolo', elo: 1400 },
    ]);
    settingsServiceMock.saveSettings.mockResolvedValue(undefined);
    chessgroundMock.create.mockImplementation((_element: HTMLElement, config: Config) => {
      chessgroundMock.state.config = config;
      return {
        set: (next: Config) => {
          chessgroundMock.state.config = next;
        },
        destroy: vi.fn(),
      };
    });
    chessgroundMock.state.config = undefined;
    useGameStore.setState({
      session: null,
      loadError: null,
      actionError: null,
      storageError: null,
    });
    useEngineStore.setState({ analysisEnabled: false, latest: null, engineError: null });
    useObservationsStore.setState({ enabled: false });
    useSettingsStore.setState({
      settings: {
        theme: 'studio',
        board: 'rovere',
        pieces: 'icona',
        notation: 'italiana',
        sounds: 'on',
        gameSetup: {
          opponent: 'computer',
          color: 'white',
          engine: 'stockfish',
          level: 4,
          maiaLevel: 5,
          timeControl: null,
        },
        trainingSetup: {
          mode: 'depth',
          depth: 2,
          theme: null,
          difficulty: 'normal',
          visualizationSeconds: 5,
        },
        statsFilter: { period: 'all', opponent: 'all' },
        updateCheck: 'manual',
      },
    });
  });

  it('mostra un messaggio di attesa mentre arriva la partita', () => {
    serviceMock.fetchGame.mockReturnValue(new Promise(() => undefined));

    render(<PlayPage />);

    expect(screen.getByText('Preparo la scacchiera…')).toBeDefined();
  });

  it('mostra la partita ricevuta da Rust', async () => {
    serviceMock.fetchGame.mockResolvedValue(AFTER_E4);

    render(<PlayPage />);

    await pageReady();
    expect(screen.getByRole('button', { name: 'e4' })).toBeDefined();
    expect(screen.getByText('Bianco')).toBeDefined();
    expect(chessgroundMock.state.config?.lastMove).toEqual(['e2', 'e4']);
  });

  it('in modalità Tutor commenta la mossa sulla scacchiera, un messaggio alla volta', async () => {
    serviceMock.fetchGame.mockResolvedValue(AFTER_E4);
    tutorMock.tryWhatIf.mockResolvedValue(AFTER_E4_SCENARIO);
    tutorMock.explainPosition.mockResolvedValue(START_EXPLANATION);

    render(<CoachedPlayPage />);

    expect(await screen.findByText(/Ottima scelta/)).toBeDefined();
    expect(tutorMock.tryWhatIf).toHaveBeenCalledWith(AFTER_E4_GAME.startingFen, [], 'e2e4', true);
    await waitFor(() => {
      expect(chessgroundMock.state.config?.drawable?.autoShapes).toEqual([
        { orig: 'e2', dest: 'e4', brush: 'blue' },
      ]);
    });

    fireEvent.click(screen.getByRole('button', { name: 'Avanti' }));
    expect(screen.getByText('Tocca al Bianco')).toBeDefined();
  });

  it('spiega il problema se Rust non risponde', async () => {
    serviceMock.fetchGame.mockRejectedValue('comando non trovato');

    render(<PlayPage />);

    expect((await screen.findByRole('alert')).textContent).toContain('comando non trovato');
  });

  it('invia a Rust la mossa fatta sulla scacchiera', async () => {
    serviceMock.fetchGame.mockResolvedValue(AFTER_E4);
    serviceMock.playMove.mockResolvedValue(AFTER_E4);
    render(<PlayPage />);
    await pageReady();

    dragPiece('e7', 'e5');

    expect(serviceMock.playMove).toHaveBeenCalledWith('e7e5');
  });

  it('per la promozione chiede il pezzo e invia la mossa completa', async () => {
    serviceMock.fetchGame.mockResolvedValue(PROMOTION);
    serviceMock.playMove.mockResolvedValue(PROMOTION);
    render(<PlayPage />);
    await pageReady();

    dragPiece('e7', 'e8');
    fireEvent.click(await screen.findByRole('button', { name: 'Cavallo' }));

    expect(serviceMock.playMove).toHaveBeenCalledWith('e7e8n');
    expect(screen.queryByRole('dialog')).toBeNull();
  });

  it('annuncia la fine della partita e blocca la scacchiera', async () => {
    serviceMock.fetchGame.mockResolvedValue(
      twoPlayers({
        ...AFTER_E4_GAME,
        result: { winner: 'black', reason: 'checkmate', score: '0-1' },
      }),
    );

    render(<PlayPage />);

    expect(await screen.findByText('Scacco matto')).toBeDefined();
    expect(chessgroundMock.state.config?.viewOnly).toBe(true);
  });

  it('mentre pensa il computer blocca la scacchiera e lo dice', async () => {
    serviceMock.fetchGame.mockResolvedValue(COMPUTER_THINKING);

    render(<PlayPage />);
    await pageReady();

    expect(chessgroundMock.state.config?.viewOnly).toBe(true);
    expect(screen.getByText('Stockfish')).toBeDefined();
    expect(screen.getByText('Tu')).toBeDefined();
    expect(screen.getByText('sta pensando')).toBeDefined();
    expect(screen.getByLabelText('Tempo del Bianco').textContent).toBe('5:00');
  });

  it('la freccia sinistra torna indietro di una mossa', async () => {
    serviceMock.fetchGame.mockResolvedValue(AFTER_E4);
    serviceMock.goToPly.mockResolvedValue(AFTER_E4);
    render(<PlayPage />);
    await pageReady();

    fireEvent.keyDown(window, { key: 'ArrowLeft' });

    expect(serviceMock.goToPly).toHaveBeenCalledWith(0);
  });

  it('"Gira la scacchiera" mostra il nero in basso', async () => {
    serviceMock.fetchGame.mockResolvedValue(AFTER_E4);
    render(<PlayPage />);

    fireEvent.click(await screen.findByRole('button', { name: 'Gira la scacchiera' }));

    await waitFor(() => {
      expect(chessgroundMock.state.config?.orientation).toBe('black');
    });
  });

  it('"Nuova partita" apre la finestra e avvia la partita scelta, con il proprio colore in basso', async () => {
    serviceMock.fetchGame.mockResolvedValue(AFTER_E4);
    serviceMock.startNewGame.mockResolvedValue({
      ...COMPUTER_THINKING,
      id: 'partita-nuova',
      computer: {
        side: 'white',
        engine: 'stockfish',
        level: { level: 4, name: 'Circolo', elo: 1400 },
      },
    });
    render(<PlayPage />);
    await pageReady();

    fireEvent.click(screen.getByRole('button', { name: /Nuova partita/ }));
    fireEvent.click(screen.getByRole('radio', { name: /Nero/ }));
    fireEvent.click(screen.getByRole('button', { name: 'Inizia la partita' }));

    await waitFor(() => {
      expect(chessgroundMock.state.config?.orientation).toBe('black');
    });
    expect(serviceMock.startNewGame).toHaveBeenCalledWith(
      expect.objectContaining({ opponent: 'computer', color: 'black' }),
    );
    expect(useSettingsStore.getState().settings?.gameSetup.color).toBe('black');
  });

  it('accende l’analisi e mostra la barra di valutazione', async () => {
    serviceMock.fetchGame.mockResolvedValue(AFTER_E4);
    render(<PlayPage />);
    await pageReady();

    fireEvent.click(screen.getByRole('switch', { name: 'Mostra analisi di Stockfish' }));

    expect(engineServiceMock.setAnalysis).toHaveBeenCalledWith(true);
    expect(await screen.findByRole('meter', { name: 'Valutazione della posizione' })).toBeDefined();
  });

  it('mostra la freccia della mossa migliore della posizione sulla scacchiera', async () => {
    serviceMock.fetchGame.mockResolvedValue(AFTER_E4);
    useEngineStore.setState({
      analysisEnabled: true,
      latest: {
        depth: 10,
        evaluation: { kind: 'centipawns', value: 20 },
        line: {
          fen: AFTER_E4_GAME.position.fen,
          turn: 'black',
          moveNumber: 1,
          san: ['e5'],
          firstMove: { from: 'e7', to: 'e5', uci: 'e7e5' },
        },
      },
    });

    render(<PlayPage />);
    await pageReady();

    expect(chessgroundMock.state.config?.drawable?.autoShapes).toEqual([
      expect.objectContaining({ orig: 'e7', dest: 'e5' }),
    ]);
    expect(screen.getByText('profondità 10')).toBeDefined();
  });

  it('mostra il nome dell’apertura sopra le mosse', async () => {
    serviceMock.fetchGame.mockResolvedValue({
      ...AFTER_E4,
      opening: { eco: 'B00', name: "King's Pawn Game" },
    });

    render(<PlayPage />);
    await pageReady();

    expect(screen.getByText('B00')).toBeDefined();
    expect(screen.getByText("King's Pawn Game")).toBeDefined();
  });

  it('una partita aperta col nero mostra il nero in basso', async () => {
    serviceMock.fetchGame.mockResolvedValue({
      ...COMPUTER_THINKING,
      computer: {
        side: 'white',
        engine: 'stockfish',
        level: { level: 4, name: 'Circolo', elo: 1400 },
      },
    });

    render(<PlayPage />);
    await pageReady();

    await waitFor(() => {
      expect(chessgroundMock.state.config?.orientation).toBe('black');
    });
  });

  it('le osservazioni accese evidenziano sulla scacchiera il fatto indicato', async () => {
    serviceMock.fetchGame.mockResolvedValue(AFTER_E4);
    insightMock.fetchPositionReport.mockResolvedValue({
      fen: AFTER_E4_GAME.position.fen,
      turn: 'black',
      phase: 'opening',
      material: {
        white: { pawns: 8, knights: 2, bishops: 2, rooks: 2, queens: 1, points: 39 },
        black: { pawns: 8, knights: 2, bishops: 2, rooks: 2, queens: 1, points: 39 },
        advantage: 0,
      },
      facts: [
        {
          category: 'tactics',
          side: 'white',
          text: 'Una tattica di prova',
          squares: ['e5'],
          arrows: [{ from: 'e4', to: 'e5' }],
          moves: [],
        },
      ],
    });
    render(<PlayPage />);
    await pageReady();

    fireEvent.click(screen.getByRole('switch', { name: 'Mostra osservazioni' }));
    fireEvent.mouseEnter(await screen.findByRole('button', { name: /Una tattica di prova/ }));

    await waitFor(() => {
      expect(chessgroundMock.state.config?.drawable?.autoShapes).toEqual([
        { orig: 'e4', dest: 'e5', brush: 'blue' },
        { orig: 'e5', brush: 'blue' },
      ]);
    });
  });

  it('una nuova partita contro il computer spegne le osservazioni', async () => {
    serviceMock.fetchGame.mockResolvedValue(AFTER_E4);
    serviceMock.startNewGame.mockResolvedValue(COMPUTER_THINKING);
    useObservationsStore.setState({ enabled: true });
    insightMock.fetchPositionReport.mockReturnValue(new Promise(() => undefined));
    render(<PlayPage />);
    await pageReady();

    fireEvent.click(screen.getByRole('button', { name: /Nuova partita/ }));
    fireEvent.click(screen.getByRole('button', { name: 'Inizia la partita' }));

    await waitFor(() => {
      expect(useObservationsStore.getState().enabled).toBe(false);
    });
  });
});
