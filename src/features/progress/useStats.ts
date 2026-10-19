/**
 * useStats — le statistiche della sezione «Progressi», con i filtri scelti.
 *
 * Cosa fa: chiede a Rust le statistiche ogni volta che i filtri cambiano e
 * tiene traccia della richiesta (in attesa, pronta, errore). Se arrivano
 * risposte fuori ordine (filtri cambiati in fretta), conta solo l'ultima.
 * Mentre si ricarica, restano visibili le statistiche precedenti.
 *
 * Riceve:
 * - `filter`: il periodo e l'avversario.
 * Restituisce: le statistiche (o `null` prima della prima risposta),
 * l'eventuale errore e `reload`, per ricaricarle (ad esempio quando
 * finisce l'analisi di una partita).
 */
import { useCallback, useEffect, useState } from 'react';
import type { StatsFilter } from '../../bindings/StatsFilter';
import type { StatsReport } from '../../bindings/StatsReport';
import { describeError } from '../../lib/describeError';
import { fetchStats } from '../../services/statsService';

/** Ciò che restituisce l'hook. */
export interface Stats {
  /** Le statistiche; `null` finché non arriva la prima risposta. */
  report: StatsReport | null;
  /** Il motivo per cui non si sono potute leggere, oppure `null`. */
  error: string | null;
  /** Le ricarica con gli stessi filtri. */
  reload: () => void;
}

export function useStats(filter: StatsFilter): Stats {
  const [report, setReport] = useState<StatsReport | null>(null);
  const [error, setError] = useState<string | null>(null);
  /** Cambiandolo si ricaricano le statistiche. */
  const [reloadKey, setReloadKey] = useState(0);

  useEffect(() => {
    /** `false` se nel frattempo è partita un'altra richiesta: questa risposta va ignorata. */
    let isCurrent = true;
    fetchStats(filter)
      .then((found) => {
        if (isCurrent) {
          setReport(found);
          setError(null);
        }
      })
      .catch((fetchError: unknown) => {
        if (isCurrent) {
          setError(describeError(fetchError));
        }
      });
    return () => {
      isCurrent = false;
    };
  }, [filter, reloadKey]);

  const reload = useCallback(() => {
    setReloadKey((key) => key + 1);
  }, []);

  return { report, error, reload };
}
