/**
 * ChoiceGroup — un gruppo di scelte a "pastiglie", di cui se ne seleziona una.
 *
 * Cosa fa: mostra le opzioni come piccoli riquadri cliccabili, con un nome e,
 * se serve, un dettaglio sotto (ad esempio "Circolo" e "1400 Elo"). Sotto c'è
 * un normale gruppo di pulsanti di scelta (radio), quindi funziona anche con le
 * frecce della tastiera e con i lettori di schermo. È la versione compatta di
 * `OptionGroup` (che ha anche un'anteprima disegnata).
 *
 * Riceve:
 * - `legend`: il titolo del gruppo;
 * - `name`: un nome tecnico unico per il gruppo;
 * - `options`: le scelte possibili;
 * - `value`: la scelta attuale;
 * - `onChange`: chiamata con il nuovo valore quando l'utente sceglie.
 * Restituisce: il gruppo di scelte.
 */
import { classNames } from '../../lib/classNames';
import styles from './ChoiceGroup.module.css';

/** Una scelta del gruppo. */
export interface Choice<T extends string> {
  /** Il valore della scelta. */
  value: T;
  /** Il nome mostrato. */
  label: string;
  /** Un dettaglio facoltativo, scritto più piccolo sotto il nome. */
  detail?: string;
}

export interface ChoiceGroupProps<T extends string> {
  /** Il titolo del gruppo. */
  legend: string;
  /** Nome tecnico unico del gruppo (collega tra loro i pulsanti di scelta). */
  name: string;
  /** Le scelte possibili, nell'ordine in cui compaiono. */
  options: readonly Choice<T>[];
  /** La scelta attuale. */
  value: T;
  /** Chiamata con il nuovo valore quando l'utente sceglie. */
  onChange: (value: T) => void;
}

export function ChoiceGroup<T extends string>({
  legend,
  name,
  options,
  value,
  onChange,
}: ChoiceGroupProps<T>) {
  return (
    <fieldset className={styles.group}>
      <legend className={styles.legend}>{legend}</legend>
      <div className={styles.choices}>
        {options.map((option) => (
          <label
            key={option.value}
            className={classNames(styles.choice, option.value === value && styles.selected)}
          >
            <input
              className={styles.input}
              type="radio"
              name={name}
              value={option.value}
              checked={option.value === value}
              onChange={() => {
                onChange(option.value);
              }}
            />
            <span className={styles.label}>{option.label}</span>
            {option.detail !== undefined && <span className={styles.detail}>{option.detail}</span>}
          </label>
        ))}
      </div>
    </fieldset>
  );
}
