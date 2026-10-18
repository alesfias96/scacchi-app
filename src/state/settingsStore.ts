/**
 * Stato condiviso delle impostazioni (tema, scacchiera, pezzi).
 *
 * È l'unico punto in cui l'interfaccia conserva le impostazioni mentre l'app
 * è aperta. Ogni componente che ne ha bisogno le legge da qui, così una
 * modifica fatta nella sezione Impostazioni si vede subito in tutta l'app.
 *
 * Usa Zustand: una piccola libreria che permette a più componenti di leggere
 * lo stesso dato e di ridisegnarsi quando cambia.
 */
import { create } from 'zustand';
import type { Settings } from '../bindings/Settings';
import { describeError } from '../lib/describeError';
import { fetchSettings, saveSettings } from '../services/settingsService';

/** Cosa contiene lo stato e cosa si può fare con esso. */
export interface SettingsState {
  /** Le impostazioni attuali; `null` finché non sono state lette da Rust. */
  settings: Settings | null;
  /** Il messaggio dell'ultimo salvataggio fallito, oppure `null`. */
  saveError: string | null;
  /** Legge le impostazioni da Rust (una volta, all'avvio). */
  load: () => Promise<void>;
  /**
   * Cambia una o più impostazioni e le salva.
   * La modifica è visibile subito; se il salvataggio fallisce viene annullata
   * e `saveError` spiega il motivo.
   */
  update: (changes: Partial<Settings>) => Promise<void>;
}

export const useSettingsStore = create<SettingsState>()((set, get) => ({
  settings: null,
  saveError: null,

  load: async () => {
    const settings = await fetchSettings();
    set({ settings });
  },

  update: async (changes) => {
    const previous = get().settings;
    if (previous === null) {
      return;
    }
    const next: Settings = { ...previous, ...changes };
    set({ settings: next, saveError: null });
    try {
      await saveSettings(next);
    } catch (error) {
      set({ settings: previous, saveError: describeError(error) });
    }
  },
}));

/**
 * Restituisce le impostazioni già caricate.
 *
 * Va usato solo dentro `SettingsGate`, che mostra l'app soltanto dopo aver
 * letto le impostazioni: per questo il valore non è mai `null`.
 *
 * @throws se usato fuori da `SettingsGate` (errore di programmazione).
 */
export function useLoadedSettings(): Settings {
  const settings = useSettingsStore((state) => state.settings);
  if (settings === null) {
    throw new Error(
      'Impostazioni non ancora caricate: il componente deve stare dentro SettingsGate.',
    );
  }
  return settings;
}
