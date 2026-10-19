/**
 * Stato condiviso dell'allenamento: il riepilogo e il puzzle in corso.
 *
 * Sta in uno stato condiviso (e non dentro la pagina) così il puzzle resta
 * com'era anche passando da una sezione all'altra. Le regole (mossa giusta o
 * sbagliata, rating, ripasso) sono tutte in Rust: qui si conserva solo
 * l'ultima risposta ricevuta.
 */
import { create } from 'zustand';
import type { AttemptView } from '../bindings/AttemptView';
import type { TrainingOverview } from '../bindings/TrainingOverview';
import type { TrainingSetup } from '../bindings/TrainingSetup';
import { describeError } from '../lib/describeError';
import {
  fetchTrainingOverview,
  playPuzzleMove,
  requestPuzzleHint,
  revealPuzzleSolution,
  startPuzzle,
} from '../services/trainingService';

/** Cosa contiene lo stato e cosa si può fare con esso. */
export interface TrainingState {
  /** Il riepilogo (rating, statistiche, temi); `null` finché non è arrivato. */
  overview: TrainingOverview | null;
  /** Il puzzle in corso; `null` prima del primo puzzle. */
  attempt: AttemptView | null;
  /** Il messaggio dell'ultima richiesta fallita, oppure `null`. */
  error: string | null;
  /** Legge il riepilogo da Rust. */
  loadOverview: () => Promise<void>;
  /** Comincia un nuovo puzzle con le scelte indicate. */
  start: (setup: TrainingSetup) => Promise<void>;
  /** Prova una mossa (in notazione UCI). */
  play: (uci: string) => Promise<void>;
  /** Chiede un aiuto. */
  hint: () => Promise<void>;
  /** Mostra la soluzione. */
  showSolution: () => Promise<void>;
}

export const useTrainingStore = create<TrainingState>()((set, get) => {
  /**
   * Esegue una richiesta sul puzzle e ne conserva la risposta. Quando il
   * risultato si decide (rating cambiato o puzzle finito) rilegge il
   * riepilogo, che nel frattempo è cambiato.
   */
  async function act(request: () => Promise<AttemptView>): Promise<void> {
    try {
      const previous = get().attempt;
      const attempt = await request();
      set({ attempt, error: null });
      const decided = attempt.finished !== previous?.finished || attempt.rating !== previous.rating;
      if (decided) {
        await get().loadOverview();
      }
    } catch (error) {
      set({ error: describeError(error) });
    }
  }

  return {
    overview: null,
    attempt: null,
    error: null,

    loadOverview: async () => {
      try {
        set({ overview: await fetchTrainingOverview() });
      } catch (error) {
        set({ error: describeError(error) });
      }
    },

    start: (setup) => act(() => startPuzzle(setup)),
    play: (uci) => act(() => playPuzzleMove(uci)),
    hint: () => act(requestPuzzleHint),
    showSolution: () => act(revealPuzzleSolution),
  };
});
