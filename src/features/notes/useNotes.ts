/**
 * useNotes — gli appunti che corrispondono alla ricerca, e tutte le etichette.
 *
 * Cosa fa: chiede a Rust l'elenco ogni volta che la ricerca cambia (conta
 * solo l'ultima risposta, se ne arrivano fuori ordine) e, insieme, l'elenco
 * delle etichette usate.
 *
 * Riceve:
 * - `query`: le parole da cercare e l'etichetta scelta.
 * Restituisce: gli appunti, le etichette, se si sta caricando, l'eventuale
 * errore e `reload`, per rileggere tutto (ad esempio dopo un salvataggio).
 */
import { useCallback, useEffect, useState } from 'react';
import type { Note } from '../../bindings/Note';
import type { NoteQuery } from '../../bindings/NoteQuery';
import { describeError } from '../../lib/describeError';
import { fetchNoteTags, listNotes } from '../../services/notesService';

/** Ciò che restituisce l'hook. */
export interface NotesList {
  /** Gli appunti trovati, dal più recente. */
  notes: Note[];
  /** Tutte le etichette usate, in ordine alfabetico. */
  tags: string[];
  /** `true` finché non arriva la prima risposta. */
  isLoading: boolean;
  /** Il motivo per cui l'elenco non si è potuto leggere, oppure `null`. */
  error: string | null;
  /** Rilegge appunti ed etichette. */
  reload: () => void;
}

export function useNotes(query: NoteQuery): NotesList {
  const [notes, setNotes] = useState<Note[]>([]);
  const [tags, setTags] = useState<string[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  /** Cambiandolo si rilegge tutto. */
  const [reloadKey, setReloadKey] = useState(0);

  useEffect(() => {
    /** `false` se nel frattempo è partita un'altra richiesta: questa risposta va ignorata. */
    let isCurrent = true;
    Promise.all([listNotes(query), fetchNoteTags()])
      .then(([found, allTags]) => {
        if (isCurrent) {
          setNotes(found);
          setTags(allTags);
          setError(null);
        }
      })
      .catch((listError: unknown) => {
        if (isCurrent) {
          setError(describeError(listError));
        }
      })
      .finally(() => {
        if (isCurrent) {
          setIsLoading(false);
        }
      });
    return () => {
      isCurrent = false;
    };
  }, [query, reloadKey]);

  const reload = useCallback(() => {
    setReloadKey((key) => key + 1);
  }, []);

  return { notes, tags, isLoading, error, reload };
}
