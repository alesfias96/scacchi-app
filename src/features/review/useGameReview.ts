/**
 * useGameReview — la revisione di una partita, letta da Rust.
 *
 * Cosa fa: chiede la revisione della partita indicata e la richiede di nuovo
 * quando finisce un'analisi di quella partita.
 *
 * @param id - il nome unico della partita, oppure `null`.
 * @returns lo stato: in attesa, non analizzata, pronta oppure errore.
 */
import { useEffect, useState } from 'react';
import type { GameReview } from '../../bindings/GameReview';
import { describeError } from '../../lib/describeError';
import { fetchReview } from '../../services/reviewService';
import { useReviewStore } from '../../state/reviewStore';

/** I quattro stati possibili. */
export type GameReviewState =
  | { status: 'loading' }
  | { status: 'missing' }
  | { status: 'ready'; review: GameReview }
  | { status: 'error'; message: string };

export function useGameReview(id: string | null): GameReviewState {
  const [state, setState] = useState<{ key: string; value: GameReviewState } | null>(null);
  const lastFinished = useReviewStore((store) => store.lastFinished);
  /** Una nuova lettura a ogni analisi finita di questa partita. */
  const finishedHere = lastFinished?.gameId === id ? lastFinished : null;
  const key = `${id ?? ''}|${finishedHere === null ? '' : JSON.stringify(finishedHere)}`;

  useEffect(() => {
    if (id === null) {
      return;
    }
    let isCurrent = true;
    fetchReview(id).then(
      (review) => {
        if (isCurrent) {
          setState({
            key,
            value: review === null ? { status: 'missing' } : { status: 'ready', review },
          });
        }
      },
      (error: unknown) => {
        if (isCurrent) {
          setState({ key, value: { status: 'error', message: describeError(error) } });
        }
      },
    );
    return () => {
      isCurrent = false;
    };
  }, [id, key]);

  return state?.key === key ? state.value : { status: 'loading' };
}
