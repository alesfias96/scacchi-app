/**
 * Servizio dell'analisi delle partite: le chiamate al nucleo Rust e i suoi eventi.
 *
 * Insieme agli altri file di questa cartella, è l'unico punto dell'interfaccia
 * che conosce i nomi dei comandi e degli eventi Rust.
 */
import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import type { GameReview } from '../bindings/GameReview';
import type { ReviewFinished } from '../bindings/ReviewFinished';
import type { ReviewProgress } from '../bindings/ReviewProgress';

/** L'evento con l'avanzamento dell'analisi. */
const REVIEW_PROGRESS_EVENT = 'review-progress';
/** L'evento di fine analisi. */
const REVIEW_FINISHED_EVENT = 'review-finished';

/**
 * Comincia ad analizzare una partita, in sottofondo.
 *
 * @param id - il nome unico della partita.
 * @throws se un'altra analisi è già in corso.
 */
export async function startReview(id: string): Promise<void> {
  await invoke('start_review', { id });
}

/** Ferma l'analisi in corso. */
export async function cancelReview(): Promise<void> {
  await invoke('cancel_review');
}

/**
 * La partita in analisi in questo momento.
 *
 * @returns il suo nome unico, oppure `null` se non c'è nessuna analisi in corso.
 */
export function fetchRunningReview(): Promise<string | null> {
  return invoke<string | null>('running_review');
}

/**
 * La revisione di una partita.
 *
 * @param id - il nome unico della partita.
 * @returns la revisione, oppure `null` se la partita non è ancora stata analizzata.
 */
export function fetchReview(id: string): Promise<GameReview | null> {
  return invoke<GameReview | null>('get_review', { id });
}

/**
 * Ascolta l'avanzamento dell'analisi.
 *
 * @param onProgress - chiamata dopo ogni posizione valutata.
 * @returns la funzione che smette di ascoltare.
 */
export function listenToReviewProgress(
  onProgress: (progress: ReviewProgress) => void,
): Promise<UnlistenFn> {
  return listen<ReviewProgress>(REVIEW_PROGRESS_EVENT, (event) => {
    onProgress(event.payload);
  });
}

/**
 * Ascolta la fine dell'analisi (riuscita, fermata o fallita).
 *
 * @param onFinished - chiamata alla fine.
 * @returns la funzione che smette di ascoltare.
 */
export function listenToReviewFinished(
  onFinished: (finished: ReviewFinished) => void,
): Promise<UnlistenFn> {
  return listen<ReviewFinished>(REVIEW_FINISHED_EVENT, (event) => {
    onFinished(event.payload);
  });
}
