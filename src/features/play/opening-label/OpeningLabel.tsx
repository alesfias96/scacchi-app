/**
 * OpeningLabel — il nome dell'apertura giocata, sopra l'elenco delle mosse.
 *
 * Cosa fa: mostra il codice ECO e il nome dell'apertura riconosciuta da Rust
 * (ad esempio "B90 · Sicilian Defense: Najdorf Variation"). Il nome è in
 * inglese, come nell'elenco di Lichess. Se l'apertura non è riconosciuta
 * (nessuna mossa, o una posizione di partenza particolare) non mostra nulla.
 *
 * Riceve:
 * - `opening`: l'apertura, oppure `null`.
 * Restituisce: l'etichetta, oppure niente.
 */
import type { Opening } from '../../../bindings/Opening';
import styles from './OpeningLabel.module.css';

export interface OpeningLabelProps {
  /** L'apertura riconosciuta, oppure `null`. */
  opening: Opening | null;
}

export function OpeningLabel({ opening }: OpeningLabelProps) {
  if (opening === null) {
    return null;
  }
  return (
    <p className={styles.label}>
      <span className={styles.eco}>{opening.eco}</span>
      <span>{opening.name}</span>
    </p>
  );
}
