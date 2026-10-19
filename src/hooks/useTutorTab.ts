/**
 * useTutorTab — la voce scelta in cima alla colonna di destra: il contenuto
 * della sezione («Partita», «Allenamento») oppure il «Tutor».
 *
 * Cosa fa: ricorda la voce scelta, dice al tutor quando è mostrato (lavora
 * solo allora) e fa passare da una voce all'altra con il tasto T. Il tasto è
 * ignorato mentre si scrive in un campo.
 *
 * @returns la voce scelta e la funzione per cambiarla.
 */
import { useEffect, useState } from 'react';
import { useTutorStore } from '../state/tutorStore';

/** Le due voci: il contenuto della sezione o il tutor. */
export type SideTab = 'main' | 'tutor';

/** Il tasto che passa da una voce all'altra. */
const TUTOR_KEY = 't';

/** Gli elementi in cui si scrive: lì una lettera è testo, non una scorciatoia. */
const TYPING_ELEMENTS = new Set(['INPUT', 'TEXTAREA', 'SELECT']);

export function useTutorTab(): [SideTab, (tab: SideTab) => void] {
  const [tab, setTab] = useState<SideTab>('main');
  const setTutorOpen = useTutorStore((state) => state.setOpen);

  useEffect(() => {
    setTutorOpen(tab === 'tutor');
  }, [tab, setTutorOpen]);

  // Chiudendo la sezione il tutor non è più mostrato.
  useEffect(
    () => () => {
      setTutorOpen(false);
    },
    [setTutorOpen],
  );

  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      const target = event.target;
      const isTyping =
        target instanceof HTMLElement &&
        (TYPING_ELEMENTS.has(target.tagName) || target.isContentEditable);
      if (isTyping || event.ctrlKey || event.altKey || event.metaKey) {
        return;
      }
      if (event.key.toLowerCase() === TUTOR_KEY) {
        setTab((current) => (current === 'tutor' ? 'main' : 'tutor'));
      }
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => {
      window.removeEventListener('keydown', handleKeyDown);
    };
  }, []);

  return [tab, setTab];
}
