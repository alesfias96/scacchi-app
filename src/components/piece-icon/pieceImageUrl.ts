/**
 * Trova l'immagine di un pezzo in uno stile.
 *
 * Le immagini stanno in src/assets/pieces/<stile>/<codice>.svg. Vite le
 * raccoglie tutte durante la preparazione dell'app (`import.meta.glob`), così
 * non serve importarle una per una e un nuovo stile funziona appena si
 * aggiunge la sua cartella.
 */
import type { PieceStyle } from '../../bindings/PieceStyle';
import type { PieceCode } from './pieceCode';

/** Tutte le immagini dei pezzi: percorso del file → indirizzo dell'immagine. */
const PIECE_IMAGES: Record<string, string> = import.meta.glob<string>(
  '../../assets/pieces/*/*.svg',
  { eager: true, query: '?url', import: 'default' },
);

/**
 * @param style - lo stile dei pezzi (es. `"icona"`).
 * @param piece - il pezzo (es. `"wK"` per il re bianco).
 * @returns l'indirizzo dell'immagine, da usare in un tag `<img>`.
 * @throws se l'immagine non esiste (errore di programmazione: manca un file).
 */
export function pieceImageUrl(style: PieceStyle, piece: PieceCode): string {
  const url = PIECE_IMAGES[`../../assets/pieces/${style}/${piece}.svg`];
  if (url === undefined) {
    throw new Error(`Immagine mancante: pezzo ${piece} nello stile ${style}.`);
  }
  return url;
}
