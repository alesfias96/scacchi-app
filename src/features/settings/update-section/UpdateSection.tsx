/**
 * UpdateSection — la scheda «Aggiornamenti» delle Impostazioni.
 *
 * Cosa fa: mostra la versione installata e il pulsante «Controlla
 * aggiornamenti». Se c'è una versione nuova, la propone con «Installa e
 * riavvia»: l'app la scarica, ne controlla la firma, la installa e si riavvia.
 * Con l'interruttore «Controlla a ogni avvio» il controllo si fa anche da solo
 * all'apertura dell'app (spento di default: l'app non si collega a Internet
 * se non glielo si chiede).
 *
 * Riceve:
 * - `checkOnStart`: se il controllo a ogni avvio è acceso;
 * - `onCheckOnStartChange`: chiamata quando lo si accende o spegne.
 * Restituisce: la scheda.
 */
import { useEffect } from 'react';
import { Button } from '../../../components/button/Button';
import { PanelCard } from '../../../components/panel-card/PanelCard';
import { Switch } from '../../../components/switch/Switch';
import { useUpdateStore, type UpdateStatus } from '../../../state/updateStore';
import styles from './UpdateSection.module.css';

export interface UpdateSectionProps {
  checkOnStart: boolean;
  onCheckOnStartChange: (checkOnStart: boolean) => void;
}

/** La frase che spiega l'esito del controllo, oppure `null` prima del primo controllo. */
function describeStatus(status: UpdateStatus): string | null {
  switch (status.kind) {
    case 'idle':
      return null;
    case 'checking':
      return 'Controllo in corso…';
    case 'upToDate':
      return 'Hai già l’ultima versione.';
    case 'available':
      return `È disponibile la versione ${status.version}.`;
    case 'installing':
      return `Sto scaricando e installando la versione ${status.version}: l’app si riavvierà da sola.`;
    case 'failed':
      return `Non sono riuscito a controllare gli aggiornamenti: ${status.message}`;
  }
}

export function UpdateSection({ checkOnStart, onCheckOnStartChange }: UpdateSectionProps) {
  const installedVersion = useUpdateStore((state) => state.installedVersion);
  const status = useUpdateStore((state) => state.status);
  const loadVersion = useUpdateStore((state) => state.loadVersion);
  const check = useUpdateStore((state) => state.check);
  const install = useUpdateStore((state) => state.install);
  const isBusy = status.kind === 'checking' || status.kind === 'installing';
  const message = describeStatus(status);

  useEffect(() => {
    void loadVersion();
  }, [loadVersion]);

  return (
    <PanelCard title="Aggiornamenti">
      <p className={styles.version}>
        Versione installata: <strong>{installedVersion ?? '—'}</strong>
      </p>
      <div className={styles.actions}>
        <Button
          disabled={isBusy}
          onClick={() => {
            void check();
          }}
        >
          Controlla aggiornamenti
        </Button>
        {status.kind === 'available' && (
          <Button
            variant="primary"
            onClick={() => {
              void install();
            }}
          >
            Installa e riavvia
          </Button>
        )}
      </div>
      {message !== null && (
        <p
          className={status.kind === 'failed' ? styles.error : styles.message}
          role={status.kind === 'failed' ? 'alert' : 'status'}
        >
          {message}
        </p>
      )}
      <Switch
        label="Controlla a ogni avvio"
        checked={checkOnStart}
        onChange={onCheckOnStartChange}
      />
      <p className={styles.note}>
        L’app si collega a Internet solo per questo controllo. Gli aggiornamenti sono firmati: l’app
        installa soltanto quelli pubblicati da noi.
      </p>
    </PanelCard>
  );
}
