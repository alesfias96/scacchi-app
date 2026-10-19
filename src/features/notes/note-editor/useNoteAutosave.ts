/**
 * useNoteAutosave — il salvataggio automatico di un appunto mentre lo si scrive.
 *
 * Cosa fa: tiene la bozza (titolo, testo, etichette) e, quando la si cambia,
 * aspetta un attimo che si smetta di scrivere e la salva. Se si chiude
 * l'appunto (o si cambia sezione) con una modifica non ancora salvata, la
 * salva subito: nulla va perso.
 *
 * Riceve:
 * - `note`: l'appunto aperto, come è salvato;
 * - `onSaved`: chiamata con l'appunto dopo ogni salvataggio riuscito.
 * Restituisce: la bozza, la funzione per cambiarla e lo stato del salvataggio.
 */
import { useEffect, useRef, useState } from 'react';
import type { Note } from '../../../bindings/Note';
import type { NoteInput } from '../../../bindings/NoteInput';
import { describeError } from '../../../lib/describeError';
import { saveNote } from '../../../services/notesService';

/** Quanto si aspetta dopo l'ultimo tasto prima di salvare (millesimi di secondo). */
export const AUTOSAVE_DELAY_MS = 700;

/** Ciò che si scrive nell'editor. */
export interface NoteDraft {
  title: string;
  body: string;
  /** Le etichette come si scrivono: separate da virgole. */
  tagsText: string;
}

/** A che punto è il salvataggio. */
export type SaveStatus =
  { kind: 'saved' } | { kind: 'saving' } | { kind: 'failed'; message: string };

/** La bozza di partenza di un appunto. */
function draftOf(note: Note): NoteDraft {
  return { title: note.title, body: note.body, tagsText: note.tags.join(', ') };
}

/** L'appunto da mandare a Rust. */
function inputOf(note: Note, draft: NoteDraft): NoteInput {
  return {
    id: note.id,
    title: draft.title,
    body: draft.body,
    tags: draft.tagsText.split(','),
    link: note.link,
  };
}

/** Un testo che cambia se cambia qualcosa della bozza (per sapere se è già salvata). */
function signature(draft: NoteDraft): string {
  return JSON.stringify(draft);
}

export function useNoteAutosave(note: Note, onSaved: (note: Note) => void) {
  const [draft, setDraft] = useState<NoteDraft>(() => draftOf(note));
  const [status, setStatus] = useState<SaveStatus>({ kind: 'saved' });
  /** La bozza salvata per ultima. */
  const savedSignature = useRef(signature(draftOf(note)));
  /** L'ultima bozza e l'appunto, letti quando l'editor si chiude. */
  const latest = useRef({ note, draft, onSaved });
  latest.current = { note, draft, onSaved };

  useEffect(() => {
    const current = signature(draft);
    if (current === savedSignature.current) {
      return;
    }
    setStatus({ kind: 'saving' });
    const timer = window.setTimeout(() => {
      saveNote(inputOf(latest.current.note, draft))
        .then((saved) => {
          savedSignature.current = current;
          setStatus({ kind: 'saved' });
          latest.current.onSaved(saved);
        })
        .catch((error: unknown) => {
          setStatus({ kind: 'failed', message: describeError(error) });
        });
    }, AUTOSAVE_DELAY_MS);
    return () => {
      window.clearTimeout(timer);
    };
  }, [draft]);

  // Chiudendo l'editor, l'ultima modifica non ancora salvata si salva subito.
  useEffect(
    () => () => {
      const { note: closing, draft: last, onSaved: notify } = latest.current;
      if (signature(last) !== savedSignature.current) {
        void saveNote(inputOf(closing, last)).then(notify);
      }
    },
    [],
  );

  return { draft, setDraft, status };
}
