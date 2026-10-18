/**
 * Scrive un tempo di riflessione come lo scrivono i giocatori: "minuti + secondi".
 *
 * @param timeControl - il tempo scelto, oppure `null` per una partita senza orologio.
 * @returns ad esempio `"5+3"`, `"½+0"` (mezzo minuto) o `"Senza orologio"`.
 */
import type { TimeControl } from '../bindings/TimeControl';

/** I minuti iniziali: interi, "½" per mezzo minuto, altrimenti con un decimale. */
function describeMinutes(seconds: number): string {
  if (seconds % 60 === 0) {
    return String(seconds / 60);
  }
  return seconds === 30 ? '½' : (seconds / 60).toFixed(1);
}

export function describeTimeControl(timeControl: TimeControl | null): string {
  if (timeControl === null) {
    return 'Senza orologio';
  }
  return `${describeMinutes(timeControl.initialSeconds)}+${String(timeControl.incrementSeconds)}`;
}
