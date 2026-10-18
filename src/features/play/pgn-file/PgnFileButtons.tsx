/**
 * PgnFileButtons — salvare e aprire file `.pgn`.
 *
 * Cosa fa: due pulsanti che aprono la finestra del sistema.
 * - "Salva file": salva la partita in un file `.pgn` (il formato di tutti i
 *   programmi di scacchi), dove si vuole;
 * - "Apri file": carica una partita da un file `.pgn` al posto di quella in
 *   corso (si gioca poi a due giocatori).
 * Dopo un salvataggio compare per qualche secondo una conferma (o il motivo
 * dell'errore). Se nella finestra si sceglie "Annulla", non succede nulla.
 *
 * Riceve:
 * - `onSave`: salva la partita; restituisce `false` se l'utente ha annullato;
 * - `onOpen`: apre un file (gli eventuali errori li mostra la pagina).
 * Restituisce: i pulsanti e la conferma.
 */
import { useEffect, useState } from 'react';
import { Button } from '../../../components/button/Button';
import { Icon } from '../../../components/icon/Icon';
import { describeError } from '../../../lib/describeError';
import styles from './PgnFileButtons.module.css';

/** Per quanto tempo resta visibile la conferma (millisecondi). */
const FEEDBACK_DURATION_MS = 2500;

export interface PgnFileButtonsProps {
  /** Salva la partita in un file; `false` se l'utente ha annullato. */
  onSave: () => Promise<boolean>;
  /** Apre una partita da un file. */
  onOpen: () => Promise<void>;
}

export function PgnFileButtons({ onSave, onOpen }: PgnFileButtonsProps) {
  /** La conferma (o l'errore) dell'ultimo salvataggio, oppure `null`. */
  const [feedback, setFeedback] = useState<string | null>(null);

  // Nasconde la conferma dopo qualche secondo.
  useEffect(() => {
    if (feedback === null) {
      return undefined;
    }
    const timer = window.setTimeout(() => {
      setFeedback(null);
    }, FEEDBACK_DURATION_MS);
    return () => {
      window.clearTimeout(timer);
    };
  }, [feedback]);

  const save = async () => {
    try {
      if (await onSave()) {
        setFeedback('Partita salvata nel file');
      }
    } catch (error) {
      setFeedback(`Salvataggio non riuscito: ${describeError(error)}`);
    }
  };

  return (
    <div className={styles.container}>
      <div className={styles.buttons}>
        <Button
          title="Salva la partita in un file .pgn"
          onClick={() => {
            void save();
          }}
        >
          <Icon name="save" />
          Salva file
        </Button>
        <Button
          title="Apri una partita da un file .pgn"
          onClick={() => {
            void onOpen();
          }}
        >
          <Icon name="open" />
          Apri file
        </Button>
      </div>
      {/* Sempre presente (anche vuota) così i lettori di schermo annunciano la conferma. */}
      <p className={styles.feedback} role="status">
        {feedback}
      </p>
    </div>
  );
}
