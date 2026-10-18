/**
 * useInitialPosition — chiede a Rust la posizione iniziale e ne segue lo stato.
 *
 * La risposta di Rust non è immediata, quindi l'hook passa per tre stati:
 * - `loading`: richiesta in corso;
 * - `ready`: posizione ricevuta;
 * - `error`: qualcosa è andato storto (con un messaggio leggibile).
 *
 * @returns lo stato attuale della richiesta.
 */
import { useEffect, useState } from 'react';
import type { PositionView } from '../bindings/PositionView';
import { describeError } from '../lib/describeError';
import { fetchInitialPosition } from '../services/positionService';

/** I tre stati possibili della richiesta. */
export type InitialPositionState =
  | { status: 'loading' }
  | { status: 'ready'; position: PositionView }
  | { status: 'error'; message: string };

export function useInitialPosition(): InitialPositionState {
  const [state, setState] = useState<InitialPositionState>({ status: 'loading' });

  useEffect(() => {
    // Se il componente viene rimosso prima della risposta, la risposta va ignorata.
    let isCancelled = false;

    void fetchInitialPosition().then(
      (position) => {
        if (!isCancelled) {
          setState({ status: 'ready', position });
        }
      },
      (error: unknown) => {
        if (!isCancelled) {
          setState({ status: 'error', message: describeError(error) });
        }
      },
    );

    return () => {
      isCancelled = true;
    };
  }, []);

  return state;
}
