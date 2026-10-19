/**
 * useCoachIdeas — lo "sguardo lungo" della modalità Tutor, dopo ogni mossa.
 *
 * Cosa fa: quando i commenti della posizione sono pronti (vedi
 * `useCoachMessages`), chiede a Rust, una dopo l'altra:
 * 1. l'**idea della tua ultima mossa** e, se Stockfish preferiva un'altra
 *    mossa, l'idea di quella (con il confronto);
 * 2. le **idee per continuare**: le migliori varianti della posizione attuale.
 * Il motore del tutor pensa in sottofondo qualche secondo per ciascuna; se
 * nel frattempo si gioca un'altra mossa, il calcolo vecchio viene annullato e
 * si ricomincia sulla posizione nuova. Le richieste partono solo dopo i
 * commenti perché il motore del tutor segue una richiesta alla volta.
 *
 * @param game - la partita mostrata, o `null`.
 * @param gameId - il nome unico della partita.
 * @param enabled - `true` nella modalità Tutor.
 * @param ready - `true` quando i commenti della posizione sono pronti.
 * @param humanSide - il colore della persona contro il computer; `null` a due giocatori.
 * @returns l'idea della mossa (con la mossa a cui si riferisce), le idee per
 *   continuare e se si sta ancora guardando avanti.
 */
import { useEffect, useState } from 'react';
import type { GameView } from '../../../bindings/GameView';
import type { MoveIdea } from '../../../bindings/MoveIdea';
import type { MoveRecord } from '../../../bindings/MoveRecord';
import type { PositionIdeas } from '../../../bindings/PositionIdeas';
import type { Side } from '../../../bindings/Side';
import { fetchMoveIdea, fetchPositionIdeas } from '../../../services/tutorService';

/** Quanto pensa il motore per l'idea della mossa (per ciascuna delle due posizioni). */
const MOVE_IDEA_MS = 1500;
/** Quanto pensa il motore per le idee per continuare. */
const POSITION_IDEAS_MS = 3000;

/** Le idee pronte per una posizione. */
export interface CoachIdeas {
  /** L'ultima mossa della persona e la sua idea, se c'è una mossa da commentare. */
  move: { record: MoveRecord; idea: MoveIdea } | null;
  /** Le idee per continuare, se la partita non è finita. */
  position: PositionIdeas | null;
  /** `true` mentre il tutor sta ancora guardando avanti. */
  isThinking: boolean;
}

/** Ciò che si tiene per una posizione. */
interface IdeasResult {
  key: string;
  move: CoachIdeas['move'];
  position: PositionIdeas | null;
  done: boolean;
}

/** La mezza mossa (1 = la prima) dell'ultima mossa della persona, fino a `cursor`. */
function lastHumanPly(game: GameView, humanSide: Side | null): number | null {
  for (let ply = game.cursor; ply >= 1 && ply >= game.cursor - 1; ply -= 1) {
    const record = game.moves[ply - 1];
    if (record !== undefined && (humanSide === null || record.side === humanSide)) {
      return ply;
    }
  }
  return null;
}

export function useCoachIdeas(
  game: GameView | null,
  gameId: string | undefined,
  enabled: boolean,
  ready: boolean,
  humanSide: Side | null,
): CoachIdeas {
  const [result, setResult] = useState<IdeasResult | null>(null);
  const shownUci =
    game === null
      ? ''
      : game.moves
          .slice(0, game.cursor)
          .map((move) => move.uci)
          .join(' ');
  const key = `${gameId ?? ''}|${game?.startingFen ?? ''}|${shownUci}`;
  const isOver = game?.result !== null && game?.result !== undefined;

  useEffect(() => {
    if (!enabled || !ready || game === null) {
      return;
    }
    let isCurrent = true;
    const shown = game.moves.slice(0, game.cursor).map((move) => move.uci);
    const ply = lastHumanPly(game, humanSide);
    const record = ply === null ? undefined : game.moves[ply - 1];

    const run = async () => {
      let move: CoachIdeas['move'] = null;
      if (ply !== null && record !== undefined) {
        const idea = await fetchMoveIdea(
          game.startingFen,
          shown.slice(0, ply - 1),
          record.uci,
          MOVE_IDEA_MS,
        );
        move = { record, idea };
        if (!isCurrent) {
          return;
        }
        setResult({ key, move, position: null, done: isOver });
      }
      if (isOver) {
        return;
      }
      const position = await fetchPositionIdeas(game.startingFen, shown, POSITION_IDEAS_MS);
      if (isCurrent) {
        setResult({ key, move, position, done: true });
      }
    };
    run().catch(() => {
      // Calcolo annullato (un'altra mossa) o motore assente: le idee non compaiono.
      if (isCurrent) {
        setResult((current) =>
          current?.key === key
            ? { ...current, done: true }
            : { key, move: null, position: null, done: true },
        );
      }
    });
    return () => {
      isCurrent = false;
    };
    // `key` riassume partita e posizione; `ready` dice quando partire.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [key, enabled, ready]);

  const current = enabled && result?.key === key ? result : null;
  return {
    move: current?.move ?? null,
    position: current?.position ?? null,
    isThinking: enabled && ready && current?.done !== true,
  };
}
