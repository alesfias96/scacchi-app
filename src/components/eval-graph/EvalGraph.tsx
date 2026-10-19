/**
 * EvalGraph — il grafico della partita: la probabilità di vittoria mossa per mossa.
 *
 * Cosa fa: disegna una striscia orizzontale. In ogni punto la parte chiara
 * (sotto) è la probabilità di vittoria del Bianco, quella scura (sopra) quella
 * del Nero; la linea a metà è la parità. Le imprecisioni, gli errori e gli
 * errori gravi sono pallini colorati. Una linea verticale indica la posizione
 * mostrata. Un clic (o le frecce della tastiera, con il grafico selezionato)
 * porta alla mossa in quel punto.
 *
 * Riceve:
 * - `chances`: la probabilità del Bianco (0–100) in ogni posizione, dalla partenza;
 * - `marks`: le mosse da segnare, con la loro gravità (`ply` = mezza mossa);
 * - `current`: la posizione mostrata (0 = partenza);
 * - `onSelect`: chiamata con la posizione scelta.
 * Restituisce: il grafico.
 */
import type { KeyboardEvent, MouseEvent } from 'react';
import { classNames } from '../../lib/classNames';
import styles from './EvalGraph.module.css';

/** Una mossa da segnare sul grafico. */
export interface GraphMark {
  ply: number;
  severity: 'inaccuracy' | 'mistake' | 'blunder';
}

export interface EvalGraphProps {
  chances: readonly number[];
  marks: readonly GraphMark[];
  current: number;
  onSelect: (position: number) => void;
}

/** La larghezza e l'altezza del disegno (il grafico si allarga quanto il contenitore). */
const WIDTH = 1000;
const HEIGHT = 100;

export function EvalGraph({ chances, marks, current, onSelect }: EvalGraphProps) {
  const last = Math.max(chances.length - 1, 1);
  const x = (position: number) => (position / last) * WIDTH;
  const y = (chance: number) => HEIGHT - chance;
  const points = chances.map((chance, position) => `${String(x(position))},${String(y(chance))}`);
  const area = `0,${String(HEIGHT)} ${points.join(' ')} ${String(WIDTH)},${String(HEIGHT)}`;

  /** La posizione più vicina al punto cliccato. */
  const handleClick = (event: MouseEvent<SVGSVGElement>) => {
    const box = event.currentTarget.getBoundingClientRect();
    const ratio = box.width === 0 ? 0 : (event.clientX - box.left) / box.width;
    onSelect(Math.min(last, Math.max(0, Math.round(ratio * last))));
  };

  const handleKeyDown = (event: KeyboardEvent<SVGSVGElement>) => {
    if (event.key === 'ArrowLeft' && current > 0) {
      onSelect(current - 1);
    } else if (event.key === 'ArrowRight' && current < chances.length - 1) {
      onSelect(current + 1);
    }
  };

  return (
    <svg
      className={styles.graph}
      viewBox={`0 0 ${String(WIDTH)} ${String(HEIGHT)}`}
      preserveAspectRatio="none"
      role="slider"
      aria-label="Grafico della partita"
      aria-valuemin={0}
      aria-valuemax={chances.length - 1}
      aria-valuenow={current}
      tabIndex={0}
      onClick={handleClick}
      onKeyDown={handleKeyDown}
    >
      <rect className={styles.black} x={0} y={0} width={WIDTH} height={HEIGHT} />
      <polygon className={styles.white} points={area} />
      <line className={styles.middle} x1={0} y1={HEIGHT / 2} x2={WIDTH} y2={HEIGHT / 2} />
      <line className={styles.current} x1={x(current)} y1={0} x2={x(current)} y2={HEIGHT} />
      {marks.map((mark) => (
        <circle
          key={mark.ply}
          className={classNames(styles.mark, styles[mark.severity])}
          cx={x(mark.ply)}
          cy={y(chances[mark.ply] ?? 50)}
          r={3}
        />
      ))}
    </svg>
  );
}
