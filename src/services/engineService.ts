/**
 * Servizio del motore: le chiamate al nucleo Rust che riguardano Stockfish.
 *
 * Insieme agli altri file di questa cartella, è l'unico punto dell'interfaccia
 * che conosce i nomi dei comandi e degli eventi Rust.
 */
import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import type { AnalysisUpdate } from '../bindings/AnalysisUpdate';
import type { LevelInfo } from '../bindings/LevelInfo';

/** L'evento con cui Rust manda gli aggiornamenti dell'analisi. */
const ANALYSIS_UPDATE_EVENT = 'analysis-update';
/** L'evento con cui Rust segnala un problema del motore. */
const ENGINE_ERROR_EVENT = 'engine-error';

/**
 * Chiede a Rust i livelli di gioco del computer.
 *
 * @returns i dieci livelli, dal più debole al più forte.
 */
export function fetchEngineLevels(): Promise<LevelInfo[]> {
  return invoke<LevelInfo[]>('get_engine_levels');
}

/**
 * Accende o spegne l'analisi in tempo reale della posizione mostrata.
 *
 * @param enabled - `true` per accenderla.
 */
export async function setAnalysis(enabled: boolean): Promise<void> {
  await invoke('set_analysis', { enabled });
}

/**
 * Ascolta gli aggiornamenti dell'analisi.
 *
 * @param onUpdate - chiamata a ogni profondità completata dal motore.
 * @returns la funzione che smette di ascoltare.
 */
export function listenToAnalysis(onUpdate: (update: AnalysisUpdate) => void): Promise<UnlistenFn> {
  return listen<AnalysisUpdate>(ANALYSIS_UPDATE_EVENT, (event) => {
    onUpdate(event.payload);
  });
}

/**
 * Ascolta i problemi del motore (ad esempio Stockfish che non si avvia).
 *
 * @param onError - chiamata con il messaggio da mostrare.
 * @returns la funzione che smette di ascoltare.
 */
export function listenToEngineErrors(onError: (message: string) => void): Promise<UnlistenFn> {
  return listen<string>(ENGINE_ERROR_EVENT, (event) => {
    onError(event.payload);
  });
}
