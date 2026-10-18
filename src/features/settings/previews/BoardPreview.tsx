/**
 * BoardPreview — la scacchiera di uno stile in miniatura, con la sua cornice.
 *
 * Il colore della cornice arriva da `data-board` (stessi stili della
 * scacchiera vera, in src/components/chessboard/themes/boards.css).
 *
 * Riceve:
 * - `board`: lo stile da mostrare;
 * - `image`: l'immagine della scacchiera.
 * Restituisce: l'anteprima.
 */
import type { BoardStyle } from '../../../bindings/BoardStyle';
import styles from './BoardPreview.module.css';

export interface BoardPreviewProps {
  /** Lo stile da mostrare. */
  board: BoardStyle;
  /** L'immagine della scacchiera (importata da src/assets/boards/). */
  image: string;
}

export function BoardPreview({ board, image }: BoardPreviewProps) {
  return (
    <span className={styles.frame} data-board={board}>
      <img src={image} alt="" className={styles.image} />
    </span>
  );
}
