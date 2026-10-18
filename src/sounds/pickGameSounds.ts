/**
 * Decide quali suoni far sentire quando la partita mostrata cambia.
 *
 * Si suona solo quando la scacchiera avanza di **una** mossa lungo la stessa
 * partita: una mossa appena giocata (dall'utente o, dalla Fase 3, dal
 * computer) oppure un passo avanti con → o ▶. Non si suona tornando
 * indietro, saltando a inizio o fine, cominciando una nuova partita o
 * incollandone una.
 *
 * Se la mossa chiude la partita, al suono della mossa segue quello di fine partita.
 */
import type { GameView } from '../bindings/GameView';
import type { GameEndSound } from './playGameEndSound';

/** I suoni da far sentire; `null` = nessun suono di quel tipo. */
export interface GameSounds {
  /** Il suono della mossa: normale o presa. */
  move: 'move' | 'capture' | null;
  /** Il suono di fine partita. */
  end: GameEndSound | null;
}

/** Nessun suono. */
const SILENT: GameSounds = { move: null, end: null };

/**
 * @param previous - la partita mostrata prima (o `null` se non c'era ancora).
 * @param next - la partita mostrata ora.
 * @returns i suoni da far sentire.
 */
export function pickGameSounds(previous: GameView | null, next: GameView): GameSounds {
  if (previous === null || next.cursor !== previous.cursor + 1) {
    return SILENT;
  }
  // Le mosse fino alla posizione di prima devono essere le stesse: altrimenti
  // è un'altra partita (ad esempio una incollata) e non una mossa in più.
  const sameGame = previous.moves
    .slice(0, previous.cursor)
    .every((record, index) => record.uci === next.moves[index]?.uci);
  const played = next.moves[next.cursor - 1];
  if (!sameGame || played === undefined) {
    return SILENT;
  }
  return {
    move: played.capture ? 'capture' : 'move',
    end: next.result === null ? null : next.result.winner === null ? 'draw' : 'decisive',
  };
}
