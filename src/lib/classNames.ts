/**
 * Unisce più nomi di classi CSS in un'unica stringa, scartando quelli vuoti.
 *
 * Serve per applicare classi in modo condizionale senza concatenazioni a mano:
 *   classNames(styles.item, isActive && styles.active)
 *   → "item active" se isActive è vero, altrimenti "item".
 *
 * @param names - nomi di classi; `undefined`, `false`, `null` e stringhe vuote vengono ignorati.
 * @returns i nomi validi separati da uno spazio.
 */
export function classNames(...names: (string | false | null | undefined)[]): string {
  return names.filter((name): name is string => typeof name === 'string' && name !== '').join(' ');
}
