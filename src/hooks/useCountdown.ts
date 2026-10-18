/**
 * useCountdown — fa scorrere i secondi dell'orologio sullo schermo.
 *
 * Cosa fa: Rust manda il tempo rimasto solo quando qualcosa cambia (una
 * mossa). Per vedere i secondi che scorrono, questo hook parte dal valore
 * ricevuto e, se quell'orologio è in moto, toglie il tempo passato da allora,
 * aggiornandosi dieci volte al secondo. Il conteggio "vero" (chi perde per
 * tempo) resta comunque in Rust.
 *
 * Riceve:
 * - `ms`: il tempo rimasto ricevuto da Rust, in millesimi di secondo;
 * - `running`: se questo orologio sta scorrendo.
 * Restituisce: il tempo rimasto adesso, in millesimi di secondo.
 */
import { useEffect, useState } from 'react';

/** Ogni quanto si aggiorna lo schermo, in millesimi: abbastanza per i decimi. */
const TICK_MS = 100;

export function useCountdown(ms: number, running: boolean): number {
  /** Quanto tempo è passato da quando è arrivato il valore `ms`. */
  const [elapsed, setElapsed] = useState(0);

  // A ogni nuovo valore da Rust si riparte da zero e, se l'orologio è in
  // moto, si misura il tempo che passa.
  useEffect(() => {
    setElapsed(0);
    if (!running) {
      return;
    }
    const receivedAt = Date.now();
    const timer = window.setInterval(() => {
      setElapsed(Date.now() - receivedAt);
    }, TICK_MS);
    return () => {
      window.clearInterval(timer);
    };
  }, [ms, running]);

  return running ? Math.max(0, ms - elapsed) : ms;
}
