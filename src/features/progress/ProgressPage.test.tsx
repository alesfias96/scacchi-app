/**
 * Test della sezione «Progressi». I servizi verso Rust sono simulati.
 */
import { fireEvent, render, screen, waitFor } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import type { StatsReport } from '../../bindings/StatsReport';
import { useReviewStore } from '../../state/reviewStore';
import { useSettingsStore } from '../../state/settingsStore';
import { ProgressPage } from './ProgressPage';

const statsMock = vi.hoisted(() => ({
  fetchStats: vi.fn(),
  analyzeMissingGames: vi.fn(),
  listenToReviewQueue: vi.fn(),
}));

vi.mock('../../services/statsService', () => statsMock);

vi.mock('../../services/reviewService', () => ({
  cancelReview: vi.fn(() => Promise.resolve()),
  fetchRunningReview: () => Promise.resolve(null),
  listenToReviewFinished: () => Promise.resolve(() => undefined),
  listenToReviewProgress: () => Promise.resolve(() => undefined),
  startReview: vi.fn(),
}));

const settingsServiceMock = vi.hoisted(() => ({
  fetchSettings: vi.fn(),
  saveSettings: vi.fn(),
}));

vi.mock('../../services/settingsService', () => settingsServiceMock);

const NO_SCORE = { wins: 0, draws: 0, losses: 0 };

const REPORT: StatsReport = {
  games: 7,
  analyzedGames: 4,
  unanalyzedGames: 2,
  level: {
    estimatedElo: 1450,
    eloGames: 6,
    puzzleRating: 1520,
    puzzleHistory: [],
    accuracy: 81,
    accuracyHistory: [],
  },
  results: {
    againstComputer: { wins: 3, draws: 1, losses: 2 },
    twoPlayerGames: 1,
    byColor: [{ side: 'white', score: { wins: 2, draws: 0, losses: 1 } }],
    byLevel: [{ level: 4, name: 'Circolo', elo: 1400, score: { wins: 3, draws: 1, losses: 2 } }],
    phases: [],
    timePressure: null,
  },
  openings: [{ name: 'Italian Game', side: 'white', score: NO_SCORE, games: 2, accuracy: null }],
  tactics: {
    attempts: 10,
    solved: 7,
    byDepth: [],
    depthHistory: [],
    weakestThemes: [],
    dueReviews: 3,
  },
};

describe('ProgressPage', () => {
  beforeEach(() => {
    vi.resetAllMocks();
    statsMock.fetchStats.mockResolvedValue(REPORT);
    statsMock.analyzeMissingGames.mockResolvedValue(2);
    statsMock.listenToReviewQueue.mockResolvedValue(() => undefined);
    settingsServiceMock.saveSettings.mockResolvedValue(undefined);
    useReviewStore.setState({ queue: null, error: null, lastFinished: null });
    useSettingsStore.setState({
      settings: {
        theme: 'studio',
        board: 'rovere',
        pieces: 'icona',
        notation: 'figurine',
        sounds: 'on',
        gameSetup: { opponent: 'computer', color: 'white', level: 4, timeControl: null },
        trainingSetup: {
          mode: 'depth',
          depth: 2,
          theme: null,
          difficulty: 'normal',
          visualizationSeconds: 5,
        },
        statsFilter: { period: 'all', opponent: 'all' },
      },
      saveError: null,
    });
  });

  it('mostra il livello e su quante partite si basa', async () => {
    render(<ProgressPage />);

    expect(await screen.findByText('1450')).toBeDefined();
    expect(screen.getByText(/Basate su 7 partite \(4 analizzate\)/)).toBeDefined();
    expect(screen.getByText('81%')).toBeDefined();
  });

  it('passa da una scheda all’altra', async () => {
    render(<ProgressPage />);
    await screen.findByText('1450');

    fireEvent.click(screen.getByRole('tab', { name: 'Partite' }));
    expect(screen.getByText('Circolo (1400)')).toBeDefined();

    fireEvent.click(screen.getByRole('tab', { name: 'Aperture' }));
    expect(screen.getByText('Italian Game')).toBeDefined();

    fireEvent.click(screen.getByRole('tab', { name: 'Calcolo e tattica' }));
    expect(screen.getByText('70%')).toBeDefined();
  });

  it('salva il filtro scelto e rilegge le statistiche', async () => {
    render(<ProgressPage />);
    await screen.findByText('1450');

    fireEvent.click(screen.getByRole('radio', { name: 'Ultimi 30 giorni' }));

    await waitFor(() => {
      expect(statsMock.fetchStats).toHaveBeenLastCalledWith({ period: 'month', opponent: 'all' });
    });
    expect(settingsServiceMock.saveSettings).toHaveBeenCalledWith(
      expect.objectContaining({ statsFilter: { period: 'month', opponent: 'all' } }),
    );
  });

  it('mette in coda le partite da analizzare e mostra l’avanzamento', async () => {
    render(<ProgressPage />);

    fireEvent.click(await screen.findByRole('button', { name: /Analizza le partite mancanti/ }));

    expect(await screen.findByText(/Analisi in corso: partita 1 di 2/)).toBeDefined();
    expect(statsMock.analyzeMissingGames).toHaveBeenCalled();
    expect(screen.getByRole('button', { name: 'Ferma' })).toBeDefined();
  });
});
