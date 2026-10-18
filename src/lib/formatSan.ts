/**
 * Scrive una mossa nella notazione scelta dall'utente.
 *
 * Rust invia le mosse sempre in notazione internazionale (`Nf3`, `Qxd8+`,
 * `e8=Q`). Le iniziali dei pezzi sono le uniche lettere maiuscole della
 * mossa (le colonne sono minuscole e l'arrocco `O-O` usa la lettera O, che
 * non è un pezzo), quindi basta sostituire quelle lettere:
 * - figurine: `♞f3`, `♛xd8+`, `e8=♛`;
 * - italiana: `Cf3`, `Dxd8+`, `e8=D` (R re, D donna, T torre, A alfiere, C cavallo);
 * - internazionale: la mossa resta com'è.
 *
 * @param san - la mossa in notazione internazionale.
 * @param notation - la notazione scelta nelle Impostazioni.
 * @returns la mossa da mostrare.
 */
import type { Notation } from '../bindings/Notation';

/** Le iniziali internazionali dei pezzi. */
type PieceLetter = 'K' | 'Q' | 'R' | 'B' | 'N';

/** Le figurine (in versione piena, più leggibile a piccole dimensioni). */
const FIGURINES: Record<PieceLetter, string> = { K: '♚', Q: '♛', R: '♜', B: '♝', N: '♞' };

/** Le iniziali italiane. */
const ITALIAN_LETTERS: Record<PieceLetter, string> = { K: 'R', Q: 'D', R: 'T', B: 'A', N: 'C' };

/** Le lettere da sostituire: solo le iniziali dei pezzi. */
const PIECE_LETTER_PATTERN = /[KQRBN]/g;

export function formatSan(san: string, notation: Notation): string {
  if (notation === 'internazionale') {
    return san;
  }
  const letters = notation === 'figurine' ? FIGURINES : ITALIAN_LETTERS;
  return san.replace(PIECE_LETTER_PATTERN, (letter) => letters[letter as PieceLetter]);
}
