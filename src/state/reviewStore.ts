/**
 * Stato condiviso dell'analisi delle partite: quale partita si sta
 * analizzando, a che punto è e com'è finita l'ultima analisi.
 *
 * Sta in uno stato condiviso perché l'analisi continua in sottofondo anche
 * passando da una sezione all'altra (Archivio, Analisi partite, Gioca).
 */
import { create } from 'zustand';
import type { ReviewFinished } from '../bindings/ReviewFinished';
import type { ReviewProgress } from '../bindings/ReviewProgress';
import { describeError } from '../lib/describeError';
import {
  cancelReview,
  fetchRunningReview,
  listenToReviewFinished,
  listenToReviewProgress,
  startReview,
} from '../services/reviewService';

/** Cosa contiene lo stato e cosa si può fare con esso. */
export interface ReviewState {
  /** La partita in analisi; `null` se non c'è nessuna analisi in corso. */
  runningId: string | null;
  /** L'avanzamento dell'analisi in corso. */
  progress: ReviewProgress | null;
  /** Com'è finita l'ultima analisi (serve a rileggere la revisione). */
  lastFinished: ReviewFinished | null;
  /** Il messaggio dell'ultimo problema, oppure `null`. */
  error: string | null;
  /** Comincia ad ascoltare gli eventi dell'analisi (una volta sola) e legge l'analisi in corso. */
  listen: () => Promise<void>;
  /** Comincia ad analizzare una partita. */
  start: (id: string) => Promise<void>;
  /** Ferma l'analisi in corso. */
  cancel: () => Promise<void>;
}

/** `true` dopo la prima chiamata a `listen`: gli eventi si ascoltano una volta sola. */
let isListening = false;

export const useReviewStore = create<ReviewState>()((set, get) => ({
  runningId: null,
  progress: null,
  lastFinished: null,
  error: null,

  listen: async () => {
    if (!isListening) {
      isListening = true;
      await listenToReviewProgress((progress) => {
        set({ runningId: progress.gameId, progress });
      });
      await listenToReviewFinished((finished) => {
        set({
          runningId: null,
          progress: null,
          lastFinished: finished,
          error: finished.error,
        });
      });
    }
    try {
      set({ runningId: await fetchRunningReview() });
    } catch (error) {
      set({ error: describeError(error) });
    }
  },

  start: async (id) => {
    // Gli eventi dell'analisi vanno ascoltati prima che arrivino.
    await get().listen();
    set({ error: null });
    try {
      await startReview(id);
      set({ runningId: id, progress: null });
    } catch (error) {
      set({ error: describeError(error) });
    }
  },

  cancel: async () => {
    try {
      await cancelReview();
    } catch (error) {
      set({ error: describeError(error) });
    }
  },
}));
