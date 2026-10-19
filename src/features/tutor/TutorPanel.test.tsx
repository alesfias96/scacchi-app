/**
 * Test del pannello del tutor. Il servizio verso Rust e chessground sono simulati.
 */
import type { Config } from '@lichess-org/chessground/config';
import { fireEvent, render, screen } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { useSettingsStore } from '../../state/settingsStore';
import { useTutorStore, type TutorTarget } from '../../state/tutorStore';
import { AFTER_E4_SCENARIO, START_EXPLANATION } from '../../test/tutorExplanations';
import { OPPONENT_PATHS, POSITION_IDEAS } from '../../test/ideas';
import { TutorPanel } from './TutorPanel';

const serviceMock = vi.hoisted(() => ({
  explainPosition: vi.fn(),
  tryWhatIf: vi.fn(),
  fetchPositionIdeas: vi.fn(),
  fetchOpponentPaths: vi.fn(),
  isCancelled: () => false,
}));

vi.mock('../../services/tutorService', () => serviceMock);

/** chessground simulato: ricorda l'ultima configurazione ricevuta. */
const chessgroundMock = vi.hoisted(() => ({
  state: { config: undefined as Config | undefined },
  create: vi.fn(),
}));

vi.mock('@lichess-org/chessground', () => ({ Chessground: chessgroundMock.create }));

const TARGET: TutorTarget = {
  startFen: START_EXPLANATION.fen,
  moves: [],
  orientation: 'white',
  lock: null,
  lockKey: 'partita-1',
};

describe('TutorPanel', () => {
  beforeEach(() => {
    vi.resetAllMocks();
    chessgroundMock.create.mockImplementation((_element: HTMLElement, config: Config) => {
      chessgroundMock.state.config = config;
      return {
        set: (next: Config) => {
          chessgroundMock.state.config = { ...chessgroundMock.state.config, ...next };
        },
        destroy: vi.fn(),
      };
    });
    serviceMock.explainPosition.mockResolvedValue(START_EXPLANATION);
    serviceMock.tryWhatIf.mockResolvedValue(AFTER_E4_SCENARIO);
    useTutorStore.setState({
      target: null,
      isOpen: true,
      withEngine: true,
      unlockedKey: null,
      explanation: null,
      isLoading: false,
      error: null,
      whatIf: null,
    });
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

  it('senza una posizione invita ad aprirne una', () => {
    render(<TutorPanel />);

    expect(screen.getByText(/Comincia una partita o un puzzle/)).toBeDefined();
  });

  it('contro il computer chiede di accenderlo comunque', async () => {
    useTutorStore.getState().setTarget({ ...TARGET, lock: 'computerGame' });
    render(<TutorPanel />);

    fireEvent.click(screen.getByRole('button', { name: 'Accendi comunque (è un aiuto)' }));

    expect(await screen.findByText('Tocca al Bianco')).toBeDefined();
  });

  it('«Guarda avanti» chiede a Stockfish le idee e le spiega', async () => {
    serviceMock.fetchPositionIdeas.mockResolvedValue(POSITION_IDEAS);
    serviceMock.fetchOpponentPaths.mockResolvedValue(OPPONENT_PATHS);
    useTutorStore.getState().setTarget(TARGET);
    render(<TutorPanel />);
    await screen.findByText('Tocca al Bianco');

    fireEvent.click(screen.getByRole('button', { name: 'Guarda avanti' }));

    expect(await screen.findByText('Idea 1 · la preferita da Stockfish')).toBeDefined();
    expect(serviceMock.fetchPositionIdeas).toHaveBeenCalledWith(TARGET.startFen, [], 3000);
    expect(screen.getByText(/Stockfish preferisce la prima idea/)).toBeDefined();
    expect(await screen.findByText('Le strade dell’avversario')).toBeDefined();
    expect(screen.getByText(/Una strada possibile: il cavallo va b8–c6/)).toBeDefined();
    expect(serviceMock.fetchOpponentPaths).toHaveBeenCalledWith(TARGET.startFen, [], 3000);
  });

  it('mostra «Prima di muovere» con i passi del metodo', async () => {
    useTutorStore.getState().setTarget(TARGET);
    render(<TutorPanel />);
    await screen.findByText('Tocca al Bianco');

    expect(screen.getByRole('region', { name: 'Prima di muovere' })).toBeDefined();
    expect(screen.getByText('Dxf7+')).toBeDefined();
  });

  it('durante un puzzle resta spento', () => {
    useTutorStore.getState().setTarget({ ...TARGET, lock: 'puzzle' });
    render(<TutorPanel />);

    expect(screen.getByText(/si accende a puzzle finito/)).toBeDefined();
    expect(serviceMock.explainPosition).not.toHaveBeenCalled();
  });

  it('mostra la spiegazione e apre la scheda con «Approfondisci»', async () => {
    useTutorStore.getState().setTarget(TARGET);
    render(<TutorPanel />);
    await screen.findByText('Tocca al Bianco');

    fireEvent.click(screen.getByRole('button', { name: 'Approfondisci' }));

    const card = screen.getByText('Il controllo del centro').closest('details');
    expect(card?.open).toBe(true);
    expect(screen.getByText('Piano: Chi ha più pedoni al centro (il Bianco)')).toBeDefined();
  });

  it('prova una mossa sulla sua scacchiera e dice cosa cambia', async () => {
    useTutorStore.getState().setTarget(TARGET);
    render(<TutorPanel />);
    await screen.findByText('Tocca al Bianco');

    fireEvent.click(screen.getByRole('button', { name: 'Prova una mossa' }));
    chessgroundMock.state.config?.movable?.events?.after?.('e2', 'e4', { premove: false });

    expect(await screen.findByText(/Ottima scelta/)).toBeDefined();
    expect(serviceMock.tryWhatIf).toHaveBeenCalledWith(TARGET.startFen, [], 'e2e4', true);
    expect(screen.getByText('Valutazione: da +0.3 a +0.4')).toBeDefined();

    fireEvent.click(screen.getByRole('button', { name: 'Torna alla posizione' }));
    expect(screen.getByRole('button', { name: 'Prova una mossa' })).toBeDefined();
  });
});
