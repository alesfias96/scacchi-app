/**
 * Il codice di un pezzo, come nei nomi dei file delle immagini.
 *
 * È formato da due lettere: il colore (`w` bianco, `b` nero) e il pezzo in
 * notazione internazionale (`K` re, `Q` donna, `R` torre, `B` alfiere,
 * `N` cavallo, `P` pedone). Ad esempio `wK` è il re bianco, `bN` il cavallo nero.
 */

/** Il colore del pezzo: `w` bianco, `b` nero. */
export type PieceColor = 'w' | 'b';

/** Il tipo di pezzo, con l'iniziale internazionale. */
export type PieceRole = 'K' | 'Q' | 'R' | 'B' | 'N' | 'P';

/** Il codice completo del pezzo (es. `"wK"`). */
export type PieceCode = `${PieceColor}${PieceRole}`;
