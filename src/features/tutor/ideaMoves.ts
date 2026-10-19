/**
 * Le mosse di un'idea scritte come nei libri, nella notazione scelta nelle Impostazioni.
 *
 * Rust non scrive le mosse dentro le frasi delle idee: dà il numero della
 * mezza mossa di ogni momento, e qui si aggiunge la mossa ("12. ♞c6+").
 */
import type { IdeaView } from '../../bindings/IdeaView';
import type { Notation } from '../../bindings/Notation';
import { formatSan } from '../../lib/formatSan';

/**
 * La mossa della variante dopo `ply` mezze mosse, con il suo numero.
 *
 * @param idea - l'idea.
 * @param ply - la mezza mossa della variante (0 = la prima mossa).
 * @param notation - la notazione scelta nelle Impostazioni.
 * @returns ad esempio `"12. ♞c6+"` (mossa del Bianco) o `"12… e5"` (del Nero);
 *   stringa vuota se la variante è più corta.
 */
export function ideaMoveLabel(idea: IdeaView, ply: number, notation: Notation): string {
  const san = idea.moves[ply];
  if (san === undefined) {
    return '';
  }
  // Quante mezze mosse dalla prima del Bianco del numero `firstMoveNumber`.
  const offset = ply + (idea.firstSide === 'black' ? 1 : 0);
  const number = idea.firstMoveNumber + Math.floor(offset / 2);
  const whiteMoves = offset % 2 === 0;
  return `${String(number)}${whiteMoves ? '.' : '…'} ${formatSan(san, notation)}`;
}
