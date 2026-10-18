/**
 * useGameKeyboard — scorre la partita con la tastiera.
 *
 * Tasti:
 * - ← mossa precedente, → mossa successiva;
 * - Home posizione iniziale, Fine (End) ultima mossa.
 *
 * I tasti vengono ignorati mentre si scrive in un campo di testo (ad esempio
 * nella finestra "Incolla"), quando sono premuti insieme a Ctrl, Alt o Cmd
 * (così le scorciatoie del sistema continuano a funzionare) e quando
 * `enabled` è falso (ad esempio con una finestra di dialogo aperta).
 *
 * @param actions - cosa fare per ciascun tasto.
 * @param enabled - se falso i tasti vengono ignorati. Predefinito: vero.
 */
import { useEffect, useRef } from 'react';

/** Le azioni collegate ai tasti. */
export interface GameKeyboardActions {
  /** ← */
  onPrevious: () => void;
  /** → */
  onNext: () => void;
  /** Home */
  onFirst: () => void;
  /** Fine (End) */
  onLast: () => void;
}

/** Gli elementi in cui si scrive: lì le frecce servono a spostare il cursore. */
const TYPING_ELEMENTS = new Set(['INPUT', 'TEXTAREA', 'SELECT']);

export function useGameKeyboard(actions: GameKeyboardActions, enabled = true): void {
  /** Le azioni più recenti, senza dover riattaccare l'ascolto dei tasti a ogni disegno. */
  const actionsRef = useRef(actions);
  useEffect(() => {
    actionsRef.current = actions;
  }, [actions]);

  useEffect(() => {
    if (!enabled) {
      return undefined;
    }

    const handleKeyDown = (event: KeyboardEvent) => {
      const target = event.target;
      const isTyping =
        target instanceof HTMLElement &&
        (TYPING_ELEMENTS.has(target.tagName) || target.isContentEditable);
      if (isTyping || event.ctrlKey || event.altKey || event.metaKey) {
        return;
      }

      /** Quale azione corrisponde a ciascun tasto. */
      const keyActions: Record<string, () => void> = {
        ArrowLeft: actionsRef.current.onPrevious,
        ArrowRight: actionsRef.current.onNext,
        Home: actionsRef.current.onFirst,
        End: actionsRef.current.onLast,
      };
      const action = keyActions[event.key];

      if (action !== undefined) {
        // Evita che la pagina scorra mentre si naviga tra le mosse.
        event.preventDefault();
        action();
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => {
      window.removeEventListener('keydown', handleKeyDown);
    };
  }, [enabled]);
}
