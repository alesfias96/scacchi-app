/**
 * Scrive una sequenza di mosse come nei libri, con i numeri di mossa (la
 * variante del motore, la soluzione di un puzzle, le mosse giocate).
 *
 * Esempi (notazione a figurine):
 * - variante che comincia con il Bianco: `12. ♞f3 e5 13. ♝c4`;
 * - variante che comincia con il Nero:   `12… e5 13. ♞f3`.
 *
 * Rust ha già scritto le mosse in notazione SAN; qui si aggiungono solo i
 * numeri e si traduce la notazione scelta nelle Impostazioni.
 *
 * @param line - la sequenza: numero di mossa e colore della prima mossa, mosse in SAN.
 * @param notation - la notazione scelta nelle Impostazioni.
 * @param maxMoves - quante mezze mosse mostrare al massimo (le varianti lunghe si accorciano).
 * @returns la variante come testo.
 */
import type { LineView } from '../bindings/LineView';
import type { Notation } from '../bindings/Notation';
import { formatSan } from './formatSan';

/** Ciò che serve di una sequenza di mosse per scriverla. */
export type MoveSequence = Pick<LineView, 'moveNumber' | 'turn' | 'san'>;

export function formatLine(line: MoveSequence, notation: Notation, maxMoves: number): string {
  const parts: string[] = [];
  let moveNumber = line.moveNumber;
  let whiteToMove = line.turn === 'white';
  line.san.slice(0, maxMoves).forEach((san, index) => {
    if (whiteToMove) {
      parts.push(`${String(moveNumber)}.`);
    } else if (index === 0) {
      parts.push(`${String(moveNumber)}…`);
    }
    parts.push(formatSan(san, notation));
    if (!whiteToMove) {
      moveNumber += 1;
    }
    whiteToMove = !whiteToMove;
  });
  return parts.join(' ');
}
