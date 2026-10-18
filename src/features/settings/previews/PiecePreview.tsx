/**
 * PiecePreview — tre pezzi di uno stile, su case alternate chiare e scure.
 *
 * Riceve:
 * - `images`: le immagini dei pezzi da mostrare (re bianco, regina nera, cavallo bianco).
 * Restituisce: l'anteprima.
 */
import styles from './PiecePreview.module.css';

export interface PiecePreviewProps {
  /** Le immagini dei pezzi da mostrare, nell'ordine. */
  images: readonly string[];
}

export function PiecePreview({ images }: PiecePreviewProps) {
  return (
    <span className={styles.strip}>
      {images.map((image) => (
        <span key={image} className={styles.square}>
          <img src={image} alt="" className={styles.piece} />
        </span>
      ))}
    </span>
  );
}
