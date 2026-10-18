/**
 * Decide quale tema mostrare davvero.
 *
 * I temi "concreti" hanno un loro foglio di stile (src/styles/themes/).
 * Il tema "auto" non ne ha uno: diventa Sera o Giorno a seconda della
 * modalità del sistema operativo.
 */
import type { AppTheme } from '../../bindings/AppTheme';

/** I temi che hanno un foglio di stile. */
export type ConcreteTheme = Exclude<AppTheme, 'auto'>;

/**
 * @param theme - il tema scelto dall'utente.
 * @param systemPrefersDark - `true` se il sistema operativo è in modalità scura.
 * @returns il tema da applicare.
 */
export function resolveTheme(theme: AppTheme, systemPrefersDark: boolean): ConcreteTheme {
  if (theme === 'auto') {
    return systemPrefersDark ? 'sera' : 'giorno';
  }
  return theme;
}
