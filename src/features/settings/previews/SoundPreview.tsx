/**
 * SoundPreview — l'altoparlante acceso o spento, per la scelta dei suoni.
 *
 * Riceve:
 * - `sounds`: la scelta da mostrare (`"on"` = suoni attivi, `"off"` = disattivati).
 * Restituisce: l'anteprima.
 */
import type { SoundSetting } from '../../../bindings/SoundSetting';
import { Icon } from '../../../components/icon/Icon';
import styles from './SoundPreview.module.css';

export interface SoundPreviewProps {
  /** La scelta da mostrare. */
  sounds: SoundSetting;
}

export function SoundPreview({ sounds }: SoundPreviewProps) {
  return (
    <span className={styles.sample}>
      <Icon name={sounds === 'on' ? 'soundOn' : 'soundOff'} size="large" />
    </span>
  );
}
