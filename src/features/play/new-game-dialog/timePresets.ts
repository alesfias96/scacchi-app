/**
 * I tempi di riflessione proposti nella finestra "Nuova partita".
 *
 * Sono i più comuni nei tornei e nei siti di scacchi, raggruppati per
 * categoria: Bullet (pochissimo tempo), Blitz, Rapid e Classica. Oltre a
 * questi ci sono "Senza orologio" e "Personalizzato" (minuti e secondi scelti a mano).
 */
import type { TimeControl } from '../../../bindings/TimeControl';
import { describeTimeControl } from '../../../lib/describeTimeControl';

/** Un tempo proposto. */
export interface TimePreset {
  /** Il nome tecnico della scelta (es. `"5+3"`, `"none"`, `"custom"`). */
  id: string;
  /** Il tempo; `null` = senza orologio. Per "Personalizzato" non c'è (lo sceglie l'utente). */
  timeControl: TimeControl | null;
  /** La categoria mostrata sotto il nome (es. `"Blitz"`). */
  category: string;
}

/** Il nome della scelta "Personalizzato". */
export const CUSTOM_PRESET_ID = 'custom';

/** Un tempo in minuti e secondi di incremento. */
function minutes(initialMinutes: number, incrementSeconds: number): TimeControl {
  return { initialSeconds: initialMinutes * 60, incrementSeconds };
}

/** I tempi con orologio, dal più veloce al più lento. */
const TIMED: readonly [TimeControl, string][] = [
  [minutes(1, 0), 'Bullet'],
  [minutes(2, 1), 'Bullet'],
  [minutes(3, 0), 'Blitz'],
  [minutes(3, 2), 'Blitz'],
  [minutes(5, 0), 'Blitz'],
  [minutes(5, 3), 'Blitz'],
  [minutes(10, 0), 'Rapid'],
  [minutes(10, 5), 'Rapid'],
  [minutes(15, 10), 'Rapid'],
  [minutes(30, 0), 'Classica'],
  [minutes(30, 20), 'Classica'],
];

/** Tutte le scelte, nell'ordine in cui compaiono. */
export const TIME_PRESETS: readonly TimePreset[] = [
  { id: 'none', timeControl: null, category: 'Libera' },
  ...TIMED.map(([timeControl, category]) => ({
    id: describeTimeControl(timeControl),
    timeControl,
    category,
  })),
];

/**
 * @param timeControl - un tempo (ad esempio quello dell'ultima partita).
 * @returns il nome della scelta corrispondente, oppure `"custom"` se non è tra quelli proposti.
 */
export function presetIdFor(timeControl: TimeControl | null): string {
  const found = TIME_PRESETS.find(
    (preset) =>
      preset.timeControl?.initialSeconds === timeControl?.initialSeconds &&
      preset.timeControl?.incrementSeconds === timeControl?.incrementSeconds,
  );
  return found?.id ?? CUSTOM_PRESET_ID;
}
