/**
 * Test dei pulsanti per salvare e aprire file `.pgn`.
 */
import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { PgnFileButtons } from './PgnFileButtons';

describe('PgnFileButtons', () => {
  it('salva la partita e lo conferma', async () => {
    const onSave = vi.fn(() => Promise.resolve(true));
    render(<PgnFileButtons onSave={onSave} onOpen={vi.fn()} />);

    fireEvent.click(screen.getByRole('button', { name: /Salva file/ }));

    expect(await screen.findByText('Partita salvata nel file')).toBeDefined();
  });

  it('se si annulla non conferma nulla', async () => {
    const onSave = vi.fn(() => Promise.resolve(false));
    render(<PgnFileButtons onSave={onSave} onOpen={vi.fn()} />);

    fireEvent.click(screen.getByRole('button', { name: /Salva file/ }));
    await Promise.resolve();

    expect(screen.getByRole('status').textContent).toBe('');
  });

  it('spiega il motivo se il salvataggio non riesce', async () => {
    const onSave = vi.fn(() => Promise.reject(new Error('disco pieno')));
    render(<PgnFileButtons onSave={onSave} onOpen={vi.fn()} />);

    fireEvent.click(screen.getByRole('button', { name: /Salva file/ }));

    expect(await screen.findByText('Salvataggio non riuscito: disco pieno')).toBeDefined();
  });

  it('apre un file', () => {
    const onOpen = vi.fn(() => Promise.resolve());
    render(<PgnFileButtons onSave={vi.fn()} onOpen={onOpen} />);

    fireEvent.click(screen.getByRole('button', { name: /Apri file/ }));

    expect(onOpen).toHaveBeenCalled();
  });
});
