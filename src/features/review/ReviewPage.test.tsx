/**
 * Test della sezione «Analisi partite». I servizi verso Rust e chessground sono simulati.
 */
import type { Config } from '@lichess-org/chessground/config';
import { fireEvent, screen, waitFor } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { useReviewStore } from '../../state/reviewStore';
import { useSettingsStore } from '../../state/settingsStore';
import { SAMPLE_REVIEW } from '../../test/gameReviews';
import { renderWithRouter } from '../../test/renderWithRouter';
import { ReviewPage } from './ReviewPage';

const reviewMock = vi.hoisted(() => ({
  startReview: vi.fn(),
  cancelReview: vi.fn(),
  fetchRunningReview: vi.fn(),
  fetchReview: vi.fn(),
  listenToReviewProgress: vi.fn(),
  listenToReviewFinished: vi.fn(),
}));

vi.mock('../../services/reviewService', () => reviewMock);

// La coda «Analizza le partite mancanti» non serve qui: si ascolta e basta.
vi.mock('../../services/statsService', () => ({
  analyzeMissingGames: vi.fn(),
  listenToReviewQueue: () => Promise.resolve(() => undefined),
}));

/** chessground simulato: ricorda l'ultima configurazione ricevuta. */
const chessgroundMock = vi.hoisted(() => ({
  state: { config: undefined as Config | undefined },
  create: vi.fn(),
}));

vi.mock('@lichess-org/chessground', () => ({ Chessground: chessgroundMock.create }));

describe('ReviewPage', () => {
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
    reviewMock.listenToReviewProgress.mockResolvedValue(() => undefined);
    reviewMock.listenToReviewFinished.mockResolvedValue(() => undefined);
    reviewMock.fetchRunningReview.mockResolvedValue(null);
    reviewMock.startReview.mockResolvedValue(undefined);
    useReviewStore.setState({ runningId: null, progress: null, lastFinished: null, error: null });
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

  it('senza una partita invita a sceglierla nell’Archivio', async () => {
    renderWithRouter(<ReviewPage />, '/analisi');

    expect(await screen.findByText(/Scegli una partita/)).toBeDefined();
  });

  it('propone di analizzare una partita non ancora analizzata', async () => {
    reviewMock.fetchReview.mockResolvedValue(null);
    renderWithRouter(<ReviewPage />, '/analisi?partita=p1');

    fireEvent.click(await screen.findByRole('button', { name: 'Analizza la partita' }));

    await waitFor(() => {
      expect(reviewMock.startReview).toHaveBeenCalledWith('p1');
    });
    expect(await screen.findByRole('button', { name: 'Ferma l’analisi' })).toBeDefined();
  });

  it('mostra riepilogo, momenti critici e mosse con i simboli', async () => {
    reviewMock.fetchReview.mockResolvedValue(SAMPLE_REVIEW);
    renderWithRouter(<ReviewPage />, '/analisi?partita=p1');

    expect(await screen.findByText('Momenti critici')).toBeDefined();
    expect(screen.getByText('60%')).toBeDefined();
    expect(screen.getByRole('button', { name: 'Cf6??' })).toBeDefined();
    expect(screen.getByText('Era meglio: Cc6')).toBeDefined();
  });

  it('un momento critico mostra la posizione prima dell’errore con le due frecce', async () => {
    reviewMock.fetchReview.mockResolvedValue(SAMPLE_REVIEW);
    renderWithRouter(<ReviewPage />, '/analisi?partita=p1');

    fireEvent.click(await screen.findByRole('button', { name: 'Mostra sulla scacchiera' }));

    await waitFor(() => {
      expect(chessgroundMock.state.config?.fen).toBe(SAMPLE_REVIEW.positions[3]?.boardFen);
    });
    expect(chessgroundMock.state.config?.drawable?.autoShapes).toEqual([
      { orig: 'b8', dest: 'c6', brush: 'paleGreen' },
      { orig: 'g8', dest: 'f6', brush: 'blue' },
    ]);
  });
});
