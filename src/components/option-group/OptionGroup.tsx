/**
 * OptionGroup — un gruppo di scelte a riquadri, di cui se ne seleziona una.
 *
 * Cosa fa: mostra le opzioni come riquadri con un'anteprima, un nome e una
 * breve descrizione. Sotto c'è un normale gruppo di pulsanti di scelta
 * (radio), quindi funziona anche con la tastiera e con i lettori di schermo.
 *
 * Riceve:
 * - `legend`: il titolo del gruppo;
 * - `name`: un nome tecnico unico per il gruppo;
 * - `options`: le scelte possibili;
 * - `value`: la scelta attuale;
 * - `onChange`: chiamata con il nuovo valore quando l'utente sceglie.
 * Restituisce: il gruppo di scelte.
 */
import type { ReactNode } from 'react';
import { classNames } from '../../lib/classNames';
import styles from './OptionGroup.module.css';

/** Una scelta del gruppo. */
export interface Option<T extends string> {
  /** Il valore salvato quando questa scelta è selezionata. */
  value: T;
  /** Il nome mostrato. */
  label: string;
  /** Una breve descrizione, facoltativa. */
  description?: string;
  /** L'anteprima disegnata nel riquadro. */
  preview: ReactNode;
}

export interface OptionGroupProps<T extends string> {
  /** Il titolo del gruppo. */
  legend: string;
  /** Nome tecnico unico del gruppo (collega tra loro i pulsanti di scelta). */
  name: string;
  /** Le scelte possibili, nell'ordine in cui compaiono. */
  options: readonly Option<T>[];
  /** La scelta attuale. */
  value: T;
  /** Chiamata con il nuovo valore quando l'utente sceglie. */
  onChange: (value: T) => void;
}

export function OptionGroup<T extends string>({
  legend,
  name,
  options,
  value,
  onChange,
}: OptionGroupProps<T>) {
  return (
    <fieldset className={styles.group}>
      <legend className={styles.legend}>{legend}</legend>
      <div className={styles.options}>
        {options.map((option) => {
          const inputId = `${name}-${option.value}`;
          const isSelected = option.value === value;
          return (
            <label
              key={option.value}
              htmlFor={inputId}
              className={classNames(styles.option, isSelected && styles.selected)}
            >
              <input
                id={inputId}
                type="radio"
                name={name}
                value={option.value}
                checked={isSelected}
                onChange={() => {
                  onChange(option.value);
                }}
                className={styles.input}
              />
              <span className={styles.preview} aria-hidden="true">
                {option.preview}
              </span>
              <span className={styles.label}>{option.label}</span>
              {option.description !== undefined && (
                <span className={styles.description}>{option.description}</span>
              )}
            </label>
          );
        })}
      </div>
    </fieldset>
  );
}
