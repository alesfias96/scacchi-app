/**
 * Icon — una piccola icona a tratto, nel colore del testo che la circonda.
 *
 * Riceve:
 * - `name`: quale icona mostrare (l'elenco è in `iconPaths.ts`).
 * Restituisce: l'icona. È decorativa: il nome dell'azione va scritto nel
 * pulsante che la contiene (testo o `aria-label`).
 */
import { ICON_PATHS, type IconName } from './iconPaths';
import styles from './Icon.module.css';

export interface IconProps {
  /** L'icona da mostrare. */
  name: IconName;
}

export function Icon({ name }: IconProps) {
  return (
    <svg className={styles.icon} viewBox="0 0 24 24" aria-hidden="true" focusable="false">
      {ICON_PATHS[name].map((path) => (
        <path key={path} d={path} />
      ))}
    </svg>
  );
}
