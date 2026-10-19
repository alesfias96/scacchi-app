/**
 * UpdateNotice — l'avviso di una versione nuova, all'avvio dell'app.
 *
 * Cosa fa: se nelle Impostazioni è acceso «Controlla a ogni avvio», all'apertura
 * dell'app controlla (una volta) se c'è una versione nuova. Se c'è, mostra un
 * piccolo riquadro in basso a destra con «Installa e riavvia» e «Più tardi».
 * Se il controllo non riesce (ad esempio senza Internet) non mostra niente:
 * l'esito si può vedere nelle Impostazioni.
 *
 * Riceve: niente.
 * Restituisce: il riquadro, oppure niente.
 */
import { useEffect } from 'react';
import { Button } from '../../components/button/Button';
import { useLoadedSettings } from '../../state/settingsStore';
import { useUpdateStore } from '../../state/updateStore';
import styles from './UpdateNotice.module.css';

export function UpdateNotice() {
  const { updateCheck } = useLoadedSettings();
  const status = useUpdateStore((state) => state.status);
  const noticeDismissed = useUpdateStore((state) => state.noticeDismissed);
  const check = useUpdateStore((state) => state.check);
  const install = useUpdateStore((state) => state.install);
  const dismissNotice = useUpdateStore((state) => state.dismissNotice);

  // Il controllo all'avvio parte una volta sola, e solo se lo si è chiesto.
  useEffect(() => {
    if (updateCheck === 'onStart' && useUpdateStore.getState().status.kind === 'idle') {
      void check();
    }
  }, [updateCheck, check]);

  if (noticeDismissed || (status.kind !== 'available' && status.kind !== 'installing')) {
    return null;
  }

  return (
    <aside className={styles.notice} aria-label="Aggiornamento disponibile">
      <p className={styles.text}>
        {status.kind === 'available'
          ? `È disponibile la versione ${status.version} di Scacchi.`
          : `Sto installando la versione ${status.version}: l’app si riavvierà da sola.`}
      </p>
      {status.kind === 'available' && (
        <div className={styles.actions}>
          <Button onClick={dismissNotice}>Più tardi</Button>
          <Button
            variant="primary"
            onClick={() => {
              void install();
            }}
          >
            Installa e riavvia
          </Button>
        </div>
      )}
    </aside>
  );
}
