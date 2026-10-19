/**
 * SideDrawer — una barra laterale a comparsa, sul bordo sinistro o destro della finestra.
 *
 * Cosa fa: tiene a portata di mano dei contenuti senza occupare spazio. Si
 * apre in quattro modi:
 * - portando il mouse sul bordo della finestra (o sulla linguetta);
 * - con un clic sulla linguetta;
 * - con il tasto indicato in `shortcut` (es. "I"), che la apre e la chiude.
 * Si richiude da sola quando il mouse la lascia (dopo un breve ritardo, così
 * non "sfarfalla"), a meno che sia **fissata** con il pulsante "Fissa" o che
 * la si stia usando con la tastiera (un suo elemento ha il focus).
 * Esc la chiude. Si sovrappone alla pagina senza spostarla; dentro scorre
 * per conto suo se il contenuto è lungo.
 *
 * Riceve:
 * - `side`: su quale bordo sta (predefinito: destra);
 * - `title`: il titolo della barra (letto anche dai lettori di schermo);
 * - `shortcut`: il tasto che la apre e la chiude (una lettera);
 * - `shortcutEnabled`: se falso il tasto è ignorato (ad esempio con una
 *   finestra di dialogo aperta);
 * - `padded`: se falso il contenuto occupa tutta la barra, senza margini
 *   (per contenuti che hanno già il loro fondo, come il menu principale);
 * - `onOpenChange`: facoltativa, chiamata quando la barra si apre o si chiude
 *   (ad esempio il tutor lavora solo quando la sua barra è aperta);
 * - `children`: il contenuto.
 * Restituisce: la linguetta sul bordo e la barra.
 */
import { useCallback, useEffect, useId, useRef, useState, type ReactNode } from 'react';
import { classNames } from '../../lib/classNames';
import { Button } from '../button/Button';
import styles from './SideDrawer.module.css';

/** Dopo quanto tempo (millisecondi) la barra si chiude quando il mouse la lascia. */
const CLOSE_DELAY_MS = 350;

/** Gli elementi in cui si scrive: lì una lettera è testo, non una scorciatoia. */
const TYPING_ELEMENTS = new Set(['INPUT', 'TEXTAREA', 'SELECT']);

export interface SideDrawerProps {
  /** Il bordo su cui sta la barra. Predefinito: `"right"`. */
  side?: 'left' | 'right';
  /** Il titolo della barra. */
  title: string;
  /** Il tasto che apre e chiude la barra (una lettera, es. "i"). */
  shortcut: string;
  /** Se falso il tasto è ignorato. Predefinito: vero. */
  shortcutEnabled?: boolean;
  /** Se falso il contenuto occupa tutta la barra, senza margini. Predefinito: vero. */
  padded?: boolean;
  /** Chiamata con `true` quando la barra si apre e con `false` quando si chiude. */
  onOpenChange?: (isOpen: boolean) => void;
  /** Il contenuto della barra. */
  children: ReactNode;
}

export function SideDrawer({
  side = 'right',
  title,
  shortcut,
  shortcutEnabled = true,
  padded = true,
  onOpenChange,
  children,
}: SideDrawerProps) {
  const [isOpen, setIsOpen] = useState(false);
  const [isPinned, setIsPinned] = useState(false);
  const titleId = useId();
  /** Il timer della chiusura ritardata, per poterla annullare. */
  const closeTimer = useRef<number | null>(null);

  const cancelClose = useCallback(() => {
    if (closeTimer.current !== null) {
      window.clearTimeout(closeTimer.current);
      closeTimer.current = null;
    }
  }, []);

  const open = useCallback(() => {
    cancelClose();
    setIsOpen(true);
  }, [cancelClose]);

  /** Chiude subito e toglie anche il "fissa". */
  const close = useCallback(() => {
    cancelClose();
    setIsOpen(false);
    setIsPinned(false);
  }, [cancelClose]);

  /** Chiude dopo un breve ritardo, se la barra non è fissata. */
  const scheduleClose = useCallback(() => {
    if (isPinned) {
      return;
    }
    cancelClose();
    closeTimer.current = window.setTimeout(() => {
      closeTimer.current = null;
      setIsOpen(false);
    }, CLOSE_DELAY_MS);
  }, [isPinned, cancelClose]);

  // Il tasto della scorciatoia apre e chiude; Esc chiude.
  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      const target = event.target;
      const isTyping =
        target instanceof HTMLElement &&
        (TYPING_ELEMENTS.has(target.tagName) || target.isContentEditable);
      if (isTyping || event.ctrlKey || event.altKey || event.metaKey) {
        return;
      }
      if (event.key === 'Escape' && isOpen) {
        close();
      } else if (shortcutEnabled && event.key.toLowerCase() === shortcut.toLowerCase()) {
        if (isOpen) {
          close();
        } else {
          open();
        }
      }
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => {
      window.removeEventListener('keydown', handleKeyDown);
    };
  }, [isOpen, shortcut, shortcutEnabled, open, close]);

  // Avvisa chi lo ha chiesto quando la barra si apre o si chiude. La funzione
  // sta in un riferimento: se il genitore ne crea una nuova non si riavvisa.
  const onOpenChangeRef = useRef(onOpenChange);
  useEffect(() => {
    onOpenChangeRef.current = onOpenChange;
  }, [onOpenChange]);
  useEffect(() => {
    onOpenChangeRef.current?.(isOpen);
  }, [isOpen]);

  // Alla rimozione del componente, nessun timer resta in sospeso.
  useEffect(() => cancelClose, [cancelClose]);

  return (
    <>
      <div className={classNames(styles.edge, styles[side])} onMouseEnter={open}>
        <button
          type="button"
          className={styles.handle}
          aria-expanded={isOpen}
          aria-controls={titleId}
          title={`${title} (tasto ${shortcut.toUpperCase()})`}
          onClick={() => {
            if (isOpen) {
              close();
            } else {
              open();
            }
          }}
        >
          <span className={styles.handleLabel}>{title}</span>
        </button>
      </div>
      <aside
        className={classNames(styles.drawer, styles[side], isOpen && styles.open)}
        aria-labelledby={titleId}
        onMouseEnter={open}
        onMouseLeave={scheduleClose}
        onFocus={open}
        onBlur={(event) => {
          // Resta aperta se il focus si sposta su un altro elemento della barra.
          if (!event.currentTarget.contains(event.relatedTarget)) {
            scheduleClose();
          }
        }}
      >
        <header className={styles.header}>
          <h2 id={titleId} className={styles.title}>
            {title}
          </h2>
          <Button
            aria-pressed={isPinned}
            onClick={() => {
              setIsPinned((pinned) => !pinned);
            }}
          >
            {isPinned ? 'Fissata' : 'Fissa'}
          </Button>
          <Button variant="icon" aria-label="Chiudi (Esc)" onClick={close}>
            ×
          </Button>
        </header>
        <div className={classNames(styles.content, padded && styles.padded)}>{children}</div>
      </aside>
    </>
  );
}
