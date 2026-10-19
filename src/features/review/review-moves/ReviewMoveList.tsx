/**
 * ReviewMoveList — la scheda «Mosse» dell'analisi: la partita con i simboli.
 *
 * Cosa fa: scrive la partita come nei libri, due mezze mosse per riga, con i
 * simboli delle mosse sbagliate (?! imprecisione, ? errore, ?? errore grave)
 * colorati per gravità. Un clic su una mossa mostra la posizione dopo quella mossa.
 *
 * Riceve:
 * - `moves`: le mosse con il loro giudizio;
 * - `current`: la posizione mostrata (0 = partenza, 1 = dopo la prima mossa…);
 * - `notation`: la notazione scelta nelle Impostazioni;
 * - `onSelect`: chiamata con la posizione scelta.
 * Restituisce: la scheda.
 */
import type { MoveReview } from '../../../bindings/MoveReview';
import type { Notation } from '../../../bindings/Notation';
import { PanelCard } from '../../../components/panel-card/PanelCard';
import { classNames } from '../../../lib/classNames';
import { formatSan } from '../../../lib/formatSan';
import { qualitySeverity, qualitySymbol } from '../describeQuality';
import styles from './ReviewMoveList.module.css';

export interface ReviewMoveListProps {
  moves: readonly MoveReview[];
  current: number;
  notation: Notation;
  onSelect: (position: number) => void;
}

/** Le mosse divise in righe: il numero, la mossa del Bianco e quella del Nero. */
function rows(moves: readonly MoveReview[]) {
  const result: { number: number; white?: MoveReview; black?: MoveReview }[] = [];
  for (const move of moves) {
    const last = result[result.length - 1];
    if (move.side === 'white' || last?.number !== move.moveNumber) {
      result.push({ number: move.moveNumber });
    }
    const row = result[result.length - 1];
    if (row !== undefined) {
      row[move.side] = move;
    }
  }
  return result;
}

export function ReviewMoveList({ moves, current, notation, onSelect }: ReviewMoveListProps) {
  /** Una mossa cliccabile, oppure uno spazio vuoto. */
  const cell = (move: MoveReview | undefined) => {
    if (move === undefined) {
      return <span />;
    }
    const severity = qualitySeverity(move.quality);
    return (
      <button
        type="button"
        className={classNames(
          styles.move,
          move.ply === current && styles.current,
          severity !== null && styles[severity],
        )}
        aria-current={move.ply === current ? 'step' : undefined}
        onClick={() => {
          onSelect(move.ply);
        }}
      >
        {formatSan(move.san, notation)}
        {qualitySymbol(move.quality)}
      </button>
    );
  };

  return (
    <PanelCard title="Mosse">
      <ol className={styles.list}>
        {rows(moves).map((row) => (
          <li key={row.number} className={styles.row}>
            <span className={styles.number}>{row.number}.</span>
            {cell(row.white)}
            {cell(row.black)}
          </li>
        ))}
      </ol>
    </PanelCard>
  );
}
