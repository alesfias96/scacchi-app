/**
 * CoachPopup — il messaggio del tutor sopra la scacchiera (modalità Tutor).
 *
 * Cosa fa: mostra un messaggio alla volta, con il titoletto, la frase (nella
 * notazione scelta), la risposta migliore se c'è e il conteggio ("2 di 5").
 * Le case e le frecce del messaggio le disegna la scacchiera (le passa la
 * pagina). Si scorre con «Indietro» / «Avanti», oppure con Invio o Spazio;
 * Esc lo chiude. «Approfondisci» apre la scheda della base di conoscenza.
 * Sta in basso sulla scacchiera; con «Sposta in alto» va in alto, se copre
 * la zona interessante.
 *
 * Riceve:
 * - `messages`, `index`: i messaggi e quello mostrato;
 * - `notation`: la notazione scelta nelle Impostazioni;
 * - `onIndexChange`: chiamata con il nuovo messaggio da mostrare;
 * - `onClose`: chiude i messaggi;
 * - `onOpenCard`: apre una scheda della base di conoscenza.
 * Restituisce: il popup.
 */
import { useEffect, useState } from 'react';
import type { Notation } from '../../../bindings/Notation';
import { Button } from '../../../components/button/Button';
import { classNames } from '../../../lib/classNames';
import { formatFact } from '../../../lib/formatFact';
import { formatLine } from '../../../lib/formatLine';
import type { CoachMessage } from './buildCoachMessages';
import styles from './CoachPopup.module.css';

export interface CoachPopupProps {
  messages: readonly CoachMessage[];
  index: number;
  notation: Notation;
  onIndexChange: (index: number) => void;
  onClose: () => void;
  onOpenCard: (cardId: string) => void;
}

/** Quante mezze mosse della risposta migliore mostrare. */
const LINE_LENGTH = 4;

/** Gli elementi in cui si scrive o che reagiscono già da soli a Invio e Spazio. */
const IGNORED_ELEMENTS = new Set(['INPUT', 'TEXTAREA', 'SELECT', 'BUTTON', 'SUMMARY']);

export function CoachPopup({
  messages,
  index,
  notation,
  onIndexChange,
  onClose,
  onOpenCard,
}: CoachPopupProps) {
  /** `true` se il popup è stato spostato in alto. */
  const [isOnTop, setIsOnTop] = useState(false);
  const message = messages[index];
  const isLast = index >= messages.length - 1;

  // Invio o Spazio: messaggio successivo (o chiude dopo l'ultimo); Esc: chiude.
  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      const target = event.target;
      if (target instanceof HTMLElement && IGNORED_ELEMENTS.has(target.tagName)) {
        return;
      }
      if (event.key === 'Enter' || event.key === ' ') {
        event.preventDefault();
        if (isLast) {
          onClose();
        } else {
          onIndexChange(index + 1);
        }
      } else if (event.key === 'Escape') {
        onClose();
      }
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => {
      window.removeEventListener('keydown', handleKeyDown);
    };
  }, [index, isLast, onIndexChange, onClose]);

  if (message === undefined) {
    return null;
  }

  return (
    <section
      key={index}
      className={classNames(styles.popup, isOnTop ? styles.top : styles.bottom)}
      aria-label="Messaggio del tutor"
      aria-live="polite"
    >
      <header className={styles.header}>
        <span className={styles.heading}>{message.heading}</span>
        <span className={styles.count}>
          {index + 1} di {messages.length}
        </span>
      </header>
      <p className={styles.text}>{formatFact(message.fact, notation)}</p>
      {message.line !== null && message.line.san.length > 0 && (
        <p className={styles.line}>
          La risposta migliore: {formatLine(message.line, notation, LINE_LENGTH)}
        </p>
      )}
      <div className={styles.actions}>
        <Button
          onClick={() => {
            onIndexChange(index - 1);
          }}
          disabled={index === 0}
        >
          Indietro
        </Button>
        {isLast ? (
          <Button variant="primary" onClick={onClose}>
            Fine
          </Button>
        ) : (
          <Button
            variant="primary"
            onClick={() => {
              onIndexChange(index + 1);
            }}
          >
            Avanti
          </Button>
        )}
        {message.cardId !== null && (
          <Button
            onClick={() => {
              if (message.cardId !== null) {
                onOpenCard(message.cardId);
              }
            }}
          >
            Approfondisci
          </Button>
        )}
        <span className={styles.spacer} />
        <Button
          onClick={() => {
            setIsOnTop((top) => !top);
          }}
        >
          {isOnTop ? 'Sposta in basso' : 'Sposta in alto'}
        </Button>
        <Button variant="icon" aria-label="Chiudi i messaggi (Esc)" onClick={onClose}>
          ×
        </Button>
      </div>
    </section>
  );
}
