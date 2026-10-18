/**
 * useColorSchemePreference — dice se il sistema operativo è in modalità scura.
 *
 * Si aggiorna da solo quando l'utente cambia la modalità di Windows, macOS o Linux
 * mentre l'app è aperta.
 *
 * @returns `true` se il sistema è in modalità scura, `false` se è in modalità chiara.
 */
import { useSyncExternalStore } from 'react';

/** La domanda che il motore web pone al sistema operativo. */
const DARK_MODE_QUERY = '(prefers-color-scheme: dark)';

/** Si mette in ascolto dei cambi di modalità; restituisce la funzione per smettere. */
function subscribe(onChange: () => void): () => void {
  const query = window.matchMedia(DARK_MODE_QUERY);
  query.addEventListener('change', onChange);
  return () => {
    query.removeEventListener('change', onChange);
  };
}

/** Legge la modalità attuale. */
function readPreference(): boolean {
  return window.matchMedia(DARK_MODE_QUERY).matches;
}

export function useColorSchemePreference(): boolean {
  return useSyncExternalStore(subscribe, readPreference);
}
