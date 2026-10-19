/**
 * Stato condiviso del tutor: quale posizione spiegare, la spiegazione e le
 * mosse provate ("e se…").
 *
 * Le sezioni dicono al tutor quale posizione stanno mostrando (con
 * `useTutorTarget`); il pannello del tutor, che sta nella cornice dell'app,
 * la spiega quando è aperto. Tutte le spiegazioni le compone Rust.
 */
import { create } from 'zustand';
import type { Side } from '../bindings/Side';
import type { TutorExplanation } from '../bindings/TutorExplanation';
import type { WhatIfView } from '../bindings/WhatIfView';
import { describeError } from '../lib/describeError';
import { explainPosition, tryWhatIf } from '../services/tutorService';

/** Perché il tutor non può spiegare la posizione in questo momento. */
export type TutorLock = 'computerGame' | 'puzzle';

/** La posizione che la sezione aperta mostra al tutor. */
export interface TutorTarget {
  /** La posizione di partenza della partita (FEN). */
  startFen: string;
  /** Le mosse fino alla posizione mostrata (UCI). */
  moves: readonly string[];
  /** Il lato della scacchiera in basso. */
  orientation: Side;
  /**
   * Se il tutor deve restare spento: durante una partita contro il computer
   * (si può accendere comunque) o durante un puzzle non ancora finito.
   */
  lock: TutorLock | null;
  /** A cosa si riferisce il blocco (la partita o il puzzle), per sbloccarlo una volta sola. */
  lockKey: string;
}

/** Cosa contiene lo stato e cosa si può fare con esso. */
export interface TutorState {
  /** La posizione mostrata dalla sezione aperta; `null` se non ce n'è. */
  target: TutorTarget | null;
  /** `true` mentre il pannello del tutor è aperto. */
  isOpen: boolean;
  /** Se chiedere anche il giudizio di Stockfish. */
  withEngine: boolean;
  /** La partita contro il computer per cui il tutor è stato acceso comunque. */
  unlockedKey: string | null;
  /** La spiegazione della posizione; `null` finché non arriva. */
  explanation: TutorExplanation | null;
  /** `true` mentre si aspetta una risposta. */
  isLoading: boolean;
  /** Il messaggio dell'ultima richiesta fallita, oppure `null`. */
  error: string | null;
  /** Le mosse provate, ognuna con il suo scenario; `null` se non si sta provando. */
  whatIf: WhatIfView[] | null;
  /** Cambia la posizione mostrata (la chiamano le sezioni). */
  setTarget: (target: TutorTarget | null) => void;
  /** Il pannello si apre o si chiude. */
  setOpen: (isOpen: boolean) => void;
  /** Accende o spegne il giudizio di Stockfish. */
  setWithEngine: (withEngine: boolean) => void;
  /** Accende il tutor anche durante la partita contro il computer. */
  unlock: () => void;
  /** Spiega la posizione mostrata (se il pannello è aperto e il tutor non è bloccato). */
  refresh: () => Promise<void>;
  /** Comincia a provare mosse dalla posizione spiegata. */
  startWhatIf: () => void;
  /** Prova una mossa (UCI) dopo quelle già provate. */
  playWhatIf: (uci: string) => Promise<void>;
  /** Ritira l'ultima mossa provata. */
  undoWhatIf: () => void;
  /** Smette di provare e torna alla posizione della partita. */
  stopWhatIf: () => void;
}

/** `true` se il tutor non può spiegare la posizione. */
export function isTutorLocked(target: TutorTarget, unlockedKey: string | null): boolean {
  if (target.lock === 'puzzle') {
    return true;
  }
  return target.lock === 'computerGame' && unlockedKey !== target.lockKey;
}

/** Due posizioni sono la stessa se hanno la stessa partenza e le stesse mosse. */
function samePosition(a: TutorTarget | null, b: TutorTarget | null): boolean {
  return (
    a !== null &&
    b !== null &&
    a.startFen === b.startFen &&
    a.moves.length === b.moves.length &&
    a.moves.every((move, index) => move === b.moves[index])
  );
}

export const useTutorStore = create<TutorState>()((set, get) => {
  /** Il numero dell'ultima richiesta di spiegazione: le risposte vecchie si ignorano. */
  let request = 0;

  return {
    target: null,
    isOpen: false,
    withEngine: true,
    unlockedKey: null,
    explanation: null,
    isLoading: false,
    error: null,
    whatIf: null,

    setTarget: (target) => {
      const previous = get().target;
      if (samePosition(previous, target) && previous?.lock === target?.lock) {
        // Stessa posizione: si aggiornano solo i dettagli (es. il lato in basso).
        set({ target });
        return;
      }
      set({ target, explanation: null, error: null, whatIf: null });
      void get().refresh();
    },

    setOpen: (isOpen) => {
      set({ isOpen });
      if (isOpen && get().explanation === null) {
        void get().refresh();
      }
    },

    setWithEngine: (withEngine) => {
      set({ withEngine, explanation: null, whatIf: null });
      void get().refresh();
    },

    unlock: () => {
      const { target } = get();
      if (target !== null) {
        set({ unlockedKey: target.lockKey });
        void get().refresh();
      }
    },

    refresh: async () => {
      const { target, isOpen, unlockedKey, withEngine } = get();
      if (!isOpen || target === null || isTutorLocked(target, unlockedKey)) {
        return;
      }
      request += 1;
      const own = request;
      set({ isLoading: true, error: null });
      try {
        const explanation = await explainPosition(target.startFen, target.moves, withEngine);
        if (own === request) {
          set({ explanation, isLoading: false });
        }
      } catch (error) {
        if (own === request) {
          set({ error: describeError(error), isLoading: false });
        }
      }
    },

    startWhatIf: () => {
      set({ whatIf: [] });
    },

    playWhatIf: async (uci) => {
      const { target, whatIf, withEngine } = get();
      if (target === null || whatIf === null) {
        return;
      }
      const tried = whatIf.map((scenario) => scenario.lastMove?.uci ?? '');
      set({ isLoading: true, error: null });
      try {
        const scenario = await tryWhatIf(
          target.startFen,
          [...target.moves, ...tried],
          uci,
          withEngine,
        );
        // Se nel frattempo si è smesso di provare, la risposta non serve più.
        const current = get().whatIf;
        set(
          current === null
            ? { isLoading: false }
            : { whatIf: [...current, scenario], isLoading: false },
        );
      } catch (error) {
        set({ error: describeError(error), isLoading: false });
      }
    },

    undoWhatIf: () => {
      const { whatIf } = get();
      if (whatIf !== null) {
        set({ whatIf: whatIf.slice(0, -1) });
      }
    },

    stopWhatIf: () => {
      set({ whatIf: null });
    },
  };
});
