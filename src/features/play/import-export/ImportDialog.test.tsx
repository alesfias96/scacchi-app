/**
 * Test della finestra "Incolla posizione o partita".
 */
import { fireEvent, render, screen, waitFor } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { ImportDialog } from './ImportDialog';

const PGN = '1. e4 e5 *';

describe('ImportDialog', () => {
  it('carica il testo incollato e si chiude', async () => {
    const onImport = vi.fn(() => Promise.resolve());
    const onClose = vi.fn();
    render(<ImportDialog onImport={onImport} onClose={onClose} />);

    fireEvent.change(screen.getByRole('textbox'), { target: { value: PGN } });
    fireEvent.click(screen.getByRole('button', { name: 'Carica' }));

    await waitFor(() => {
      expect(onClose).toHaveBeenCalled();
    });
    expect(onImport).toHaveBeenCalledWith(PGN);
  });

  it('se il testo non è valido spiega il motivo e resta aperta', async () => {
    const onImport = vi.fn(() => Promise.reject(new Error('Mossa non valida nel PGN: banana')));
    const onClose = vi.fn();
    render(<ImportDialog onImport={onImport} onClose={onClose} />);

    fireEvent.change(screen.getByRole('textbox'), { target: { value: '1. banana' } });
    fireEvent.click(screen.getByRole('button', { name: 'Carica' }));

    expect((await screen.findByRole('alert')).textContent).toContain('banana');
    expect(onClose).not.toHaveBeenCalled();
  });

  it('"Carica" è disattivato finché non c’è testo', () => {
    render(<ImportDialog onImport={vi.fn()} onClose={vi.fn()} />);

    expect(screen.getByRole<HTMLButtonElement>('button', { name: 'Carica' }).disabled).toBe(true);
  });

  it('Esc e "Annulla" chiudono senza caricare', () => {
    const onImport = vi.fn();
    const onClose = vi.fn();
    render(<ImportDialog onImport={onImport} onClose={onClose} />);

    fireEvent.keyDown(window, { key: 'Escape' });
    fireEvent.click(screen.getByRole('button', { name: 'Annulla' }));

    expect(onClose).toHaveBeenCalledTimes(2);
    expect(onImport).not.toHaveBeenCalled();
  });
});
