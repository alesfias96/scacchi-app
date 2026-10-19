/**
 * NoteList — l'elenco degli appunti, nella colonna di sinistra della sezione.
 *
 * Cosa fa: per ogni appunto mostra il nome (il titolo, oppure l'inizio del
 * testo), quando è stato cambiato l'ultima volta, le etichette e, se è
 * collegato a una posizione, il segno ♟. Un clic lo apre; quello aperto è
 * evidenziato.
 *
 * Riceve:
 * - `notes`: gli appunti da mostrare;
 * - `selectedId`: l'appunto aperto, oppure `null`;
 * - `onSelect`: chiamata con il nome unico dell'appunto scelto.
 * Restituisce: l'elenco.
 */
import type { Note } from '../../../bindings/Note';
import { classNames } from '../../../lib/classNames';
import { formatDateTime } from '../../../lib/formatDateTime';
import { noteLabel } from '../noteLinks';
import styles from './NoteList.module.css';

export interface NoteListProps {
  notes: readonly Note[];
  selectedId: string | null;
  onSelect: (id: string) => void;
}

export function NoteList({ notes, selectedId, onSelect }: NoteListProps) {
  return (
    <ul className={styles.list} aria-label="Appunti">
      {notes.map((note) => (
        <li key={note.id}>
          <button
            type="button"
            className={classNames(styles.item, note.id === selectedId && styles.selected)}
            aria-current={note.id === selectedId ? 'true' : undefined}
            onClick={() => {
              onSelect(note.id);
            }}
          >
            <span className={styles.label}>
              {note.link !== null && (
                <span className={styles.linked} aria-label="Collegato a una posizione">
                  ♟
                </span>
              )}
              {noteLabel(note.title, note.body)}
            </span>
            <span className={styles.meta}>
              {formatDateTime(note.updatedAtMs)}
              {note.tags.length > 0 && ` · ${note.tags.join(', ')}`}
            </span>
          </button>
        </li>
      ))}
    </ul>
  );
}
