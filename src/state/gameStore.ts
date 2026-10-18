/**
 * Stato condiviso della partita in corso.
 *
 * La partita "vera" vive in Rust; qui l'interfaccia conserva l'ultima
 * descrizione ricevuta (`SessionView`) e offre le azioni per cambiarla.
 * Ogni azione chiede a Rust di eseguire l'operazione e poi salva la risposta:
 * così le regole degli scacchi restano tutte nel nucleo Rust.
 *
 * La partita può anche cambiare "da sola" (mossa del computer, tempo
 * scaduto): Rust lo annuncia con un evento, che questo stato ascolta dal
 * primo caricamento in poi.
 */
import { create } from 'zustand';
import type { GameSetup } from '../bindings/GameSetup';
import type { SessionView } from '../bindings/SessionView';
import { describeError } from '../lib/describeError';
import {
  fetchGame,
  goToPly,
  importGame,
  listenToGameUpdates,
  playMove,
  resignGame,
  startNewGame,
  undoMove,
} from '../services/gameService';

/** Cosa contiene lo stato e cosa si può fare con esso. */
export interface GameState {
  /** La partita in corso; `null` finché non è arrivata da Rust. */
  session: SessionView | null;
  /** Il motivo per cui la partita non si è potuta leggere all'avvio, oppure `null`. */
  loadError: string | null;
  /** Il messaggio dell'ultima azione non riuscita (es. mossa rifiutata), oppure `null`. */
  actionError: string | null;
  /** Legge la partita in corso da Rust e comincia ad ascoltarne i cambiamenti. */
  load: () => Promise<void>;
  /** Comincia una nuova partita, come scelto nella finestra "Nuova partita". */
  newGame: (setup: GameSetup) => Promise<void>;
  /** Gioca una mossa in notazione UCI (es. `"e2e4"`). */
  play: (uci: string) => Promise<void>;
  /** Ritira l'ultima mossa (contro il computer anche la sua risposta). */
  undo: () => Promise<void>;
  /** Mostra la posizione dopo `ply` mosse (0 = partenza). */
  goTo: (ply: number) => Promise<void>;
  /** Abbandona la partita. */
  resign: () => Promise<void>;
  /**
   * Sostituisce la partita con un testo FEN o PGN.
   * A differenza delle altre azioni, in caso di errore lo rilancia: chi la
   * chiama (la finestra "Incolla") mostra il motivo accanto al testo.
   */
  importText: (text: string) => Promise<void>;
}

/** `true` dopo che si è cominciato ad ascoltare gli eventi della partita (una volta sola). */
let isListening = false;

export const useGameStore = create<GameState>()((set) => {
  /**
   * Esegue una richiesta a Rust e salva la partita ricevuta.
   * Se la richiesta fallisce, la partita resta com'era e l'errore viene annotato.
   */
  const run = async (request: () => Promise<SessionView>) => {
    try {
      const session = await request();
      set({ session, actionError: null });
    } catch (error) {
      set({ actionError: describeError(error) });
    }
  };

  return {
    session: null,
    loadError: null,
    actionError: null,

    load: async () => {
      if (!isListening) {
        isListening = true;
        try {
          await listenToGameUpdates((session) => {
            set({ session });
          });
        } catch {
          // Senza eventi la partita funziona lo stesso: mancano solo le
          // mosse del computer. L'errore vero compare leggendo la partita.
          isListening = false;
        }
      }
      try {
        const session = await fetchGame();
        set({ session, loadError: null });
      } catch (error) {
        set({ loadError: describeError(error) });
      }
    },

    newGame: (setup) => run(() => startNewGame(setup)),
    play: (uci) => run(() => playMove(uci)),
    undo: () => run(undoMove),
    goTo: (ply) => run(() => goToPly(ply)),
    resign: () => run(resignGame),

    importText: async (text) => {
      const session = await importGame(text);
      set({ session, actionError: null });
    },
  };
});
