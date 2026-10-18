/**
 * Test della sezione "Archivio". I servizi verso Rust sono simulati.
 */
import { fireEvent, screen, waitFor } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import type { GameSummary } from '../../bindings/GameSummary';
import { renderWithRouter } from '../../test/renderWithRouter';
import { ArchivePage } from './ArchivePage';

const archiveMock = vi.hoisted(() => ({
  listGames: vi.fn(),
  openSavedGame: vi.fn(),
  deleteSavedGame: vi.fn(),
  listenToStorageErrors: vi.fn(),
}));

vi.mock('../../services/archiveService', () => archiveMock);

vi.mock('../../services/engineService', () => ({
  fetchEngineLevels: () => Promise.resolve([{ level: 4, name: 'Circolo', elo: 1400 }]),
}));

const GAME: GameSummary = {
  id: 'partita-1',
  startedAtMs: new Date(2026, 9, 18, 16, 35).getTime(),
  opponent: 'computer',
  computerSide: 'black',
  level: 4,
  timeControl: null,
  result: null,
  opening: { eco: 'C50', name: 'Italian Game' },
  plyCount: 12,
};

describe('ArchivePage', () => {
  beforeEach(() => {
    vi.resetAllMocks();
    archiveMock.deleteSavedGame.mockResolvedValue(undefined);
  });

  it('mostra le partite salvate', async () => {
    archiveMock.listGames.mockResolvedValue([GAME]);

    renderWithRouter(<ArchivePage />, '/archivio');

    expect(await screen.findByText('Italian Game')).toBeDefined();
    expect(screen.getByText('1 partita')).toBeDefined();
    expect(screen.getByText(/Stockfish · Circolo/)).toBeDefined();
  });

  it('invita a giocare se l’archivio è vuoto', async () => {
    archiveMock.listGames.mockResolvedValue([]);

    renderWithRouter(<ArchivePage />, '/archivio');

    expect(await screen.findByText(/Non hai ancora giocato nessuna partita/)).toBeDefined();
  });

  it('chiede di nuovo l’elenco quando cambia un filtro', async () => {
    archiveMock.listGames.mockResolvedValue([GAME]);
    renderWithRouter(<ArchivePage />, '/archivio');
    await screen.findByText('Italian Game');

    fireEvent.click(screen.getByRole('radio', { name: 'In corso' }));

    await waitFor(() => {
      expect(archiveMock.listGames).toHaveBeenLastCalledWith({
        opponent: null,
        outcome: 'unfinished',
        text: null,
      });
    });
  });

  it('elimina una partita e ricarica l’elenco', async () => {
    archiveMock.listGames.mockResolvedValueOnce([GAME]).mockResolvedValue([]);
    renderWithRouter(<ArchivePage />, '/archivio');
    await screen.findByText('Italian Game');

    fireEvent.click(screen.getByRole('button', { name: 'Elimina' }));
    fireEvent.click(screen.getByRole('button', { name: 'Sì, elimina' }));

    expect(archiveMock.deleteSavedGame).toHaveBeenCalledWith('partita-1');
    expect(await screen.findByText(/Nessuna partita|Non hai ancora/)).toBeDefined();
  });

  it('spiega il problema se l’archivio non si legge', async () => {
    archiveMock.listGames.mockRejectedValue('database rovinato');

    renderWithRouter(<ArchivePage />, '/archivio');

    expect((await screen.findByRole('alert')).textContent).toContain('database rovinato');
  });
});
