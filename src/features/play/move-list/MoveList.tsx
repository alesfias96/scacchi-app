/**
 * MoveList — l'elenco delle mosse della partita, come nei libri.
 *
 * Cosa fa: mostra le mosse in righe ("1. e4 e5") nella notazione scelta
 * nelle Impostazioni. La mossa della posizione mostrata è evidenziata; un
 * clic su una mossa mostra la posizione dopo quella mossa. Quando la mossa
 * evidenziata cambia, l'elenco scorre per tenerla visibile.
 *
 * Riceve:
 * - `moves`: le mosse della partita;
 * - `cursor`: quante mosse sono state giocate fino alla posizione mostrata;
 * - `notation`: la notazione con cui scrivere le mosse;
 * - `onSelect`: chiamata con il numero della mezza mossa scelta (1 = la prima).
 * Restituisce: l'elenco, oppure un invito a muovere se la partita è vuota.
 */
import { useEffect, useRef } from 'react';
import type { MoveRecord } from '../../../bindings/MoveRecord';
import type { Notation } from '../../../bindings/Notation';
import { classNames } from '../../../lib/classNames';
import { formatSan } from '../../../lib/formatSan';
import { groupMovesIntoRows } from './groupMovesIntoRows';
import styles from './MoveList.module.css';

export interface MoveListProps {
  /** Le mosse della partita, in ordine. */
  moves: readonly MoveRecord[];
  /** Quante mosse sono state giocate fino alla posizione mostrata (0 = partenza). */
  cursor: number;
  /** La notazione con cui scrivere le mosse. */
  notation: Notation;
  /** Chiamata quando l'utente sceglie una mossa. */
  onSelect: (ply: number) => void;
}

export function MoveList({ moves, cursor, notation, onSelect }: MoveListProps) {
  /** Il pulsante della mossa evidenziata, per farlo scorrere in vista. */
  const currentRef = useRef<HTMLButtonElement>(null);

  // Tiene in vista la mossa evidenziata (ad esempio navigando con le frecce).
  useEffect(() => {
    currentRef.current?.scrollIntoView({ block: 'nearest' });
  }, [cursor]);

  if (moves.length === 0) {
    return <p className={styles.empty}>Nessuna mossa: muovi un pezzo per cominciare.</p>;
  }

  /** Una cella con una mossa, oppure vuota (con i puntini, come nei libri). */
  const renderMove = (move: MoveRecord | null) => {
    if (move === null) {
      return <span className={styles.placeholder}>…</span>;
    }
    const isCurrent = move.ply === cursor;
    return (
      <button
        type="button"
        ref={isCurrent ? currentRef : undefined}
        className={classNames(styles.move, isCurrent && styles.current)}
        aria-current={isCurrent ? 'step' : undefined}
        onClick={() => {
          onSelect(move.ply);
        }}
      >
        {formatSan(move.san, notation)}
      </button>
    );
  };

  return (
    <ol className={styles.list} aria-label="Mosse della partita">
      {groupMovesIntoRows(moves).map((row) => (
        <li key={row.moveNumber} className={styles.row}>
          <span className={styles.number}>{row.moveNumber}.</span>
          {renderMove(row.white)}
          {renderMove(row.black)}
        </li>
      ))}
    </ol>
  );
}
