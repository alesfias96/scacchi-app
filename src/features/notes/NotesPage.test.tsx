/**
 * Test della sezione «Appunti». Il servizio verso Rust è simulato.
 */
import { act, fireEvent, screen, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import type { Note } from '../../bindings/Note';
import type { NoteInput } from '../../bindings/NoteInput';
import { renderWithRouter } from '../../test/renderWithRouter';
import { AUTOSAVE_DELAY_MS } from './note-editor/useNoteAutosave';
import { NotesPage } from './NotesPage';

const notesMock = vi.hoisted(() => ({
  listNotes: vi.fn(),
  fetchNoteTags: vi.fn(),
  saveNote: vi.fn(),
  deleteNote: vi.fn(),
}));

vi.mock('../../services/notesService', () => notesMock);

// La scacchiera piccola della posizione collegata non serve qui.
vi.mock('../../components/chessboard/Chessboard', () => ({ Chessboard: () => null }));

const LINKED: Note = {
  id: 'nota-1',
  title: 'Idea per la Siciliana',
  body: 'Spingere f5',
  tags: ['Siciliana'],
  link: { fen: '8/8/8/8/8/8/8/K6k w - - 0 1', gameId: 'partita-1', ply: 12 },
  createdAtMs: 1,
  updatedAtMs: 2,
};

const PLAIN: Note = {
  id: 'nota-2',
  title: '',
  body: 'Ripassare i finali di torre',
  tags: [],
  link: null,
  createdAtMs: 1,
  updatedAtMs: 1,
};

/** Un appunto salvato a partire da quello scritto. */
function saved(input: NoteInput): Note {
  return {
    id: input.id ?? 'nota-nuova',
    title: input.title,
    body: input.body,
    tags: input.tags.map((tag) => tag.trim()).filter((tag) => tag !== ''),
    link: input.link,
    createdAtMs: 1,
    updatedAtMs: 3,
  };
}

describe('NotesPage', () => {
  beforeEach(() => {
    vi.resetAllMocks();
    notesMock.listNotes.mockResolvedValue([LINKED, PLAIN]);
    notesMock.fetchNoteTags.mockResolvedValue(['Siciliana']);
    notesMock.saveNote.mockImplementation((input: NoteInput) => Promise.resolve(saved(input)));
    notesMock.deleteNote.mockResolvedValue(undefined);
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it('mostra gli appunti e usa l’inizio del testo se manca il titolo', async () => {
    renderWithRouter(<NotesPage />, '/appunti');

    expect(await screen.findByText('Idea per la Siciliana')).toBeDefined();
    expect(screen.getByText('Ripassare i finali di torre')).toBeDefined();
    expect(screen.getByLabelText('Collegato a una posizione')).toBeDefined();
  });

  it('cerca con le parole scritte e filtra per etichetta', async () => {
    renderWithRouter(<NotesPage />, '/appunti');
    await screen.findByText('Idea per la Siciliana');

    fireEvent.change(screen.getByRole('searchbox', { name: 'Cerca negli appunti' }), {
      target: { value: 'finali' },
    });
    await waitFor(() => {
      expect(notesMock.listNotes).toHaveBeenLastCalledWith({ text: 'finali', tag: null });
    });

    fireEvent.click(screen.getByRole('button', { name: 'Siciliana' }));
    await waitFor(() => {
      expect(notesMock.listNotes).toHaveBeenLastCalledWith({ text: 'finali', tag: 'Siciliana' });
    });
  });

  it('apre l’appunto dell’indirizzo con la sua posizione collegata', async () => {
    renderWithRouter(<NotesPage />, '/appunti?appunto=nota-1');

    expect(await screen.findByRole('textbox', { name: 'Titolo' })).toBeDefined();
    expect(screen.getByRole('button', { name: 'Apri in Gioca' })).toBeDefined();
    expect(screen.getByRole('button', { name: 'Apri la partita a questa mossa' })).toBeDefined();
  });

  it('salva da solo mentre si scrive', async () => {
    renderWithRouter(<NotesPage />, '/appunti?appunto=nota-2');
    const body = await screen.findByRole('textbox', { name: 'Testo' });
    vi.useFakeTimers();

    fireEvent.change(body, { target: { value: 'Ripassare Lucena' } });
    expect(notesMock.saveNote).not.toHaveBeenCalled();
    await act(async () => {
      await vi.advanceTimersByTimeAsync(AUTOSAVE_DELAY_MS + 50);
    });

    expect(notesMock.saveNote).toHaveBeenCalledWith(
      expect.objectContaining({ id: 'nota-2', body: 'Ripassare Lucena', link: null }),
    );
  });

  it('crea un appunto nuovo e ne elimina uno dopo la conferma', async () => {
    renderWithRouter(<NotesPage />, '/appunti?appunto=nota-2');
    await screen.findByRole('textbox', { name: 'Testo' });

    fireEvent.click(screen.getByRole('button', { name: 'Elimina' }));
    fireEvent.click(screen.getByRole('button', { name: 'Sì, elimina' }));
    await waitFor(() => {
      expect(notesMock.deleteNote).toHaveBeenCalledWith('nota-2');
    });

    fireEvent.click(screen.getByRole('button', { name: 'Nuovo appunto' }));
    await waitFor(() => {
      expect(notesMock.saveNote).toHaveBeenCalledWith(
        expect.objectContaining({ id: null, body: '', link: null }),
      );
    });
  });
});
