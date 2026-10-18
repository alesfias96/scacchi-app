/**
 * Raggruppa le mosse in righe, come nei libri: "1. e4 e5", "2. Cf3 Cc6", ...
 *
 * Ogni riga ha il numero della mossa, la mossa del bianco e quella del nero.
 * Se la partita comincia con il nero (posizione incollata), la prima riga
 * non ha la mossa del bianco; l'ultima riga può non avere quella del nero.
 *
 * @param moves - le mosse della partita, in ordine.
 * @returns le righe da mostrare.
 */
import type { MoveRecord } from '../../../bindings/MoveRecord';

/** Una riga della lista: numero, mossa del bianco, mossa del nero. */
export interface MoveRow {
  /** Il numero della mossa (1, 2, 3, ...). */
  moveNumber: number;
  /** La mossa del bianco, oppure `null` se manca. */
  white: MoveRecord | null;
  /** La mossa del nero, oppure `null` se manca. */
  black: MoveRecord | null;
}

export function groupMovesIntoRows(moves: readonly MoveRecord[]): MoveRow[] {
  const rows: MoveRow[] = [];
  for (const move of moves) {
    const lastRow = rows[rows.length - 1];
    // Il nero completa la riga del bianco con lo stesso numero, se c'è.
    if (move.side === 'black' && lastRow?.moveNumber === move.moveNumber) {
      lastRow.black = move;
    } else if (move.side === 'white') {
      rows.push({ moveNumber: move.moveNumber, white: move, black: null });
    } else {
      rows.push({ moveNumber: move.moveNumber, white: null, black: move });
    }
  }
  return rows;
}
