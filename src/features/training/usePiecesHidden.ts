/**
 * usePiecesHidden — nella visualizzazione, nasconde i pezzi dopo qualche secondo.
 *
 * Cosa fa: a ogni nuovo puzzle in modalità "visualizzazione" lascia vedere la
 * posizione per i secondi scelti, poi dice di nascondere i pezzi. A puzzle
 * finito i pezzi tornano visibili, per guardare la soluzione.
 *
 * @param attempt - il puzzle in corso (`null` se non ce n'è).
 * @param seconds - per quanti secondi si vede la posizione.
 * @returns `hidden`: i pezzi vanno nascosti; `memorizing`: si sta ancora
 *   memorizzando la posizione (i pezzi si vedono, ma spariranno).
 */
import { useEffect, useState } from 'react';
import type { AttemptView } from '../../bindings/AttemptView';

/** Millisecondi in un secondo. */
const MS_PER_SECOND = 1000;

export function usePiecesHidden(
  attempt: AttemptView | null,
  seconds: number,
): { hidden: boolean; memorizing: boolean } {
  /** Il puzzle per cui il tempo per memorizzare è già finito. */
  const [expiredFor, setExpiredFor] = useState<string | null>(null);

  const puzzleId = attempt?.puzzleId ?? null;
  const isVisualization = attempt?.mode === 'visualization';
  const isActive = isVisualization && !attempt.finished;

  useEffect(() => {
    if (!isVisualization || puzzleId === null) {
      return;
    }
    const timer = window.setTimeout(() => {
      setExpiredFor(puzzleId);
    }, seconds * MS_PER_SECOND);
    return () => {
      window.clearTimeout(timer);
    };
  }, [isVisualization, puzzleId, seconds]);

  const expired = puzzleId !== null && expiredFor === puzzleId;
  return { hidden: isActive && expired, memorizing: isActive && !expired };
}
