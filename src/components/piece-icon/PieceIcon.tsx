/**
 * PieceIcon — l'immagine di un pezzo nello stile scelto nelle Impostazioni.
 *
 * Cosa fa: mostra un pezzo fuori dalla scacchiera (ad esempio nella scelta
 * della promozione). Se non si indica uno stile usa quello dell'utente,
 * così tutta l'app mostra sempre gli stessi pezzi.
 *
 * Riceve:
 * - `piece`: il pezzo da mostrare (es. `"wQ"`);
 * - `style`: facoltativo, uno stile diverso da quello scelto (usato nelle anteprime);
 * - `label`: facoltativo, il nome letto dai lettori di schermo; se manca
 *   l'immagine è considerata decorativa.
 * Restituisce: l'immagine, che riempie lo spazio del suo contenitore.
 */
import type { PieceStyle } from '../../bindings/PieceStyle';
import { useLoadedSettings } from '../../state/settingsStore';
import type { PieceCode } from './pieceCode';
import { pieceImageUrl } from './pieceImageUrl';
import styles from './PieceIcon.module.css';

export interface PieceIconProps {
  /** Il pezzo da mostrare. */
  piece: PieceCode;
  /** Lo stile da usare; predefinito: quello scelto nelle Impostazioni. */
  style?: PieceStyle;
  /** Il nome per i lettori di schermo; predefinito: immagine decorativa. */
  label?: string;
}

export function PieceIcon({ piece, style, label = '' }: PieceIconProps) {
  const { pieces } = useLoadedSettings();
  return <img src={pieceImageUrl(style ?? pieces, piece)} alt={label} className={styles.icon} />;
}
