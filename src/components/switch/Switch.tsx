/**
 * Switch — un interruttore acceso/spento, con la sua etichetta.
 *
 * Cosa fa: una normale casella di spunta HTML (quindi funziona con mouse,
 * tastiera e lettori di schermo) disegnata come un interruttore che scorre.
 *
 * Riceve:
 * - `label`: il testo accanto all'interruttore;
 * - `checked`: se è acceso;
 * - `onChange`: chiamata con il nuovo stato quando l'utente lo cambia;
 * - `disabled`: facoltativo, per renderlo non utilizzabile.
 * Restituisce: l'interruttore.
 */
import styles from './Switch.module.css';

export interface SwitchProps {
  /** Il testo accanto all'interruttore. */
  label: string;
  /** `true` se è acceso. */
  checked: boolean;
  /** Chiamata con il nuovo stato. */
  onChange: (checked: boolean) => void;
  /** `true` per renderlo non utilizzabile. */
  disabled?: boolean;
}

export function Switch({ label, checked, onChange, disabled = false }: SwitchProps) {
  return (
    <label className={styles.switch}>
      <input
        className={styles.input}
        type="checkbox"
        role="switch"
        checked={checked}
        disabled={disabled}
        onChange={(event) => {
          onChange(event.target.checked);
        }}
      />
      <span className={styles.track} aria-hidden="true">
        <span className={styles.knob} />
      </span>
      <span className={styles.label}>{label}</span>
    </label>
  );
}
