/**
 * useArchiveGames — le partite dell'archivio che rispettano i filtri.
 *
 * Cosa fa: chiede a Rust l'elenco ogni volta che i filtri cambiano e tiene
 * traccia della richiesta (in attesa, pronta, errore). Se arrivano risposte
 * fuori ordine (filtri cambiati in fretta), conta solo l'ultima.
 *
 * Riceve:
 * - `filter`: i filtri da applicare.
 * Restituisce: le partite, se si sta caricando, l'eventuale errore e
 * `reload`, per ricaricare l'elenco (ad esempio dopo un'eliminazione).
 */
import { useCallback, useEffect, useState } from 'react';
import type { GameFilter } from '../../bindings/GameFilter';
import type { GameSummary } from '../../bindings/GameSummary';
import { describeError } from '../../lib/describeError';
import { listGames } from '../../services/archiveService';

/** Ciò che restituisce l'hook. */
export interface ArchiveGames {
  /** Le partite trovate, dalla più recente. */
  games: GameSummary[];
  /** `true` mentre si aspetta la risposta. */
  isLoading: boolean;
  /** Il motivo per cui l'elenco non si è potuto leggere, oppure `null`. */
  error: string | null;
  /** Ricarica l'elenco con gli stessi filtri. */
  reload: () => void;
}

export function useArchiveGames(filter: GameFilter): ArchiveGames {
  const [games, setGames] = useState<GameSummary[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  /** Cambiandolo si ricarica l'elenco. */
  const [reloadKey, setReloadKey] = useState(0);

  useEffect(() => {
    /** `false` se nel frattempo è partita un'altra richiesta: questa risposta va ignorata. */
    let isCurrent = true;
    setIsLoading(true);
    listGames(filter)
      .then((found) => {
        if (isCurrent) {
          setGames(found);
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
  }, [filter, reloadKey]);

  const reload = useCallback(() => {
    setReloadKey((key) => key + 1);
  }, []);

  return { games, isLoading, error, reload };
}
