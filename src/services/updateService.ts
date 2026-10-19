/**
 * Servizio degli aggiornamenti: le chiamate al nucleo Rust per sapere la
 * versione installata, controllare se ce n'è una nuova e installarla.
 *
 * Insieme agli altri file di questa cartella, è l'unico punto dell'interfaccia
 * che conosce i nomi dei comandi Rust.
 */
import { invoke } from '@tauri-apps/api/core';

/**
 * Chiede a Rust la versione dell'app installata.
 *
 * @returns ad esempio `"0.10.0"`.
 */
export function fetchAppVersion(): Promise<string> {
  return invoke<string>('app_version');
}

/**
 * Controlla su Internet se c'è una versione più nuova.
 *
 * @returns la versione nuova, oppure `null` se si ha già l'ultima.
 * @throws se non ci si riesce a collegare.
 */
export function checkForUpdate(): Promise<string | null> {
  return invoke<string | null>('check_update');
}

/**
 * Scarica, controlla la firma e installa la versione nuova; poi l'app si riavvia.
 *
 * @throws se lo scaricamento o l'installazione non riescono.
 */
export async function installUpdate(): Promise<void> {
  await invoke('install_update');
}
