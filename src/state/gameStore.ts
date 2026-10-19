/**
 * Stato condiviso della partita in corso.
 *
 * La partita "vera" vive in Rust; qui l'interfaccia conserva l'ultima
 * descrizione ricevuta (`SessionView`) e offre le azioni per cambiarla.
 * Ogni azione chiede a Rust di eseguire l'operazione e poi salva la risposta:
 * così le regole degli scacchi restano tutte nel nucleo Rust.
 *
 * La partita può anche cambiare "da sola" (mossa del computer, tempo
 * scaduto, partita ripresa all'avvio): Rust lo annuncia con un evento, che
 * questo stato ascolta dal primo caricamento in poi, insieme agli eventuali
 * problemi del database delle partite.
 */
import { create } from 'zustand';
import type { GameSetup } from '../bindings/GameSetup';
import type { SessionView } from '../bindings/SessionView';
import { describeError } from '../lib/describeError';
import { listenToStorageErrors, openSavedGame } from '../services/archiveService';
import {
  deleteVariation,
  fetchGame,
  goToNode,
  goToPly,
  importGame,
  listenToGameUpdates,
  openPgnFile,
  playMove,
  promoteVariation,
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
  /** L'ultimo problema del database delle partite (es. salvataggio non riuscito), oppure `null`. */
  storageError: string | null;
  /** Legge la partita in corso da Rust e comincia ad ascoltarne i cambiamenti. */
  load: () => Promise<void>;
  /** Comincia una nuova partita, come scelto nella finestra "Nuova partita". */
  newGame: (setup: GameSetup) => Promise<void>;
  /** Gioca una mossa in notazione UCI (es. `"e2e4"`). */
  play: (uci: string) => Promise<void>;
  /** Ritira l'ultima mossa (contro il computer anche la sua risposta). */
  undo: () => Promise<void>;
  /** Mostra la posizione dopo `ply` mosse della linea guardata (0 = partenza). */
  goTo: (ply: number) => Promise<void>;
  /** Mostra la posizione dopo una mossa dell'albero (`null` = partenza), anche in una variante. */
  goToNode: (node: number | null) => Promise<void>;
  /** Promuove a linea principale la variante della mossa indicata. */
  promoteVariation: (node: number) => Promise<void>;
  /** Elimina la mossa indicata e il suo seguito (solo nelle varianti). */
  deleteVariation: (node: number) => Promise<void>;
  /** Abbandona la partita. */
  resign: () => Promise<void>;
  /**
   * Apre una partita dell'archivio.
   * Come `importText`, in caso di errore lo rilancia (lo mostra l'archivio).
   */
  openSaved: (id: string) => Promise<void>;
  /** Apre un file `.pgn` scelto con la finestra del sistema (se l'utente non annulla). */
  openFile: () => Promise<void>;
  /** Toglie il messaggio del problema del database. */
  dismissStorageError: () => void;
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
    storageError: null,

    load: async () => {
      if (!isListening) {
        isListening = true;
        try {
          await listenToGameUpdates((session) => {
            set({ session });
          });
          await listenToStorageErrors((storageError) => {
            set({ storageError });
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
    goToNode: (node) => run(() => goToNode(node)),
    promoteVariation: (node) => run(() => promoteVariation(node)),
    deleteVariation: (node) => run(() => deleteVariation(node)),
    resign: () => run(resignGame),

    openSaved: async (id) => {
      const session = await openSavedGame(id);
      set({ session, actionError: null });
    },

    openFile: async () => {
      try {
        const session = await openPgnFile();
        if (session !== null) {
          set({ session, actionError: null });
        }
      } catch (error) {
        set({ actionError: describeError(error) });
      }
    },

    dismissStorageError: () => {
      set({ storageError: null });
    },

    importText: async (text) => {
      const session = await importGame(text);
      set({ session, actionError: null });
    },
  };
});
