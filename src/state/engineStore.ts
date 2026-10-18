/**
 * Stato condiviso del motore: l'analisi in tempo reale e gli eventuali problemi.
 *
 * L'analisi si accende e si spegne da qui; quando è accesa, Rust manda un
 * aggiornamento a ogni profondità completata dal motore e qui si conserva
 * l'ultimo. Chi lo mostra controlla che riguardi la posizione mostrata
 * (stessa FEN), perché un aggiornamento può arrivare subito dopo un cambio di posizione.
 */
import { create } from 'zustand';
import type { AnalysisUpdate } from '../bindings/AnalysisUpdate';
import { describeError } from '../lib/describeError';
import { listenToAnalysis, listenToEngineErrors, setAnalysis } from '../services/engineService';

/** Cosa contiene lo stato e cosa si può fare con esso. */
export interface EngineState {
  /** `true` se l'analisi è accesa. */
  analysisEnabled: boolean;
  /** L'ultimo aggiornamento dell'analisi, oppure `null`. */
  latest: AnalysisUpdate | null;
  /** L'ultimo problema del motore da mostrare, oppure `null`. */
  engineError: string | null;
  /** Comincia ad ascoltare analisi e problemi del motore (una volta sola). */
  listen: () => Promise<void>;
  /** Accende o spegne l'analisi. */
  setAnalysisEnabled: (enabled: boolean) => Promise<void>;
  /** Toglie il messaggio di errore mostrato. */
  dismissError: () => void;
}

/** `true` dopo che si è cominciato ad ascoltare gli eventi del motore. */
let isListening = false;

export const useEngineStore = create<EngineState>()((set) => ({
  analysisEnabled: false,
  latest: null,
  engineError: null,

  listen: async () => {
    if (isListening) {
      return;
    }
    isListening = true;
    try {
      await listenToAnalysis((latest) => {
        set({ latest });
      });
      await listenToEngineErrors((engineError) => {
        set({ engineError });
      });
    } catch {
      // Fuori dall'app desktop (ad esempio nei test) gli eventi non esistono.
      isListening = false;
    }
  },

  setAnalysisEnabled: async (enabled) => {
    set({ analysisEnabled: enabled, latest: null });
    try {
      await setAnalysis(enabled);
    } catch (error) {
      set({ analysisEnabled: false, engineError: describeError(error) });
    }
  },

  dismissError: () => {
    set({ engineError: null });
  },
}));
