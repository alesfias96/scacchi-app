/**
 * GameControls — i comandi della partita, sotto l'elenco delle mosse.
 *
 * Cosa fa: mostra due file di pulsanti.
 * - Navigazione: inizio, mossa precedente, mossa successiva, fine
 *   (gli stessi comandi funzionano con i tasti Home, ←, →, Fine).
 * - Azioni: nuova partita, annulla l'ultima mossa, abbandona, gira la scacchiera.
 * I pulsanti che non avrebbero effetto sono disattivati (ad esempio
 * "mossa precedente" quando si è già all'inizio, o "annulla" in una partita
 * con l'orologio). "Abbandona" chiede conferma, perché non si può tornare indietro.
 *
 * Riceve:
 * - `cursor`: quante mosse sono state giocate fino alla posizione mostrata;
 * - `totalMoves`: quante mosse ha la partita;
 * - `canUndo`, `canResign`: se annullare e abbandonare sono possibili (lo decide Rust);
 * - `onGoTo`: chiamata con la mezza mossa da mostrare (0 = partenza);
 * - `onNewGame`, `onUndo`, `onResign`, `onFlip`: chiamate dai rispettivi pulsanti.
 * Restituisce: i comandi.
 */
import { useState } from 'react';
import { Button } from '../../../components/button/Button';
import { Icon } from '../../../components/icon/Icon';
import styles from './GameControls.module.css';

export interface GameControlsProps {
  /** Quante mosse sono state giocate fino alla posizione mostrata. */
  cursor: number;
  /** Quante mosse ha la partita. */
  totalMoves: number;
  /** `true` se si può ritirare la mossa. */
  canUndo: boolean;
  /** `true` se si può abbandonare (la partita non è finita). */
  canResign: boolean;
  /** Mostra la posizione dopo `ply` mosse. */
  onGoTo: (ply: number) => void;
  /** Apre la finestra "Nuova partita". */
  onNewGame: () => void;
  /** Ritira l'ultima mossa. */
  onUndo: () => void;
  /** Abbandona la partita (dopo la conferma). */
  onResign: () => void;
  /** Gira la scacchiera. */
  onFlip: () => void;
}

export function GameControls({
  cursor,
  totalMoves,
  canUndo,
  canResign,
  onGoTo,
  onNewGame,
  onUndo,
  onResign,
  onFlip,
}: GameControlsProps) {
  /** `true` mentre si chiede conferma dell'abbandono. */
  const [isConfirmingResign, setIsConfirmingResign] = useState(false);
  const isAtStart = cursor === 0;
  const isAtEnd = cursor >= totalMoves;

  return (
    <div className={styles.controls}>
      <div className={styles.navigation} role="group" aria-label="Scorri la partita">
        <Button
          variant="icon"
          aria-label="Posizione iniziale (Home)"
          disabled={isAtStart}
          onClick={() => {
            onGoTo(0);
          }}
        >
          <Icon name="first" />
        </Button>
        <Button
          variant="icon"
          aria-label="Mossa precedente (←)"
          disabled={isAtStart}
          onClick={() => {
            onGoTo(cursor - 1);
          }}
        >
          <Icon name="previous" />
        </Button>
        <Button
          variant="icon"
          aria-label="Mossa successiva (→)"
          disabled={isAtEnd}
          onClick={() => {
            onGoTo(cursor + 1);
          }}
        >
          <Icon name="next" />
        </Button>
        <Button
          variant="icon"
          aria-label="Ultima mossa (Fine)"
          disabled={isAtEnd}
          onClick={() => {
            onGoTo(totalMoves);
          }}
        >
          <Icon name="last" />
        </Button>
      </div>

      {isConfirmingResign ? (
        <div className={styles.confirm} role="group" aria-label="Conferma abbandono">
          <span className={styles.question}>Abbandonare la partita?</span>
          <Button
            variant="primary"
            onClick={() => {
              setIsConfirmingResign(false);
              onResign();
            }}
          >
            Sì, abbandono
          </Button>
          <Button
            onClick={() => {
              setIsConfirmingResign(false);
            }}
          >
            No
          </Button>
        </div>
      ) : (
        <div className={styles.actions}>
          <Button variant="primary" onClick={onNewGame}>
            <Icon name="plus" />
            Nuova partita
          </Button>
          <Button
            variant="icon"
            aria-label="Annulla l’ultima mossa"
            disabled={!canUndo}
            onClick={onUndo}
          >
            <Icon name="undo" />
          </Button>
          <Button
            variant="icon"
            aria-label="Abbandona"
            disabled={!canResign}
            onClick={() => {
              setIsConfirmingResign(true);
            }}
          >
            <Icon name="flag" />
          </Button>
          <Button variant="icon" aria-label="Gira la scacchiera" onClick={onFlip}>
            <Icon name="flip" />
          </Button>
        </div>
      )}
    </div>
  );
}
