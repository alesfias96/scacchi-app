/**
 * Test del pulsante «Aggiungi un appunto». Il servizio verso Rust è simulato.
 */
import { fireEvent, screen, waitFor } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { renderWithRouter } from '../../../test/renderWithRouter';
import { AddNoteButton } from './AddNoteButton';

const notesMock = vi.hoisted(() => ({ saveNote: vi.fn() }));

vi.mock('../../../services/notesService', () => notesMock);

describe('AddNoteButton', () => {
  it('crea un appunto collegato alla posizione', async () => {
    const link = { fen: '8/8/8/8/8/8/8/K6k w - - 0 1', gameId: 'partita-1', ply: 4 };
    notesMock.saveNote.mockResolvedValue({ id: 'nota-1' });
    renderWithRouter(<AddNoteButton link={link} />);

    fireEvent.click(await screen.findByRole('button', { name: 'Aggiungi un appunto' }));

    await waitFor(() => {
      expect(notesMock.saveNote).toHaveBeenCalledWith({
        id: null,
        title: '',
        body: '',
        tags: [],
        link,
      });
    });
  });

  it('dice se non è riuscito a creare l’appunto', async () => {
    notesMock.saveNote.mockRejectedValue('database chiuso');
    renderWithRouter(<AddNoteButton link={{ fen: 'x', gameId: null, ply: null }} />);

    fireEvent.click(await screen.findByRole('button', { name: 'Aggiungi un appunto' }));

    expect((await screen.findByRole('alert')).textContent).toContain('database chiuso');
  });
});
