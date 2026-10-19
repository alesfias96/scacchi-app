/**
 * Prepara i segni da disegnare sulla scacchiera (frecce e cerchi) nel formato di chessground.
 *
 * - La freccia della **mossa migliore** (analisi del motore) è verde tenue.
 * - I **segni delle osservazioni** (le case di un fatto, ad esempio la
 *   forchetta indicata con il mouse) sono blu: frecce e cerchi.
 * Le case non valide vengono ignorate.
 *
 * @param bestMove - la mossa migliore da indicare, oppure `null`.
 * @param marks - i segni delle osservazioni, oppure `null`.
 * @returns l'elenco dei segni per `drawable.autoShapes`.
 */
import type { DrawShape } from '@lichess-org/chessground/draw';
import type { LegalMove } from '../../bindings/LegalMove';
import { isSquareKey } from './isSquareKey';

/** I segni delle osservazioni: frecce e cerchi. */
export interface BoardMarks {
  /** Le frecce, dalla casa `from` alla casa `to`. */
  arrows: readonly { from: string; to: string }[];
  /** Le case da cerchiare. */
  circles: readonly string[];
}

/** I "pennelli" già pronti in chessground. */
const BEST_MOVE_BRUSH = 'paleGreen';
const MARK_BRUSH = 'blue';

export function buildShapes(bestMove: LegalMove | null, marks: BoardMarks | null): DrawShape[] {
  const shapes: DrawShape[] = [];
  if (bestMove !== null && isSquareKey(bestMove.from) && isSquareKey(bestMove.to)) {
    shapes.push({ orig: bestMove.from, dest: bestMove.to, brush: BEST_MOVE_BRUSH });
  }
  for (const arrow of marks?.arrows ?? []) {
    if (isSquareKey(arrow.from) && isSquareKey(arrow.to)) {
      shapes.push({ orig: arrow.from, dest: arrow.to, brush: MARK_BRUSH });
    }
  }
  for (const square of marks?.circles ?? []) {
    if (isSquareKey(square)) {
      shapes.push({ orig: square, brush: MARK_BRUSH });
    }
  }
  return shapes;
}
