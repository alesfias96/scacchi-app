/**
 * Controlla che un testo sia il nome di una casa della scacchiera.
 *
 * chessground accetta solo nomi di case validi (da `"a1"` a `"h8"`); Rust li
 * invia come semplice testo. Questa funzione fa da "dogana": dopo il
 * controllo TypeScript sa che il testo è una casa valida.
 *
 * @param text - il testo da controllare (es. `"e4"`).
 * @returns `true` se è una casa da a1 a h8.
 */
import type { Key } from '@lichess-org/chessground/types';

/** Una lettera da a ad h seguita da una cifra da 1 a 8. */
const SQUARE_PATTERN = /^[a-h][1-8]$/;

export function isSquareKey(text: string): text is Exclude<Key, 'a0'> {
  return SQUARE_PATTERN.test(text);
}
