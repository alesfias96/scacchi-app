/**
 * ImportExport — copiare e incollare posizioni e partite.
 *
 * Cosa fa: tre pulsanti.
 * - "Copia FEN": copia negli appunti la posizione mostrata (formato FEN,
 *   accettato da tutti i programmi e siti di scacchi);
 * - "Copia PGN": copia negli appunti l'intera partita (formato PGN);
 * - "Incolla": apre la finestra per caricare una posizione o una partita.
 * Dopo una copia compare per qualche secondo una conferma (o il motivo
 * dell'errore).
 *
 * Il salvataggio delle partite in file `.pgn` arriverà nella Fase 4.
 *
 * Riceve:
 * - `fen`: la posizione mostrata, in notazione FEN;
 * - `onRequestPgn`: restituisce la partita in formato PGN;
 * - `onImport`: carica un testo FEN o PGN (lancia un errore se non è valido).
 * Restituisce: i pulsanti, la conferma e, se aperta, la finestra "Incolla".
 */
import { useEffect, useState } from 'react';
import { Button } from '../../../components/button/Button';
import { Icon } from '../../../components/icon/Icon';
import { copyText } from '../../../services/clipboardService';
import { describeError } from '../../../lib/describeError';
import { ImportDialog } from './ImportDialog';
import styles from './ImportExport.module.css';

/** Per quanto tempo resta visibile la conferma della copia (millisecondi). */
const FEEDBACK_DURATION_MS = 2500;

export interface ImportExportProps {
  /** La posizione mostrata, in notazione FEN. */
  fen: string;
  /** Restituisce la partita in formato PGN. */
  onRequestPgn: () => Promise<string>;
  /** Carica un testo FEN o PGN; lancia un errore se non è valido. */
  onImport: (text: string) => Promise<void>;
}

export function ImportExport({ fen, onRequestPgn, onImport }: ImportExportProps) {
  /** `true` mentre la finestra "Incolla" è aperta. */
  const [isDialogOpen, setIsDialogOpen] = useState(false);
  /** La conferma (o l'errore) dell'ultima copia, oppure `null`. */
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

  /**
   * Prepara un testo e lo copia, poi mostra la conferma.
   *
   * @param getText - restituisce il testo da copiare.
   * @param confirmation - il messaggio da mostrare se la copia riesce.
   */
  const copy = async (getText: () => Promise<string>, confirmation: string) => {
    try {
      await copyText(await getText());
      setFeedback(confirmation);
    } catch (error) {
      setFeedback(`Copia non riuscita: ${describeError(error)}`);
    }
  };

  return (
    <div className={styles.container}>
      <div className={styles.buttons}>
        <Button
          aria-label="Copia FEN"
          title="Copia la posizione (FEN)"
          onClick={() => {
            void copy(() => Promise.resolve(fen), 'Posizione (FEN) copiata');
          }}
        >
          <Icon name="copy" />
          FEN
        </Button>
        <Button
          aria-label="Copia PGN"
          title="Copia la partita (PGN)"
          onClick={() => {
            void copy(onRequestPgn, 'Partita (PGN) copiata');
          }}
        >
          <Icon name="copy" />
          PGN
        </Button>
        <Button
          title="Incolla una posizione o una partita"
          onClick={() => {
            setIsDialogOpen(true);
          }}
        >
          <Icon name="paste" />
          Incolla
        </Button>
      </div>

      {/* Sempre presente (anche vuota) così i lettori di schermo annunciano la conferma. */}
      <p className={styles.feedback} role="status">
        {feedback}
      </p>

      {isDialogOpen && (
        <ImportDialog
          onImport={onImport}
          onClose={() => {
            setIsDialogOpen(false);
          }}
        />
      )}
    </div>
  );
}
