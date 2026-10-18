/**
 * Scrive il tempo rimasto come sull'orologio da scacchi.
 *
 * - Da un'ora in su: `1:05:09`.
 * - Sotto l'ora: `4:07`.
 * - Negli ultimi 10 secondi compaiono i decimi (`9.4`), perché lì ogni
 *   istante conta.
 *
 * @param ms - il tempo rimasto, in millesimi di secondo.
 * @returns il testo da mostrare.
 */

/** Sotto questa soglia (in millesimi) si mostrano i decimi di secondo. */
const TENTHS_BELOW_MS = 10_000;

/** Un numero scritto sempre con due cifre (`7` → `07`). */
function twoDigits(value: number): string {
  return String(value).padStart(2, '0');
}

export function formatClock(ms: number): string {
  const safe = Math.max(0, ms);
  if (safe < TENTHS_BELOW_MS) {
    // Si tronca invece di arrotondare: "0.0" solo quando il tempo è davvero finito.
    return (Math.floor(safe / 100) / 10).toFixed(1);
  }
  const totalSeconds = Math.floor(safe / 1000);
  const hours = Math.floor(totalSeconds / 3600);
  const minutes = Math.floor((totalSeconds % 3600) / 60);
  const seconds = totalSeconds % 60;
  return hours > 0
    ? `${String(hours)}:${twoDigits(minutes)}:${twoDigits(seconds)}`
    : `${String(minutes)}:${twoDigits(seconds)}`;
}
