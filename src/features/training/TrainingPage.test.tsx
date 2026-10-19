/**
 * Test della sezione "Allenamento". I servizi verso Rust e chessground sono simulati.
 */
import type { Config } from '@lichess-org/chessground/config';
import type { Key } from '@lichess-org/chessground/types';
import { fireEvent, render, screen, waitFor } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import type { TrainingOverview } from '../../bindings/TrainingOverview';
import { useSettingsStore } from '../../state/settingsStore';
import { useTrainingStore } from '../../state/trainingStore';
import { READY_ATTEMPT, SOLVED_ATTEMPT } from '../../test/puzzleAttempts';
import { TrainingPage } from './TrainingPage';

const trainingMock = vi.hoisted(() => ({
  fetchTrainingOverview: vi.fn(),
  startPuzzle: vi.fn(),
  playPuzzleMove: vi.fn(),
  requestPuzzleHint: vi.fn(),
  revealPuzzleSolution: vi.fn(),
}));

vi.mock('../../services/trainingService', () => trainingMock);

vi.mock('../../services/positionService', () => ({
  fetchInitialPosition: () =>
    Promise.resolve({
      fen: 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1',
      boardFen: 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR',
      turn: 'white',
      inCheck: false,
      legalMoves: [],
    }),
}));

vi.mock('../../services/settingsService', () => ({
  fetchSettings: vi.fn(),
  saveSettings: () => Promise.resolve(),
}));

/** chessground simulato: ricorda l'ultima configurazione ricevuta. */
const chessgroundMock = vi.hoisted(() => ({
  state: { config: undefined as Config | undefined },
  create: vi.fn(),
}));

vi.mock('@lichess-org/chessground', () => ({ Chessground: chessgroundMock.create }));

/** Simula l'utente che sposta un pezzo sulla scacchiera. */
function dragPiece(from: Key, to: Key) {
  chessgroundMock.state.config?.movable?.events?.after?.(from, to, { premove: false });
}

const OVERVIEW: TrainingOverview = {
  rating: 1500,
  attempted: 3,
  solved: 2,
  dueReviews: 1,
  themes: [{ key: 'fork', name: 'Forchetta', count: 1234 }],
};

describe('TrainingPage', () => {
  beforeEach(() => {
    vi.resetAllMocks();
    chessgroundMock.state.config = undefined;
    chessgroundMock.create.mockImplementation((_element: HTMLElement, config: Config) => {
      chessgroundMock.state.config = config;
      return {
        set: (next: Config) => {
          chessgroundMock.state.config = { ...chessgroundMock.state.config, ...next };
        },
        destroy: vi.fn(),
      };
    });
    trainingMock.fetchTrainingOverview.mockResolvedValue(OVERVIEW);
    useTrainingStore.setState({ overview: null, attempt: null, error: null });
    useSettingsStore.setState({
      settings: {
        theme: 'studio',
        board: 'rovere',
        pieces: 'icona',
        notation: 'italiana',
        sounds: 'off',
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
          depth: 1,
          theme: null,
          difficulty: 'normal',
          visualizationSeconds: 5,
        },
        statsFilter: { period: 'all', opponent: 'all' },
      },
    });
  });

  it('mostra il proprio livello e invita a cominciare', async () => {
    render(<TrainingPage />);

    expect(await screen.findByText('2 su 3')).toBeDefined();
    expect(screen.getByText(/premi «Nuovo puzzle»/)).toBeDefined();
  });

  it('comincia un puzzle con le scelte fatte', async () => {
    trainingMock.startPuzzle.mockResolvedValue(READY_ATTEMPT);
    render(<TrainingPage />);

    fireEvent.click(screen.getByRole('button', { name: 'Nuovo puzzle' }));

    expect(await screen.findByText(/Muove il Bianco/)).toBeDefined();
    expect(trainingMock.startPuzzle).toHaveBeenCalledWith(
      expect.objectContaining({ mode: 'depth', depth: 1 }),
    );
    expect(screen.getByLabelText('Mosse giocate').textContent).toBe('3… h6');
  });

  it('gioca la mossa e, a puzzle risolto, mostra soluzione e rating', async () => {
    trainingMock.startPuzzle.mockResolvedValue(READY_ATTEMPT);
    trainingMock.playPuzzleMove.mockResolvedValue(SOLVED_ATTEMPT);
    render(<TrainingPage />);
    fireEvent.click(screen.getByRole('button', { name: 'Nuovo puzzle' }));
    await screen.findByText(/Muove il Bianco/);

    dragPiece('f3', 'f7');

    expect(await screen.findByText('Risolto!')).toBeDefined();
    expect(trainingMock.playPuzzleMove).toHaveBeenCalledWith('f3f7');
    expect(screen.getByText('(+16)')).toBeDefined();
    expect(screen.getByText('Matto in 1')).toBeDefined();
    expect(screen.getByText('4. Dxf7#')).toBeDefined();
    expect(screen.getByRole('button', { name: 'Prossimo puzzle' })).toBeDefined();
  });

  it('chiede un aiuto e cerchia il pezzo da muovere', async () => {
    trainingMock.startPuzzle.mockResolvedValue(READY_ATTEMPT);
    trainingMock.requestPuzzleHint.mockResolvedValue({ ...READY_ATTEMPT, hintSquare: 'f3' });
    render(<TrainingPage />);
    fireEvent.click(screen.getByRole('button', { name: 'Nuovo puzzle' }));
    await screen.findByText(/Muove il Bianco/);

    fireEvent.click(screen.getByRole('button', { name: 'Aiuto' }));

    await waitFor(() => {
      expect(chessgroundMock.state.config?.drawable?.autoShapes).toEqual([
        { orig: 'f3', brush: 'blue' },
      ]);
    });
  });

  it('spiega perché non trova un puzzle', async () => {
    trainingMock.startPuzzle.mockRejectedValue('nessun puzzle con queste scelte');
    render(<TrainingPage />);

    fireEvent.click(screen.getByRole('button', { name: 'Nuovo puzzle' }));

    expect((await screen.findByRole('alert')).textContent).toContain('nessun puzzle');
  });
});
