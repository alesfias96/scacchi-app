/**
 * Icon — una piccola icona a tratto, nel colore del testo che la circonda.
 *
 * Riceve:
 * - `name`: quale icona mostrare (l'elenco è in `iconPaths.ts`);
 * - `size`: facoltativo, `"normal"` (predefinito, per i pulsanti) oppure
 *   `"large"` (per le anteprime nelle Impostazioni).
 * Restituisce: l'icona. È decorativa: il nome dell'azione va scritto nel
 * pulsante che la contiene (testo o `aria-label`).
 */
import { classNames } from '../../lib/classNames';
import { ICON_PATHS, type IconName } from './iconPaths';
import styles from './Icon.module.css';

export interface IconProps {
  /** L'icona da mostrare. */
  name: IconName;
  /** La dimensione; predefinita: `"normal"`. */
  size?: 'normal' | 'large';
}

export function Icon({ name, size = 'normal' }: IconProps) {
  return (
    <svg
      className={classNames(styles.icon, size === 'large' && styles.large)}
      viewBox="0 0 24 24"
      aria-hidden="true"
      focusable="false"
    >
      {ICON_PATHS[name].map((path) => (
        <path key={path} d={path} />
      ))}
    </svg>
  );
}
