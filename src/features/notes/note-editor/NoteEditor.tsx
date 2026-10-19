/**
 * NoteEditor — l'appunto aperto, da leggere e da scrivere.
 *
 * Cosa fa: il titolo, il testo e le etichette (separate da virgole) si
 * scrivono direttamente e si salvano da soli. Se l'appunto è collegato a una
 * posizione, sotto c'è la scacchiera piccola con:
 * - «Apri in Gioca»: apre quella posizione in «Gioca», per giocarla o analizzarla;
 * - «Apri la partita a questa mossa» (se l'appunto è nato da una partita
 *   dell'archivio): apre la partita in «Gioca», già su quella mossa.
 * In fondo «Elimina», che chiede conferma.
 *
 * Riceve:
 * - `note`: l'appunto;
 * - `onSaved`: chiamata dopo ogni salvataggio;
 * - `onDelete`: chiamata quando si conferma l'eliminazione.
 * Restituisce: l'editor.
 */
import { useNavigate } from '@tanstack/react-router';
import { useState } from 'react';
import type { Note } from '../../../bindings/Note';
import { Button } from '../../../components/button/Button';
import { Chessboard } from '../../../components/chessboard/Chessboard';
import { describeError } from '../../../lib/describeError';
import { useGameStore } from '../../../state/gameStore';
import { fenBoard } from '../noteLinks';
import styles from './NoteEditor.module.css';
import { useNoteAutosave, type SaveStatus } from './useNoteAutosave';

export interface NoteEditorProps {
  note: Note;
  onSaved: (note: Note) => void;
  onDelete: (id: string) => void;
}

/** La frase sullo stato del salvataggio. */
function describeStatus(status: SaveStatus): string {
  switch (status.kind) {
    case 'saved':
      return 'Salvato';
    case 'saving':
      return 'Salvataggio…';
    case 'failed':
      return `Non sono riuscito a salvare: ${status.message}`;
  }
}

export function NoteEditor({ note, onSaved, onDelete }: NoteEditorProps) {
  const navigate = useNavigate();
  const importText = useGameStore((state) => state.importText);
  const openSaved = useGameStore((state) => state.openSaved);
  const goTo = useGameStore((state) => state.goTo);
  const { draft, setDraft, status } = useNoteAutosave(note, onSaved);
  const [isConfirming, setIsConfirming] = useState(false);
  const [openError, setOpenError] = useState<string | null>(null);
  const { link } = note;

  /** Apre in «Gioca» la posizione, oppure la partita a quella mossa. */
  const open = async (inGame: boolean) => {
    if (link === null) {
      return;
    }
    try {
      if (inGame && link.gameId !== null) {
        await openSaved(link.gameId);
        await goTo(link.ply ?? 0);
      } else {
        await importText(link.fen);
      }
      await navigate({ to: '/' });
    } catch (error) {
      setOpenError(describeError(error));
    }
  };

  return (
    <article className={styles.editor} aria-label="Appunto aperto">
      <input
        className={styles.title}
        type="text"
        placeholder="Titolo"
        aria-label="Titolo"
        value={draft.title}
        onChange={(event) => {
          setDraft({ ...draft, title: event.target.value });
        }}
      />
      <textarea
        className={styles.body}
        placeholder="Scrivi qui le tue idee…"
        aria-label="Testo"
        value={draft.body}
        onChange={(event) => {
          setDraft({ ...draft, body: event.target.value });
        }}
      />
      <label className={styles.field}>
        <span className={styles.fieldLabel}>Etichette (separate da virgole)</span>
        <input
          className={styles.tags}
          type="text"
          placeholder="es. Siciliana, Finali"
          value={draft.tagsText}
          onChange={(event) => {
            setDraft({ ...draft, tagsText: event.target.value });
          }}
        />
      </label>
      <p
        className={status.kind === 'failed' ? styles.error : styles.status}
        role={status.kind === 'failed' ? 'alert' : 'status'}
      >
        {describeStatus(status)}
      </p>

      {link !== null && (
        <section className={styles.link} aria-label="Posizione collegata">
          <div className={styles.board}>
            <Chessboard {...fenBoard(link.fen)} inCheck={false} />
          </div>
          <div className={styles.linkActions}>
            <Button
              onClick={() => {
                void open(false);
              }}
            >
              Apri in Gioca
            </Button>
            {link.gameId !== null && (
              <Button
                onClick={() => {
                  void open(true);
                }}
              >
                Apri la partita a questa mossa
              </Button>
            )}
            {openError !== null && (
              <p className={styles.error} role="alert">
                {openError}
              </p>
            )}
          </div>
        </section>
      )}

      <div className={styles.actions}>
        {isConfirming ? (
          <div className={styles.confirm} role="group" aria-label="Conferma eliminazione">
            <span>Eliminare questo appunto?</span>
            <Button
              variant="primary"
              onClick={() => {
                onDelete(note.id);
              }}
            >
              Sì, elimina
            </Button>
            <Button
              onClick={() => {
                setIsConfirming(false);
              }}
            >
              No
            </Button>
          </div>
        ) : (
          <Button
            onClick={() => {
              setIsConfirming(true);
            }}
          >
            Elimina
          </Button>
        )}
      </div>
    </article>
  );
}
