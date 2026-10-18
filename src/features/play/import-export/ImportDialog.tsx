/**
 * ImportDialog — la finestra per incollare una posizione o una partita.
 *
 * Cosa fa: mostra un'area di testo in cui incollare (Ctrl+V / Cmd+V) una
 * posizione in formato FEN oppure una partita in formato PGN, presa ad
 * esempio da un sito o da un libro digitale. "Carica" la manda a Rust: se il
 * testo è valido la finestra si chiude e la partita viene sostituita,
 * altrimenti il motivo dell'errore compare sotto il testo, che resta lì per
 * essere corretto. Esc o "Annulla" chiudono senza cambiare nulla.
 *
 * Riceve:
 * - `onImport`: carica il testo; deve lanciare un errore se il testo non è valido;
 * - `onClose`: chiude la finestra.
 * Restituisce: la finestra.
 */
import { useEffect, useId, useState, type SyntheticEvent } from 'react';
import { Button } from '../../../components/button/Button';
import { describeError } from '../../../lib/describeError';
import styles from './ImportDialog.module.css';

export interface ImportDialogProps {
  /** Carica il testo; lancia un errore se non è valido. */
  onImport: (text: string) => Promise<void>;
  /** Chiude la finestra. */
  onClose: () => void;
}

export function ImportDialog({ onImport, onClose }: ImportDialogProps) {
  /** Il testo scritto o incollato. */
  const [text, setText] = useState('');
  /** Il motivo dell'ultimo caricamento fallito, oppure `null`. */
  const [error, setError] = useState<string | null>(null);
  /** `true` mentre Rust sta leggendo il testo. */
  const [isLoading, setIsLoading] = useState(false);
  const titleId = useId();
  const hintId = useId();

  // Esc chiude la finestra.
  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === 'Escape') {
        onClose();
      }
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => {
      window.removeEventListener('keydown', handleKeyDown);
    };
  }, [onClose]);

  /** Invia il testo a Rust; chiude la finestra solo se il testo era valido. */
  const handleSubmit = async (event: SyntheticEvent<HTMLFormElement>) => {
    event.preventDefault();
    setIsLoading(true);
    try {
      await onImport(text);
      onClose();
    } catch (importError) {
      setError(describeError(importError));
      setIsLoading(false);
    }
  };

  return (
    <div className={styles.backdrop}>
      <form
        className={styles.dialog}
        role="dialog"
        aria-modal="true"
        aria-labelledby={titleId}
        onSubmit={(event) => {
          void handleSubmit(event);
        }}
      >
        <h2 id={titleId} className={styles.title}>
          Incolla posizione o partita
        </h2>
        <p id={hintId} className={styles.hint}>
          Incolla una posizione (FEN) oppure una partita intera (PGN). La partita attuale verrà
          sostituita.
        </p>
        <textarea
          className={styles.text}
          aria-label="Testo FEN o PGN"
          aria-describedby={hintId}
          value={text}
          spellCheck={false}
          autoFocus
          onChange={(event) => {
            setText(event.target.value);
            setError(null);
          }}
        />
        {error !== null && (
          <p className={styles.error} role="alert">
            {error}
          </p>
        )}
        <div className={styles.actions}>
          <Button onClick={onClose}>Annulla</Button>
          <Button type="submit" variant="primary" disabled={isLoading || text.trim() === ''}>
            Carica
          </Button>
        </div>
      </form>
    </div>
  );
}
