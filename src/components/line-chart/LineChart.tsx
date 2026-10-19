/**
 * LineChart — un grafico a linea nel tempo (es. il rating dei puzzle dopo ogni tentativo).
 *
 * Cosa fa: disegna i punti uniti da una linea, da sinistra (il più vecchio) a
 * destra (il più recente), con una griglia leggera. A sinistra scrive il valore
 * più alto e quello più basso, sotto la data del primo e dell'ultimo punto.
 * Con meno di due punti non c'è una linea da disegnare: mostra un messaggio.
 * È disegnato con SVG, senza librerie, con i colori del tema.
 *
 * Riceve:
 * - `label`: cosa mostra il grafico (letto dai lettori di schermo);
 * - `points`: i punti (istante e valore), dal più vecchio;
 * - `formatValue`: come scrivere un valore (predefinito: il numero arrotondato).
 * Restituisce: il grafico.
 */
import type { ChartPoint } from '../../bindings/ChartPoint';
import styles from './LineChart.module.css';

export interface LineChartProps {
  label: string;
  points: readonly ChartPoint[];
  formatValue?: (value: number) => string;
}

/** La larghezza e l'altezza del disegno (il grafico si allarga quanto il contenitore). */
const WIDTH = 1000;
const HEIGHT = 100;
/** Lo spazio lasciato sopra e sotto la linea, perché non tocchi i bordi. */
const MARGIN = 6;

/** Il formato delle date sotto il grafico: "18 ott 2026". */
const DATE_FORMAT = new Intl.DateTimeFormat('it-IT', { dateStyle: 'medium' });

/** Il valore arrotondato all'intero. */
function roundValue(value: number): string {
  return String(Math.round(value));
}

export function LineChart({ label, points, formatValue = roundValue }: LineChartProps) {
  const first = points[0];
  const last = points[points.length - 1];
  if (points.length < 2 || first === undefined || last === undefined) {
    return <p className={styles.empty}>Servono almeno due risultati per disegnare il grafico.</p>;
  }

  const values = points.map((point) => point.value);
  const highest = Math.max(...values);
  const lowest = Math.min(...values);
  // I punti sono messi a distanza uguale: conta l'ordine, non quanto tempo è passato.
  const x = (index: number) => (index / (points.length - 1)) * WIDTH;
  const y = (value: number) =>
    highest === lowest
      ? HEIGHT / 2
      : MARGIN + ((highest - value) / (highest - lowest)) * (HEIGHT - 2 * MARGIN);
  const line = points
    .map((point, index) => `${String(x(index))},${String(y(point.value))}`)
    .join(' ');

  return (
    <figure className={styles.chart}>
      <div className={styles.values} aria-hidden="true">
        <span>{formatValue(highest)}</span>
        <span>{formatValue(lowest)}</span>
      </div>
      <svg
        className={styles.drawing}
        viewBox={`0 0 ${String(WIDTH)} ${String(HEIGHT)}`}
        preserveAspectRatio="none"
        role="img"
        aria-label={`${label}: da ${formatValue(first.value)} a ${formatValue(last.value)}`}
      >
        <line className={styles.grid} x1={0} y1={MARGIN} x2={WIDTH} y2={MARGIN} />
        <line className={styles.grid} x1={0} y1={HEIGHT / 2} x2={WIDTH} y2={HEIGHT / 2} />
        <line className={styles.grid} x1={0} y1={HEIGHT - MARGIN} x2={WIDTH} y2={HEIGHT - MARGIN} />
        <polyline className={styles.line} points={line} />
      </svg>
      <figcaption className={styles.dates}>
        <span>{DATE_FORMAT.format(new Date(first.timeMs))}</span>
        <span>{DATE_FORMAT.format(new Date(last.timeMs))}</span>
      </figcaption>
    </figure>
  );
}
