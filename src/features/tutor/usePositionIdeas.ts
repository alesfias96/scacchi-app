/**
 * usePositionIdeas — le idee della posizione, chieste a richiesta («Guarda avanti»).
 *
 * Cosa fa: con `look(thinkMs)` chiede a Rust le migliori varianti di Stockfish
 * spiegate, lasciando pensare il motore per `thinkMs` millesimi di secondo.
 * Le idee valgono solo per la posizione per cui sono state chieste: se la
 * posizione cambia, spariscono. Un calcolo annullato (perché nel frattempo è
 * partita un'altra richiesta al motore del tutor) non è un errore da mostrare.
 *
 * Riceve: `startFen` e `moves`, la posizione (o `null` se non ce n'è).
 * Restituisce: lo stato (pronto, in corso, errore), le idee e `look`.
 */
import { useCallback, useState } from 'react';
import type { PositionIdeas } from '../../bindings/PositionIdeas';
import { describeError } from '../../lib/describeError';
import { fetchPositionIdeas, isCancelled } from '../../services/tutorService';

/** A che punto è la richiesta, per una certa posizione. */
type IdeasState =
  | { kind: 'thinking'; key: string }
  | { kind: 'ready'; key: string; ideas: PositionIdeas }
  | { kind: 'failed'; key: string; message: string };

/** Ciò che restituisce l'hook. */
export interface PositionIdeasRequest {
  /** `true` mentre il motore pensa. */
  isThinking: boolean;
  /** Le idee della posizione attuale, oppure `null`. */
  ideas: PositionIdeas | null;
  /** Il motivo dell'ultimo errore, oppure `null`. */
  error: string | null;
  /** Chiede le idee, lasciando pensare il motore per `thinkMs` millesimi di secondo. */
  look: (thinkMs: number) => void;
}

export function usePositionIdeas(
  startFen: string | null,
  moves: readonly string[],
): PositionIdeasRequest {
  const [state, setState] = useState<IdeasState | null>(null);
  const key = startFen === null ? null : `${startFen}|${moves.join(' ')}`;

  const look = useCallback(
    (thinkMs: number) => {
      if (startFen === null || key === null) {
        return;
      }
      setState({ kind: 'thinking', key });
      fetchPositionIdeas(startFen, moves, thinkMs)
        .then((ideas) => {
          setState((current) => (current?.key === key ? { kind: 'ready', key, ideas } : current));
        })
        .catch((error: unknown) => {
          setState((current) => {
            if (current?.key !== key) {
              return current;
            }
            return isCancelled(error)
              ? null
              : { kind: 'failed', key, message: describeError(error) };
          });
        });
    },
    [startFen, moves, key],
  );

  const current = state !== null && state.key === key ? state : null;
  return {
    isThinking: current?.kind === 'thinking',
    ideas: current?.kind === 'ready' ? current.ideas : null,
    error: current?.kind === 'failed' ? current.message : null,
    look,
  };
}
