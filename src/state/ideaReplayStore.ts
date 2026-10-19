/**
 * Stato condiviso di «Guarda l'idea sulla scacchiera»: quale idea si sta
 * guardando e a quale mossa della sua variante si è arrivati.
 *
 * Sta in uno stato condiviso perché lo usano due parti lontane della pagina:
 * la scheda dell'idea (nella colonna di destra, con i pulsanti per scorrere la
 * variante) e la scacchiera (a sinistra, che mostra la posizione della variante
 * al posto di quella della partita).
 */
import { create } from 'zustand';
import type { IdeaView } from '../bindings/IdeaView';

/** Cosa contiene lo stato e cosa si può fare con esso. */
export interface IdeaReplayState {
  /** L'idea che si sta guardando; `null` se la scacchiera mostra la partita. */
  idea: IdeaView | null;
  /** La posizione mostrata: 0 = partenza, 1 = dopo la prima mossa della variante, … */
  frame: number;
  /** Comincia a guardare un'idea, dalla sua posizione di partenza. */
  show: (idea: IdeaView) => void;
  /** Va a una posizione della variante (riportata nei limiti). */
  goTo: (frame: number) => void;
  /** Smette di guardare: la scacchiera torna alla partita. */
  close: () => void;
}

export const useIdeaReplayStore = create<IdeaReplayState>()((set, get) => ({
  idea: null,
  frame: 0,

  show: (idea) => {
    set({ idea, frame: 0 });
  },

  goTo: (frame) => {
    const { idea } = get();
    if (idea === null) {
      return;
    }
    set({ frame: Math.min(Math.max(frame, 0), idea.frames.length - 1) });
  },

  close: () => {
    set({ idea: null, frame: 0 });
  },
}));
