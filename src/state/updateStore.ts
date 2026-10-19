/**
 * Stato condiviso degli aggiornamenti: la versione installata, l'esito
 * dell'ultimo controllo e l'installazione in corso.
 *
 * Sta in uno stato condiviso perché lo usano due parti dell'app: la sezione
 * «Aggiornamenti» delle Impostazioni e l'avviso che compare all'avvio (se
 * nelle Impostazioni si è scelto di controllare a ogni avvio).
 */
import { create } from 'zustand';
import { describeError } from '../lib/describeError';
import { checkForUpdate, fetchAppVersion, installUpdate } from '../services/updateService';

/** A che punto è il controllo degli aggiornamenti. */
export type UpdateStatus =
  | { kind: 'idle' }
  | { kind: 'checking' }
  | { kind: 'upToDate' }
  | { kind: 'available'; version: string }
  | { kind: 'installing'; version: string }
  | { kind: 'failed'; message: string };

/** Cosa contiene lo stato e cosa si può fare con esso. */
export interface UpdateState {
  /** La versione installata; `null` finché Rust non l'ha detta. */
  installedVersion: string | null;
  /** L'esito dell'ultimo controllo. */
  status: UpdateStatus;
  /** `true` se l'avviso all'avvio è stato chiuso con «Più tardi». */
  noticeDismissed: boolean;
  /** Chiede a Rust la versione installata. */
  loadVersion: () => Promise<void>;
  /** Controlla se c'è una versione nuova. */
  check: () => Promise<void>;
  /** Installa la versione nuova (poi l'app si riavvia). */
  install: () => Promise<void>;
  /** Chiude l'avviso all'avvio fino alla prossima apertura dell'app. */
  dismissNotice: () => void;
}

export const useUpdateStore = create<UpdateState>()((set, get) => ({
  installedVersion: null,
  status: { kind: 'idle' },
  noticeDismissed: false,

  loadVersion: async () => {
    try {
      set({ installedVersion: await fetchAppVersion() });
    } catch {
      // Senza la versione la sezione dice solo di non saperla.
    }
  },

  check: async () => {
    set({ status: { kind: 'checking' } });
    try {
      const version = await checkForUpdate();
      set({ status: version === null ? { kind: 'upToDate' } : { kind: 'available', version } });
    } catch (error) {
      set({ status: { kind: 'failed', message: describeError(error) } });
    }
  },

  install: async () => {
    const { status } = get();
    if (status.kind !== 'available') {
      return;
    }
    set({ status: { kind: 'installing', version: status.version } });
    try {
      await installUpdate();
    } catch (error) {
      set({ status: { kind: 'failed', message: describeError(error) } });
    }
  },

  dismissNotice: () => {
    set({ noticeDismissed: true });
  },
}));
