/**
 * GameRow — una partita nell'elenco dell'archivio.
 *
 * Cosa fa: mostra in una riga quando si è giocato, contro chi (e a che
 * livello), con che colore, con che tempo, l'apertura, quante mosse e
 * com'è andata (etichetta colorata: vinta, persa, patta, in corso). A destra:
 * - "Rivedi" (partita finita) o "Continua" (non finita): la apre in "Gioca";
 * - "Analizza" o, se è già stata analizzata, "Vedi l'analisi": la apre in
 *   "Analisi partite" (l'analisi parte da sola se serve);
 * - "Elimina": la toglie dall'archivio, dopo una conferma.
 *
 * Riceve:
 * - `game`: il riassunto della partita;
 * - `levelName`: il nome del livello del computer (es. "Circolo", "Maia 1500"), se c'è;
 * - `onOpen`, `onReview`, `onDelete`: chiamate con il nome unico della partita.
 * Restituisce: la riga.
 */
import { useState } from 'react';
import type { GameSummary } from '../../../bindings/GameSummary';
import { Button } from '../../../components/button/Button';
import { classNames } from '../../../lib/classNames';
import { describeTimeControl } from '../../../lib/describeTimeControl';
import { formatDateTime } from '../../../lib/formatDateTime';
import { describeOutcome } from './describeOutcome';
import styles from './GameRow.module.css';

export interface GameRowProps {
  /** Il riassunto della partita. */
  game: GameSummary;
  /** Il nome del livello del computer, se si è giocato contro il computer. */
  levelName?: string;
  /** Apre la partita in "Gioca". */
  onOpen: (id: string) => void;
  /** Apre l'analisi della partita. */
  onReview: (id: string) => void;
  /** Elimina la partita (dopo la conferma). */
  onDelete: (id: string) => void;
}

/** Chi era il computer: "Stockfish · Circolo" oppure "Maia 1500" (il nome del livello dice già tutto). */
function computerName(game: GameSummary, levelName: string | undefined): string {
  if (game.engine === 'maia') {
    return levelName ?? 'Maia';
  }
  return `Stockfish${levelName === undefined ? '' : ` · ${levelName}`}`;
}

export function GameRow({ game, levelName, onOpen, onReview, onDelete }: GameRowProps) {
  /** `true` mentre si chiede conferma dell'eliminazione. */
  const [isConfirming, setIsConfirming] = useState(false);
  const outcome = describeOutcome(game);
  const humanSide = game.computerSide === 'white' ? 'Nero' : 'Bianco';
  const opponent =
    game.computerSide === null
      ? 'Due giocatori'
      : `${computerName(game, levelName)} · tu col ${humanSide}`;
  const moves = Math.ceil(game.plyCount / 2);

  return (
    <li className={styles.row}>
      <span className={classNames(styles.outcome, styles[outcome.tone])}>{outcome.label}</span>
      <div className={styles.details}>
        <p className={styles.headline}>
          {game.opening === null ? 'Apertura non riconosciuta' : game.opening.name}
          {game.opening !== null && <span className={styles.eco}>{game.opening.eco}</span>}
        </p>
        <p className={styles.meta}>
          {formatDateTime(game.startedAtMs)} · {opponent} · {describeTimeControl(game.timeControl)}{' '}
          · {moves} {moves === 1 ? 'mossa' : 'mosse'}
        </p>
      </div>
      {isConfirming ? (
        <div className={styles.actions} role="group" aria-label="Conferma eliminazione">
          <span className={styles.question}>Eliminarla?</span>
          <Button
            variant="primary"
            onClick={() => {
              setIsConfirming(false);
              onDelete(game.id);
            }}
          >
            Sì, elimina
          </Button>
          <Button
            onClick={() => {
              setIsConfirming(false);
            }}
          >
            No
          </Button>
        </div>
      ) : (
        <div className={styles.actions}>
          <Button
            variant="primary"
            onClick={() => {
              onOpen(game.id);
            }}
          >
            {game.result === null ? 'Continua' : 'Rivedi'}
          </Button>
          <Button
            onClick={() => {
              onReview(game.id);
            }}
          >
            {game.analyzed ? 'Vedi l’analisi' : 'Analizza'}
          </Button>
          <Button
            onClick={() => {
              setIsConfirming(true);
            }}
          >
            Elimina
          </Button>
        </div>
      )}
    </li>
  );
}
