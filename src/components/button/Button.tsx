/**
 * Button — il pulsante dell'app, con lo stesso aspetto in ogni sezione.
 *
 * Cosa fa: un normale pulsante HTML con tre aspetti possibili:
 * - `primary`: pieno, nel colore d'accento del tema (l'azione principale);
 * - `secondary`: tenue, per le azioni comuni (è il predefinito);
 * - `icon`: quadrato, con la sola icona (serve sempre `aria-label`, letto
 *   dai lettori di schermo e mostrato come suggerimento al passaggio del mouse).
 *
 * Riceve: tutte le proprietà di un normale `<button>`, più `variant`.
 * Restituisce: il pulsante (di tipo "button", quindi non invia moduli per sbaglio).
 */
import type { ButtonHTMLAttributes } from 'react';
import { classNames } from '../../lib/classNames';
import styles from './Button.module.css';

/** I tre aspetti possibili del pulsante. */
export type ButtonVariant = 'primary' | 'secondary' | 'icon';

export interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  /** L'aspetto del pulsante. Predefinito: `"secondary"`. */
  variant?: ButtonVariant;
}

export function Button({ variant = 'secondary', className, title, ...props }: ButtonProps) {
  return (
    <button
      type="button"
      // Per i pulsanti con sola icona il nome diventa anche il suggerimento.
      title={title ?? (variant === 'icon' ? props['aria-label'] : undefined)}
      className={classNames(styles.button, styles[variant], className)}
      {...props}
    />
  );
}
