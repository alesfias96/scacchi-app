/**
 * Stato condiviso della partita in corso.
 *
 * La partita "vera" vive in Rust; qui l'interfaccia conserva l'ultima
 * descrizione ricevuta (`GameView`) e offre le azioni per cambiarla.
 * Ogni azione chiede a Rust di eseguire l'operazione e poi salva la risposta:
 * così le regole degli scacchi restano tutte nel nucleo Rust.
 */
import { create } from 'zustand';
import type { GameView } from '../bindings/GameView';
import { describeError } from '../lib/describeError';
import {
  fetchGame,
  goToPly,
  importGame,
  playMove,
  startNewGame,
  undoMove,
} from '../services/gameService';

/** Cosa contiene lo stato e cosa si può fare con esso. */
export interface GameState {
  /** La partita in corso; `null` finché non è arrivata da Rust. */
  game: GameView | null;
  /** Il motivo per cui la partita non si è potuta leggere all'avvio, oppure `null`. */
  loadError: string | null;
  /** Il messaggio dell'ultima azione non riuscita (es. mossa rifiutata), oppure `null`. */
  actionError: string | null;
  /** Legge la partita in corso da Rust. */
  load: () => Promise<void>;
  /** Comincia una nuova partita. */
  newGame: () => Promise<void>;
  /** Gioca una mossa in notazione UCI (es. `"e2e4"`). */
  play: (uci: string) => Promise<void>;
  /** Ritira l'ultima mossa. */
  undo: () => Promise<void>;
  /** Mostra la posizione dopo `ply` mosse (0 = partenza). */
  goTo: (ply: number) => Promise<void>;
  /**
   * Sostituisce la partita con un testo FEN o PGN.
   * A differenza delle altre azioni, in caso di errore lo rilancia: chi la
   * chiama (la finestra "Incolla") mostra il motivo accanto al testo.
   */
  importText: (text: string) => Promise<void>;
}

export const useGameStore = create<GameState>()((set) => {
  /**
   * Esegue una richiesta a Rust e salva la partita ricevuta.
   * Se la richiesta fallisce, la partita resta com'era e l'errore viene annotato.
   */
  const run = async (request: () => Promise<GameView>) => {
    try {
      const game = await request();
      set({ game, actionError: null });
    } catch (error) {
      set({ actionError: describeError(error) });
    }
  };

  return {
    game: null,
    loadError: null,
    actionError: null,

    load: async () => {
      try {
        const game = await fetchGame();
        set({ game, loadError: null });
      } catch (error) {
        set({ loadError: describeError(error) });
      }
    },

    newGame: () => run(startNewGame),
    play: (uci) => run(() => playMove(uci)),
    undo: () => run(undoMove),
    goTo: (ply) => run(() => goToPly(ply)),

    importText: async (text) => {
      const game = await importGame(text);
      set({ game, actionError: null });
    },
  };
});
