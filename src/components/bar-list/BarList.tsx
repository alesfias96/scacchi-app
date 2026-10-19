/**
 * BarList — un elenco di barre orizzontali, una per riga (es. i risultati per livello).
 *
 * Cosa fa: per ogni voce scrive il nome, disegna la barra e, a destra, un
 * dettaglio in parole (es. "3 vinte · 1 patta · 2 perse"). Una barra può avere
 * più tratti affiancati, ciascuno con il suo colore: così una sola barra
 * mostra vittorie, patte e sconfitte. Le lunghezze sono percentuali (0–100)
 * della barra intera. È disegnato con SVG, con i colori del tema.
 *
 * Riceve:
 * - `label`: cosa mostra l'elenco (letto dai lettori di schermo);
 * - `items`: le voci, nell'ordine in cui compaiono.
 * Restituisce: l'elenco.
 */
import { classNames } from '../../lib/classNames';
import styles from './BarList.module.css';

/** Il colore di un tratto: buono (verde), neutro (grigio), cattivo (rosso), d'accento. */
export type BarTone = 'good' | 'neutral' | 'bad' | 'accent';

/** Un tratto della barra. */
export interface BarSegment {
  /** La lunghezza, in percentuale della barra intera (0–100). */
  percent: number;
  tone: BarTone;
}

/** Una voce dell'elenco. */
export interface BarItem {
  /** Un nome tecnico unico (usato come chiave da React). */
  key: string;
  /** Il nome mostrato. */
  label: string;
  /** Il dettaglio in parole, a destra della barra (letto anche dai lettori di schermo). */
  detail: string;
  /** I tratti, da sinistra. */
  segments: readonly BarSegment[];
}

export interface BarListProps {
  label: string;
  items: readonly BarItem[];
}

/** Una percentuale tra 0 e 100. */
function clamp(percent: number): number {
  return Math.min(100, Math.max(0, percent));
}

/** Dove comincia ogni tratto: subito dopo la fine dei precedenti. */
function placeSegments(segments: readonly BarSegment[]) {
  let start = 0;
  return segments.map((segment) => {
    const width = Math.min(clamp(segment.percent), 100 - start);
    const placed = { ...segment, start, width };
    start += width;
    return placed;
  });
}

export function BarList({ label, items }: BarListProps) {
  return (
    <ul className={styles.list} aria-label={label}>
      {items.map((item) => (
        <li key={item.key} className={styles.item}>
          <span className={styles.label}>{item.label}</span>
          <svg className={styles.bar} aria-hidden="true">
            <rect className={styles.track} x="0" y="0" width="100%" height="100%" />
            {placeSegments(item.segments).map((segment) => (
              <rect
                key={`${segment.tone}-${String(segment.start)}`}
                className={classNames(styles.segment, styles[segment.tone])}
                x={`${String(segment.start)}%`}
                y="0"
                width={`${String(segment.width)}%`}
                height="100%"
              />
            ))}
          </svg>
          <span className={styles.detail}>{item.detail}</span>
        </li>
      ))}
    </ul>
  );
}
