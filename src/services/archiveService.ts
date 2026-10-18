/**
 * Servizio dell'archivio: le chiamate al nucleo Rust sulle partite salvate.
 *
 * Insieme agli altri file di questa cartella, è l'unico punto dell'interfaccia
 * che conosce i nomi dei comandi e degli eventi Rust.
 */
import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import type { GameFilter } from '../bindings/GameFilter';
import type { GameSummary } from '../bindings/GameSummary';
import type { SessionView } from '../bindings/SessionView';

/** L'evento con cui Rust segnala un problema del database delle partite. */
const STORAGE_ERROR_EVENT = 'storage-error';

/**
 * Chiede a Rust le partite salvate che rispettano i filtri.
 *
 * @param filter - avversario, esito e testo da cercare (i valori `null` non filtrano).
 * @returns le partite, dalla più recente.
 * @throws se il database non si può leggere (con il motivo).
 */
export function listGames(filter: GameFilter): Promise<GameSummary[]> {
  return invoke<GameSummary[]>('list_games', { filter });
}

/**
 * Apre una partita salvata in "Gioca" (da rivedere o da continuare).
 *
 * @param id - il nome unico della partita.
 * @returns la partita aperta.
 * @throws se la partita non esiste più o è rovinata.
 */
export function openSavedGame(id: string): Promise<SessionView> {
  return invoke<SessionView>('open_game', { id });
}

/**
 * Elimina una partita dall'archivio.
 *
 * @param id - il nome unico della partita.
 * @throws se il database non si può scrivere.
 */
export async function deleteSavedGame(id: string): Promise<void> {
  await invoke('delete_game', { id });
}

/**
 * Ascolta i problemi del database (ad esempio una partita che non si salva).
 *
 * @param onError - chiamata con il messaggio da mostrare.
 * @returns la funzione che smette di ascoltare.
 */
export function listenToStorageErrors(onError: (message: string) => void): Promise<UnlistenFn> {
  return listen<string>(STORAGE_ERROR_EVENT, (event) => {
    onError(event.payload);
  });
}
