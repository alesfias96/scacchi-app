/**
 * useCoachMessages — i commenti del tutor dopo ogni mossa (modalità Tutor).
 *
 * Cosa fa: quando cambia la posizione mostrata chiede a Rust la spiegazione
 * della posizione e il commento alle mosse appena giocate, poi sceglie i
 * messaggi da mostrare (vedi `buildCoachMessages`). Se dall'ultimo commento
 * sono state giocate due mosse (la propria e la risposta del computer) le
 * commenta entrambe, così la propria non va persa. Le richieste vanno una
 * dopo l'altra: il motore del tutor ne segue una sola alla volta.
 *
 * @param game - la partita mostrata, o `null`.
 * @param gameId - il nome unico della partita (cambiandolo si ricomincia da capo).
 * @param enabled - `true` nella modalità Tutor.
 * @param withEngine - se chiedere anche il giudizio di Stockfish.
 * @param notation - la notazione scelta nelle Impostazioni.
 * @returns i messaggi, la spiegazione (per aprire le schede) e se si sta aspettando.
 */
import { useEffect, useRef, useState } from 'react';
import type { GameView } from '../../../bindings/GameView';
import type { Notation } from '../../../bindings/Notation';
import type { TutorExplanation } from '../../../bindings/TutorExplanation';
import { explainPosition, tryWhatIf } from '../../../services/tutorService';
import { buildCoachMessages, type CoachMessage, type MoveComment } from './buildCoachMessages';

/** Quante mosse al massimo si commentano insieme (la propria e la risposta). */
const MAX_COMMENTED_MOVES = 2;

/** I commenti pronti per una posizione. */
interface CoachResult {
  /** La posizione a cui si riferiscono. */
  key: string;
  messages: CoachMessage[];
  explanation: TutorExplanation;
}

export function useCoachMessages(
  game: GameView | null,
  gameId: string | undefined,
  enabled: boolean,
  withEngine: boolean,
  notation: Notation,
): { messages: CoachMessage[]; explanation: TutorExplanation | null; isLoading: boolean } {
  const [result, setResult] = useState<CoachResult | null>(null);
  /** Le frasi già mostrate in questa partita. */
  const seen = useRef(new Set<string>());
  /** La partita e la mezza mossa dell'ultimo commento. */
  const last = useRef<{ gameId: string | undefined; ply: number } | null>(null);

  const startingFen = game?.startingFen ?? '';
  const cursor = game?.cursor ?? 0;
  const moves = game?.moves;
  const shownUci =
    moves === undefined
      ? ''
      : moves
          .slice(0, cursor)
          .map((move) => move.uci)
          .join(' ');
  const key = `${gameId ?? ''}|${startingFen}|${shownUci}|${String(withEngine)}`;

  useEffect(() => {
    if (!enabled || moves === undefined || gameId === undefined) {
      return;
    }
    let isCurrent = true;
    if (last.current?.gameId !== gameId) {
      seen.current = new Set();
      last.current = null;
    }
    const shown = moves.slice(0, cursor).map((move) => move.uci);
    const previous = last.current?.ply;
    const first =
      previous !== undefined && cursor > previous && cursor - previous <= MAX_COMMENTED_MOVES
        ? previous + 1
        : Math.max(cursor, 1);
    const plies =
      cursor === 0 ? [] : Array.from({ length: cursor - first + 1 }, (_, i) => first + i);

    const run = async () => {
      const comments: MoveComment[] = [];
      for (const ply of plies) {
        const record = moves[ply - 1];
        if (record === undefined) {
          continue;
        }
        const scenario = await tryWhatIf(
          startingFen,
          shown.slice(0, ply - 1),
          record.uci,
          withEngine,
        );
        comments.push({ record, scenario });
        if (!isCurrent) {
          return;
        }
      }
      const explanation = await explainPosition(startingFen, shown, withEngine);
      if (!isCurrent) {
        return;
      }
      const built = buildCoachMessages(explanation, comments, seen.current, notation);
      built.keys.forEach((messageKey) => seen.current.add(messageKey));
      last.current = { gameId, ply: cursor };
      setResult({ key, messages: built.messages, explanation });
    };
    run().catch(() => {
      // Senza risposta il tutor resta in silenzio: la partita continua normalmente.
    });
    return () => {
      isCurrent = false;
    };
    // `key` riassume posizione, partita e scelta del motore: basta lui.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [key, enabled]);

  const isReady = enabled && result?.key === key;
  return {
    messages: isReady ? result.messages : [],
    explanation: isReady ? result.explanation : null,
    isLoading: enabled && !isReady,
  };
}
