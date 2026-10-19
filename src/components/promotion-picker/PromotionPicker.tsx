/**
 * PromotionPicker — la scelta del pezzo quando un pedone arriva in fondo.
 *
 * Cosa fa: copre la scacchiera con un velo e mostra quattro pezzi (donna,
 * torre, alfiere, cavallo) del colore del pedone, nello stile scelto
 * nelle Impostazioni. Un clic sceglie il pezzo; Esc o un clic fuori dai
 * pezzi annullano la mossa.
 *
 * Riceve:
 * - `side`: il colore del pedone che viene promosso;
 * - `onChoose`: chiamata con la lettera UCI del pezzo scelto (`q`, `r`, `b`, `n`);
 * - `onCancel`: chiamata se l'utente rinuncia.
 * Restituisce: la finestra di scelta (da mettere sopra la scacchiera).
 */
import { useEffect } from 'react';
import type { Side } from '../../bindings/Side';
import type { PieceRole } from '../piece-icon/pieceCode';
import { PieceIcon } from '../piece-icon/PieceIcon';
import styles from './PromotionPicker.module.css';

/** La lettera UCI del pezzo scelto, aggiunta in fondo alla mossa (es. `e7e8q`). */
export type PromotionPiece = 'q' | 'r' | 'b' | 'n';

/** I quattro pezzi possibili, con il loro nome e il loro disegno. */
const CHOICES: readonly { piece: PromotionPiece; role: PieceRole; label: string }[] = [
  { piece: 'q', role: 'Q', label: 'Donna' },
  { piece: 'r', role: 'R', label: 'Torre' },
  { piece: 'b', role: 'B', label: 'Alfiere' },
  { piece: 'n', role: 'N', label: 'Cavallo' },
];

export interface PromotionPickerProps {
  /** Il colore del pedone promosso. */
  side: Side;
  /** Chiamata con il pezzo scelto. */
  onChoose: (piece: PromotionPiece) => void;
  /** Chiamata se l'utente rinuncia alla mossa. */
  onCancel: () => void;
}

export function PromotionPicker({ side, onChoose, onCancel }: PromotionPickerProps) {
  // Esc annulla la scelta.
  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === 'Escape') {
        onCancel();
      }
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => {
      window.removeEventListener('keydown', handleKeyDown);
    };
  }, [onCancel]);

  /** `w` o `b`: la prima lettera del codice dei pezzi. */
  const color = side === 'white' ? 'w' : 'b';

  return (
    // Un clic sul velo (fuori dai pezzi) annulla; la tastiera usa Esc.
    <div className={styles.backdrop} onClick={onCancel} role="presentation">
      <div
        className={styles.dialog}
        role="dialog"
        aria-modal="true"
        aria-label="Scegli il pezzo della promozione"
        onClick={(event) => {
          event.stopPropagation();
        }}
      >
        <p className={styles.title}>Promuovi a</p>
        <div className={styles.choices}>
          {CHOICES.map(({ piece, role, label }, index) => (
            <button
              key={piece}
              type="button"
              className={styles.choice}
              aria-label={label}
              title={label}
              // Il primo pezzo (la donna, la scelta più comune) riceve subito il focus:
              // basta premere Invio per confermarla.
              autoFocus={index === 0}
              onClick={() => {
                onChoose(piece);
              }}
            >
              <PieceIcon piece={`${color}${role}`} />
            </button>
          ))}
        </div>
      </div>
    </div>
  );
}
