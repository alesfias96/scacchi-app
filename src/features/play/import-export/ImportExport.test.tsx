/**
 * Test di copia e incolla di posizioni e partite.
 */
import { fireEvent, render, screen } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { ImportExport } from './ImportExport';

/** Gli appunti simulati (la copia vera la fa Rust). */
const clipboardMock = vi.hoisted(() => ({ copyText: vi.fn<(text: string) => Promise<void>>() }));

vi.mock('../../../services/clipboardService', () => clipboardMock);

const FEN = 'rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1';

describe('ImportExport', () => {
  const writeText = clipboardMock.copyText;

  beforeEach(() => {
    writeText.mockReset();
    writeText.mockResolvedValue(undefined);
  });

  it('copia la FEN della posizione mostrata e lo conferma', async () => {
    render(<ImportExport fen={FEN} onRequestPgn={vi.fn()} onImport={vi.fn()} />);

    fireEvent.click(screen.getByRole('button', { name: 'Copia FEN' }));

    expect(await screen.findByText('Posizione (FEN) copiata')).toBeDefined();
    expect(writeText).toHaveBeenCalledWith(FEN);
  });

  it('copia il PGN della partita', async () => {
    const onRequestPgn = vi.fn(() => Promise.resolve('1. e4 *'));
    render(<ImportExport fen={FEN} onRequestPgn={onRequestPgn} onImport={vi.fn()} />);

    fireEvent.click(screen.getByRole('button', { name: 'Copia PGN' }));

    expect(await screen.findByText('Partita (PGN) copiata')).toBeDefined();
    expect(writeText).toHaveBeenCalledWith('1. e4 *');
  });

  it('spiega il motivo se la copia non riesce', async () => {
    writeText.mockImplementationOnce(() => Promise.reject(new Error('permesso negato')));
    render(<ImportExport fen={FEN} onRequestPgn={vi.fn()} onImport={vi.fn()} />);

    fireEvent.click(screen.getByRole('button', { name: 'Copia FEN' }));

    expect(await screen.findByText(/permesso negato/)).toBeDefined();
  });

  it('"Incolla" apre la finestra di caricamento', () => {
    render(<ImportExport fen={FEN} onRequestPgn={vi.fn()} onImport={vi.fn()} />);

    fireEvent.click(screen.getByRole('button', { name: 'Incolla' }));

    expect(screen.getByRole('dialog', { name: 'Incolla posizione o partita' })).toBeDefined();
  });
});
