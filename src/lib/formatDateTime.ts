/**
 * Scrive una data e un'ora in italiano, in forma breve.
 *
 * @param unixMs - l'istante, in millesimi di secondo dal 1° gennaio 1970
 *   (come lo salva Rust).
 * @returns ad esempio `"18 ott 2026, 16:35"`.
 */

/** Il formato: giorno, mese abbreviato, anno, ore e minuti. */
const FORMAT = new Intl.DateTimeFormat('it-IT', { dateStyle: 'medium', timeStyle: 'short' });

export function formatDateTime(unixMs: number): string {
  return FORMAT.format(new Date(unixMs));
}
