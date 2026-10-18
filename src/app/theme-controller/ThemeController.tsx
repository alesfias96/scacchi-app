/**
 * ThemeController — applica il tema scelto a tutta l'app.
 *
 * Cosa fa: scrive il nome del tema nell'attributo `data-theme` dell'elemento
 * <html>. I fogli di stile in src/styles/themes/ reagiscono a quell'attributo,
 * quindi tutta l'interfaccia cambia colori e font in un colpo solo.
 *
 * Riceve: niente (legge il tema dalle impostazioni).
 * Restituisce: niente da disegnare.
 */
import { useEffect } from 'react';
import { useColorSchemePreference } from '../../hooks/useColorSchemePreference';
import { useLoadedSettings } from '../../state/settingsStore';
import { resolveTheme } from './resolveTheme';

export function ThemeController() {
  const { theme } = useLoadedSettings();
  const systemPrefersDark = useColorSchemePreference();
  const activeTheme = resolveTheme(theme, systemPrefersDark);

  useEffect(() => {
    document.documentElement.dataset.theme = activeTheme;
  }, [activeTheme]);

  return null;
}
