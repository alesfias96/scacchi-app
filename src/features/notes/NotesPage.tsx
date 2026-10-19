/**
 * NotesPage — la sezione «Appunti»: idee, pensieri e note di studio.
 *
 * Cosa fa: a sinistra la ricerca (senza badare a maiuscole e accenti), le
 * etichette per filtrare, «Nuovo appunto» e l'elenco degli appunti dal più
 * recente; a destra l'appunto aperto, che si scrive e si salva da solo.
 * L'appunto aperto sta nell'indirizzo (`?appunto=<nome>`): così «Aggiungi un
 * appunto» in «Gioca» e in «Analisi partite» può aprire subito quello appena creato.
 *
 * Riceve: niente.
 * Restituisce: la pagina della sezione.
 */
import { useNavigate, useSearch } from '@tanstack/react-router';
import { useCallback, useMemo, useState } from 'react';
import type { Note } from '../../bindings/Note';
import type { NoteQuery } from '../../bindings/NoteQuery';
import { Button } from '../../components/button/Button';
import { classNames } from '../../lib/classNames';
import { describeError } from '../../lib/describeError';
import { deleteNote, saveNote } from '../../services/notesService';
import { NoteEditor } from './note-editor/NoteEditor';
import { NoteList } from './note-list/NoteList';
import styles from './NotesPage.module.css';
import { useNotes } from './useNotes';

export function NotesPage() {
  const navigate = useNavigate();
  const { appunto } = useSearch({ strict: false });
  const [text, setText] = useState('');
  const [tag, setTag] = useState<string | null>(null);
  const [actionError, setActionError] = useState<string | null>(null);
  const query = useMemo<NoteQuery>(() => ({ text: text === '' ? null : text, tag }), [text, tag]);
  const { notes, tags, isLoading, error, reload } = useNotes(query);
  const selectedId = appunto ?? null;
  /** L'ultima versione salvata dell'appunto aperto: resta aperto anche se esce dalla ricerca. */
  const [kept, setKept] = useState<Note | null>(null);
  const selected =
    notes.find((note) => note.id === selectedId) ?? (kept?.id === selectedId ? kept : undefined);

  /** Dopo ogni salvataggio: si ricorda l'appunto e si rilegge l'elenco. */
  const handleSaved = useCallback(
    (note: Note) => {
      setKept(note);
      reload();
    },
    [reload],
  );

  /** Apre un appunto (o nessuno) cambiando l'indirizzo. */
  const select = useCallback(
    (id: string | null) => {
      void navigate({ to: '/appunti', search: id === null ? {} : { appunto: id } });
    },
    [navigate],
  );

  const createNote = async () => {
    try {
      const note = await saveNote({ id: null, title: '', body: '', tags: [], link: null });
      setText('');
      setTag(null);
      setKept(note);
      select(note.id);
      reload();
    } catch (createError) {
      setActionError(describeError(createError));
    }
  };

  const removeNote = async (id: string) => {
    try {
      await deleteNote(id);
      select(null);
      reload();
    } catch (deleteError) {
      setActionError(describeError(deleteError));
    }
  };

  return (
    <section className={styles.page}>
      <header className={styles.header}>
        <h1 className={styles.title}>Appunti</h1>
        <p className={styles.subtitle}>
          Le tue idee, i tuoi pensieri e le note di studio. Si salvano da soli mentre scrivi.
        </p>
      </header>

      {(error ?? actionError) !== null && (
        <p className={styles.error} role="alert">
          {error ?? actionError}
        </p>
      )}

      <div className={styles.layout}>
        <div className={styles.listColumn}>
          <Button
            variant="primary"
            onClick={() => {
              void createNote();
            }}
          >
            Nuovo appunto
          </Button>
          <input
            className={styles.search}
            type="search"
            placeholder="Cerca negli appunti"
            aria-label="Cerca negli appunti"
            value={text}
            onChange={(event) => {
              setText(event.target.value);
            }}
          />
          {tags.length > 0 && (
            <div className={styles.tags} role="group" aria-label="Etichette">
              {[null, ...tags].map((option) => (
                <button
                  key={option ?? 'tutte'}
                  type="button"
                  className={classNames(styles.tag, option === tag && styles.tagActive)}
                  aria-pressed={option === tag}
                  onClick={() => {
                    setTag(option);
                  }}
                >
                  {option ?? 'Tutte'}
                </button>
              ))}
            </div>
          )}
          {!isLoading && notes.length === 0 && (
            <p className={styles.empty}>
              {text === '' && tag === null
                ? 'Non hai ancora nessun appunto. Premi «Nuovo appunto», oppure «Aggiungi un appunto» in Gioca.'
                : 'Nessun appunto corrisponde alla ricerca.'}
            </p>
          )}
          <NoteList notes={notes} selectedId={selectedId} onSelect={select} />
        </div>

        <div className={styles.editorColumn}>
          {selected === undefined ? (
            <p className={styles.placeholder}>Scegli un appunto dall’elenco, o creane uno nuovo.</p>
          ) : (
            <NoteEditor
              key={selected.id}
              note={selected}
              onSaved={handleSaved}
              onDelete={(id) => {
                void removeNote(id);
              }}
            />
          )}
        </div>
      </div>
    </section>
  );
}
