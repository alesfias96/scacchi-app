/**
 * MoveList — la partita scritta come nei libri, con le varianti.
 *
 * Cosa fa: scrive la linea principale in righe (numero, mossa del Bianco,
 * mossa del Nero). Sotto la riga di una mossa che ha delle alternative
 * compaiono le **varianti**, rientrate e scritte in fila, con le loro
 * sotto-varianti tra parentesi. Un clic su una mossa (anche di una variante)
 * mostra la posizione dopo quella mossa; la mossa mostrata è evidenziata e
 * resta in vista. Quando la mossa mostrata è in una variante, sotto la lista
 * compaiono «Promuovi a linea principale» (se la partita lo permette) ed
 * «Elimina da qui».
 *
 * Riceve:
 * - `tree`: l'albero delle mosse;
 * - `notation`: la notazione con cui scrivere le mosse;
 * - `canPromote`: se le varianti si possono promuovere a linea principale;
 * - `onSelect`: chiamata con la mossa scelta;
 * - `onPromote`, `onDelete`: chiamate con la mossa mostrata.
 * Restituisce: la lista.
 */
import { useEffect, useRef } from 'react';
import type { MoveNode } from '../../../bindings/MoveNode';
import type { MoveTree } from '../../../bindings/MoveTree';
import type { Notation } from '../../../bindings/Notation';
import { Button } from '../../../components/button/Button';
import { classNames } from '../../../lib/classNames';
import { formatSan } from '../../../lib/formatSan';
import { buildMoveRows, variationTokens } from './buildMoveRows';
import styles from './MoveList.module.css';

export interface MoveListProps {
  /** L'albero delle mosse: linea principale e varianti. */
  tree: MoveTree;
  /** La notazione con cui scrivere le mosse. */
  notation: Notation;
  /** `true` se una variante si può promuovere a linea principale. */
  canPromote: boolean;
  /** Chiamata quando l'utente sceglie una mossa. */
  onSelect: (node: number) => void;
  /** Promuove a linea principale la variante della mossa indicata. */
  onPromote: (node: number) => void;
  /** Elimina la mossa indicata e il suo seguito. */
  onDelete: (node: number) => void;
}

export function MoveList({
  tree,
  notation,
  canPromote,
  onSelect,
  onPromote,
  onDelete,
}: MoveListProps) {
  /** Il pulsante della mossa evidenziata, per farlo scorrere in vista. */
  const currentRef = useRef<HTMLButtonElement>(null);

  /** La mossa mostrata. */
  const currentId = tree.current;

  // Tiene in vista la mossa evidenziata (ad esempio navigando con le frecce).
  useEffect(() => {
    currentRef.current?.scrollIntoView({ block: 'nearest' });
  }, [currentId]);

  if (tree.nodes.length === 0) {
    return <p className={styles.empty}>Nessuna mossa: muovi un pezzo per cominciare.</p>;
  }

  /** Un pulsante per una mossa. */
  const moveButton = (node: MoveNode, number: string | null = null) => {
    const isCurrent = node.id === tree.current;
    return (
      <button
        key={node.id}
        type="button"
        ref={isCurrent ? currentRef : undefined}
        className={classNames(
          styles.move,
          !node.mainline && styles.variationMove,
          isCurrent && styles.current,
        )}
        aria-current={isCurrent ? 'step' : undefined}
        onClick={() => {
          onSelect(node.id);
        }}
      >
        {number !== null && <span className={styles.inlineNumber}>{number} </span>}
        {formatSan(node.record.san, notation)}
      </button>
    );
  };

  /** Una variante scritta in fila, con le parentesi delle sotto-varianti. */
  const variation = (first: MoveNode) => (
    <p key={first.id} className={styles.variation}>
      {variationTokens(tree, first).map((token, position) => {
        switch (token.kind) {
          case 'open':
            return (
              <span key={`open-${String(position)}`} className={styles.paren}>
                (
              </span>
            );
          case 'close':
            return (
              <span key={`close-${String(position)}`} className={styles.paren}>
                )
              </span>
            );
          case 'move':
            return moveButton(token.node, token.number);
        }
      })}
    </p>
  );

  const current = tree.nodes.find((node) => node.id === tree.current);
  const inVariation = current !== undefined && !current.mainline;

  return (
    <div className={styles.wrapper}>
      <ol className={styles.list} aria-label="Mosse della partita">
        {buildMoveRows(tree).map((row) => (
          <li key={row.moveNumber} className={styles.item}>
            <div className={styles.row}>
              <span className={styles.number}>{row.moveNumber}.</span>
              {row.white === null ? (
                <span className={styles.placeholder}>…</span>
              ) : (
                moveButton(row.white)
              )}
              {row.black === null ? <span /> : moveButton(row.black)}
            </div>
            {row.variations.length > 0 && (
              <div className={styles.variations}>{row.variations.map(variation)}</div>
            )}
          </li>
        ))}
      </ol>
      {inVariation && (
        <div className={styles.actions} role="group" aria-label="Variante mostrata">
          {canPromote && (
            <Button
              onClick={() => {
                onPromote(current.id);
              }}
            >
              Promuovi a linea principale
            </Button>
          )}
          <Button
            onClick={() => {
              onDelete(current.id);
            }}
          >
            Elimina da qui
          </Button>
        </div>
      )}
    </div>
  );
}
