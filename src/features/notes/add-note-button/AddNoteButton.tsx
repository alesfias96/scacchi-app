/**
 * AddNoteButton — «Aggiungi un appunto» sulla posizione mostrata.
 *
 * Cosa fa: crea un appunto vuoto collegato alla posizione (e, se c'è, alla
 * partita e alla mossa) e apre la sezione «Appunti» su quell'appunto, pronto
 * da scrivere. Si usa in «Gioca» e in «Analisi partite».
 *
 * Riceve: `link`, la posizione (e la partita) a cui collegare l'appunto.
 * Restituisce: il pulsante, con l'eventuale messaggio d'errore.
 */
import { useNavigate } from '@tanstack/react-router';
import { useState } from 'react';
import type { NoteLink } from '../../../bindings/NoteLink';
import { Button } from '../../../components/button/Button';
import { describeError } from '../../../lib/describeError';
import { saveNote } from '../../../services/notesService';
import styles from './AddNoteButton.module.css';

export interface AddNoteButtonProps {
  link: NoteLink;
}

export function AddNoteButton({ link }: AddNoteButtonProps) {
  const navigate = useNavigate();
  const [error, setError] = useState<string | null>(null);

  const addNote = async () => {
    try {
      const note = await saveNote({ id: null, title: '', body: '', tags: [], link });
      await navigate({ to: '/appunti', search: { appunto: note.id } });
    } catch (saveError) {
      setError(describeError(saveError));
    }
  };

  return (
    <div className={styles.add}>
      <Button
        onClick={() => {
          void addNote();
        }}
      >
        Aggiungi un appunto
      </Button>
      {error !== null && (
        <p className={styles.error} role="alert">
          {error}
        </p>
      )}
    </div>
  );
}
