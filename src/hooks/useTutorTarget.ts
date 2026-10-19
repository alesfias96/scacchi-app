/**
 * useTutorTarget — una sezione dice al tutor quale posizione sta mostrando.
 *
 * Cosa fa: a ogni cambiamento della posizione la passa al tutor; quando la
 * sezione viene chiusa, il tutor non ha più nulla da spiegare.
 *
 * @param target - la posizione mostrata (va creata con `useMemo`, così non
 *   cambia a ogni disegno), oppure `null` se non c'è.
 */
import { useEffect } from 'react';
import { useTutorStore, type TutorTarget } from '../state/tutorStore';

export function useTutorTarget(target: TutorTarget | null) {
  const setTarget = useTutorStore((state) => state.setTarget);

  useEffect(() => {
    setTarget(target);
  }, [target, setTarget]);

  // Alla chiusura della sezione il tutor resta senza posizione.
  useEffect(
    () => () => {
      setTarget(null);
    },
    [setTarget],
  );
}
