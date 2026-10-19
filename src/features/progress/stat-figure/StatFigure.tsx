/**
 * StatFigure — un numero in evidenza, con il suo nome e una nota (es. «Elo stimato 1450»).
 *
 * Riceve:
 * - `label`: il nome del numero;
 * - `value`: il numero già scritto (es. "1450", "72%", "—");
 * - `note`: una spiegazione facoltativa, più piccola, sotto il numero.
 * Restituisce: il riquadro.
 */
import styles from './StatFigure.module.css';

export interface StatFigureProps {
  label: string;
  value: string;
  note?: string;
}

export function StatFigure({ label, value, note }: StatFigureProps) {
  return (
    <div className={styles.figure}>
      <p className={styles.label}>{label}</p>
      <p className={styles.value}>{value}</p>
      {note !== undefined && <p className={styles.note}>{note}</p>}
    </div>
  );
}
