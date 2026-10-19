/**
 * Stato condiviso delle osservazioni: se la scheda "Osservazioni" è accesa.
 *
 * Sta in uno stato condiviso (e non dentro la pagina) così la scelta resta
 * anche passando da una sezione all'altra. Contro il computer parte spenta:
 * la spegne la pagina "Gioca" quando comincia una partita contro il computer.
 */
import { create } from 'zustand';

/** Cosa contiene lo stato e cosa si può fare con esso. */
export interface ObservationsState {
  /** `true` se le osservazioni sono accese. */
  enabled: boolean;
  /** Accende o spegne le osservazioni. */
  setEnabled: (enabled: boolean) => void;
}

export const useObservationsStore = create<ObservationsState>()((set) => ({
  enabled: false,
  setEnabled: (enabled) => {
    set({ enabled });
  },
}));
