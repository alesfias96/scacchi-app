/**
 * EvalBar — la barra di valutazione accanto alla scacchiera.
 *
 * Cosa fa: una colonna verticale divisa tra bianco e nero. Più il Bianco sta
 * meglio, più la parte bianca cresce (e viceversa); la divisione scorre
 * dolcemente a ogni nuovo giudizio del motore. Il valore (`+0.4`, `M3`)
 * è scritto all'estremità di chi è in vantaggio.
 *
 * Il disegno è un'immagine SVG: l'altezza delle due parti è un attributo del
 * disegno, così non servono stili scritti nel codice.
 *
 * Riceve:
 * - `evaluation`: la valutazione dal punto di vista del Bianco, oppure `null`
 *   mentre si aspetta il primo giudizio (la barra resta a metà);
 * - `orientation`: il colore mostrato in basso sulla scacchiera; la parte
 *   di quel colore parte dal basso, come i suoi pezzi.
 * Restituisce: la barra.
 */
import type { Evaluation } from '../../bindings/Evaluation';
import type { Side } from '../../bindings/Side';
import { classNames } from '../../lib/classNames';
import styles from './EvalBar.module.css';
import { formatEvaluation } from '../../lib/formatEvaluation';
import { whiteShare } from './whiteShare';

/** L'altezza del disegno, in unità SVG (la barra si allunga fino a riempire lo spazio). */
const HEIGHT = 1000;

export interface EvalBarProps {
  /** La valutazione per il Bianco, oppure `null` se non è ancora arrivata. */
  evaluation: Evaluation | null;
  /** Il colore in basso sulla scacchiera. */
  orientation: Side;
}

export function EvalBar({ evaluation, orientation }: EvalBarProps) {
  const share = evaluation === null ? 0.5 : whiteShare(evaluation);
  const whiteHeight = Math.round(share * HEIGHT);
  const whiteAtBottom = orientation === 'white';
  const whiteLeads = share >= 0.5;
  const label = evaluation === null ? '…' : formatEvaluation(evaluation);
  /** Il valore sta dalla parte di chi è in vantaggio: in basso se è il colore in basso. */
  const labelAtBottom = whiteLeads === whiteAtBottom;

  return (
    <div
      className={styles.bar}
      role="meter"
      aria-label="Valutazione della posizione"
      aria-valuemin={0}
      aria-valuemax={100}
      aria-valuenow={Math.round(share * 100)}
      aria-valuetext={label}
    >
      <svg
        className={styles.drawing}
        viewBox={`0 0 1 ${String(HEIGHT)}`}
        preserveAspectRatio="none"
        aria-hidden="true"
      >
        <rect className={styles.black} x={0} y={0} width={1} height={HEIGHT} />
        <rect
          className={styles.white}
          x={0}
          y={whiteAtBottom ? HEIGHT - whiteHeight : 0}
          width={1}
          height={whiteHeight}
        />
      </svg>
      <span
        className={classNames(
          styles.label,
          labelAtBottom ? styles.labelBottom : styles.labelTop,
          whiteLeads ? styles.onWhite : styles.onBlack,
        )}
        aria-hidden="true"
      >
        {label}
      </span>
    </div>
  );
}
