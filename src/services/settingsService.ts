/**
 * Servizio delle impostazioni: lettura e salvataggio delle scelte dell'utente.
 *
 * Insieme agli altri file di questa cartella, è l'unico punto dell'interfaccia
 * che conosce i nomi dei comandi Rust.
 */
import { invoke } from '@tauri-apps/api/core';
import type { Settings } from '../bindings/Settings';

/**
 * Chiede a Rust le impostazioni salvate.
 *
 * @returns le impostazioni salvate, oppure quelle predefinite al primo avvio.
 * @throws se la comunicazione con Rust non riesce.
 */
export function fetchSettings(): Promise<Settings> {
  return invoke<Settings>('get_settings');
}

/**
 * Chiede a Rust di salvare le impostazioni.
 *
 * @param settings - le impostazioni complete da salvare.
 * @throws con un messaggio leggibile se il salvataggio non riesce.
 */
export async function saveSettings(settings: Settings): Promise<void> {
  await invoke('save_settings', { settings });
}
