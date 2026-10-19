/**
 * useTrainTheme — «Allenati su questo tema»: puzzle dello stesso motivo tattico.
 *
 * Cosa fa: restituisce una funzione che, ricevuto un tema dei puzzle (es.
 * `"fork"`), lo sceglie nelle impostazioni dell'allenamento (la modalità
 * «ripasso» non filtra per tema: in quel caso diventa «profondità»), comincia
 * subito un puzzle di quel tema e apre la sezione Allenamento.
 * Fuori da un router (nei test di alcuni componenti) cambia solo le scelte.
 *
 * @returns la funzione da chiamare con il tema.
 */
import { useRouter } from '@tanstack/react-router';
import { useCallback } from 'react';
import { useSettingsStore } from '../../state/settingsStore';
import { useTrainingStore } from '../../state/trainingStore';

export function useTrainTheme(): (theme: string) => void {
  const router = useRouter({ warn: false }) as ReturnType<typeof useRouter> | undefined;

  return useCallback(
    (theme: string) => {
      const { settings, update } = useSettingsStore.getState();
      if (settings === null) {
        return;
      }
      const current = settings.trainingSetup;
      const setup = {
        ...current,
        theme,
        mode: current.mode === 'review' ? ('depth' as const) : current.mode,
      };
      void update({ trainingSetup: setup });
      void useTrainingStore.getState().start(setup);
      void router?.navigate({ to: '/allenamento' });
    },
    [router],
  );
}
