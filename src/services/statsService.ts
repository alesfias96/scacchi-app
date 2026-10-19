/**
 * Servizio delle statistiche: le chiamate al nucleo Rust per la sezione «Progressi».
 *
 * Insieme agli altri file di questa cartella, è l'unico punto dell'interfaccia
 * che conosce i nomi dei comandi e degli eventi Rust.
 */
import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import type { ReviewQueueProgress } from '../bindings/ReviewQueueProgress';
import type { StatsFilter } from '../bindings/StatsFilter';
import type { StatsReport } from '../bindings/StatsReport';

/** L'evento con l'avanzamento della coda di partite da analizzare. */
const REVIEW_QUEUE_EVENT = 'review-queue';

/**
 * Chiede a Rust le statistiche.
 *
 * @param filter - periodo e avversario.
 * @returns le statistiche di tutte le schede.
 * @throws se il database non si può leggere.
 */
export function fetchStats(filter: StatsFilter): Promise<StatsReport> {
  return invoke<StatsReport>('get_stats', { filter });
}

/**
 * Mette in coda, da analizzare una alla volta, tutte le partite finite non ancora analizzate.
 *
 * @returns quante partite sono in coda.
 */
export function analyzeMissingGames(): Promise<number> {
  return invoke<number>('analyze_missing_games');
}

/**
 * Ascolta l'avanzamento della coda di partite da analizzare.
 *
 * @param onProgress - chiamata a ogni partita della coda (e alla fine).
 * @returns la funzione che smette di ascoltare.
 */
export function listenToReviewQueue(
  onProgress: (progress: ReviewQueueProgress) => void,
): Promise<UnlistenFn> {
  return listen<ReviewQueueProgress>(REVIEW_QUEUE_EVENT, (event) => {
    onProgress(event.payload);
  });
}
