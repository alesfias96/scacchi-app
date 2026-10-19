/**
 * usePositionReport — i fatti trovati dai rilevatori nella posizione mostrata.
 *
 * Cosa fa: quando le osservazioni sono accese, chiede a Rust il rapporto a
 * ogni cambio di posizione (una mossa, uno spostamento nella lista). La
 * richiesta è velocissima; se però arrivassero risposte fuori ordine, vale
 * solo quella della posizione mostrata adesso (stessa FEN).
 *
 * Riceve:
 * - `fen`: la posizione mostrata (o `null` se la partita non è ancora arrivata);
 * - `enabled`: se le osservazioni sono accese.
 * Restituisce: il rapporto della posizione mostrata, oppure `null` (spente,
 * in attesa, o non disponibile).
 */
import { useEffect, useState } from 'react';
import type { PositionReport } from '../bindings/PositionReport';
import { fetchPositionReport } from '../services/insightService';

export function usePositionReport(fen: string | null, enabled: boolean): PositionReport | null {
  const [report, setReport] = useState<PositionReport | null>(null);

  useEffect(() => {
    if (!enabled || fen === null) {
      return;
    }
    let isCurrent = true;
    fetchPositionReport()
      .then((received) => {
        if (isCurrent) {
          setReport(received);
        }
      })
      .catch(() => {
        // Senza rapporto la scheda resta in attesa: non è un errore da mostrare.
      });
    return () => {
      isCurrent = false;
    };
  }, [fen, enabled]);

  return enabled && report?.fen === fen ? report : null;
}
