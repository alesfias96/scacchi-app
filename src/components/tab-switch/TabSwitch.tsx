/**
 * TabSwitch — un selettore a schede: due o più voci affiancate, una attiva.
 *
 * Cosa fa: mostra le voci come pulsanti uniti (la voce attiva è evidenziata).
 * Un clic cambia voce; per i lettori di schermo è un elenco di schede
 * (`tablist`). Serve, ad esempio, per passare da «Partita» a «Tutor» in cima
 * alla colonna di destra.
 *
 * Riceve:
 * - `label`: cosa sceglie il selettore (letto dai lettori di schermo);
 * - `options`: le voci (valore e testo);
 * - `value`: la voce attiva;
 * - `onChange`: chiamata con la voce scelta.
 * Restituisce: il selettore.
 */
import { classNames } from '../../lib/classNames';
import styles from './TabSwitch.module.css';

/** Una voce del selettore. */
export interface TabOption<T extends string> {
  value: T;
  label: string;
}

export interface TabSwitchProps<T extends string> {
  label: string;
  options: readonly TabOption<T>[];
  value: T;
  onChange: (value: T) => void;
}

export function TabSwitch<T extends string>({
  label,
  options,
  value,
  onChange,
}: TabSwitchProps<T>) {
  return (
    <div className={styles.switch} role="tablist" aria-label={label}>
      {options.map((option) => (
        <button
          key={option.value}
          type="button"
          role="tab"
          aria-selected={option.value === value}
          className={classNames(styles.tab, option.value === value && styles.active)}
          onClick={() => {
            onChange(option.value);
          }}
        >
          {option.label}
        </button>
      ))}
    </div>
  );
}
