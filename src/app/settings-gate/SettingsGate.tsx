/**
 * SettingsGate — mostra l'app solo dopo aver letto le impostazioni.
 *
 * Cosa fa: all'avvio chiede a Rust le impostazioni salvate. Finché non
 * arrivano non mostra nulla (dura una frazione di secondo), così l'app non
 * compare prima con il tema sbagliato per poi cambiare.
 * Se la lettura fallisce, spiega il problema.
 *
 * Riceve:
 * - `children`: l'app da mostrare quando le impostazioni sono pronte.
 * Restituisce: l'app, oppure il messaggio d'errore.
 */
import { useEffect, useState, type ReactNode } from 'react';
import { describeError } from '../../lib/describeError';
import { useSettingsStore } from '../../state/settingsStore';
import styles from './SettingsGate.module.css';

export interface SettingsGateProps {
  /** L'app da mostrare quando le impostazioni sono pronte. */
  children: ReactNode;
}

export function SettingsGate({ children }: SettingsGateProps) {
  const settings = useSettingsStore((state) => state.settings);
  const load = useSettingsStore((state) => state.load);
  const [loadError, setLoadError] = useState<string | null>(null);

  useEffect(() => {
    void load().catch((error: unknown) => {
      setLoadError(describeError(error));
    });
  }, [load]);

  if (loadError !== null) {
    return (
      <p className={styles.error} role="alert">
        Non riesco a leggere le impostazioni: {loadError}
      </p>
    );
  }

  if (settings === null) {
    return null;
  }

  return children;
}
