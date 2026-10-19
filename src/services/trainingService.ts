/**
 * Servizio dell'allenamento: le chiamate al nucleo Rust sui puzzle.
 *
 * Insieme agli altri file di questa cartella, è l'unico punto dell'interfaccia
 * che conosce i nomi dei comandi Rust.
 */
import { invoke } from '@tauri-apps/api/core';
import type { AttemptView } from '../bindings/AttemptView';
import type { TrainingOverview } from '../bindings/TrainingOverview';
import type { TrainingSetup } from '../bindings/TrainingSetup';

/**
 * Chiede a Rust il riepilogo dell'allenamento.
 *
 * @returns rating, puzzle affrontati e risolti, ripassi di oggi, temi disponibili.
 * @throws se il database non si può leggere.
 */
export function fetchTrainingOverview(): Promise<TrainingOverview> {
  return invoke<TrainingOverview>('get_training_overview');
}

/**
 * Comincia un puzzle adatto alle scelte fatte (oppure uno da ripassare).
 *
 * @param setup - modalità, profondità, tema, difficoltà.
 * @returns il puzzle appena cominciato.
 * @throws se nessun puzzle va bene per le scelte fatte (con il motivo).
 */
export function startPuzzle(setup: TrainingSetup): Promise<AttemptView> {
  return invoke<AttemptView>('start_puzzle', { setup });
}

/**
 * Prova una mossa nel puzzle in corso.
 *
 * @param uci - la mossa in notazione UCI (es. `"e2e4"`, `"e7e8q"`).
 * @returns il puzzle aggiornato.
 */
export function playPuzzleMove(uci: string): Promise<AttemptView> {
  return invoke<AttemptView>('play_puzzle_move', { uci });
}

/**
 * Chiede un aiuto: il pezzo da muovere viene indicato.
 *
 * @returns il puzzle aggiornato (con la casa del pezzo).
 */
export function requestPuzzleHint(): Promise<AttemptView> {
  return invoke<AttemptView>('puzzle_hint');
}

/**
 * Mostra la soluzione del puzzle in corso.
 *
 * @returns il puzzle finito, con la soluzione giocata sulla scacchiera.
 */
export function revealPuzzleSolution(): Promise<AttemptView> {
  return invoke<AttemptView>('show_puzzle_solution');
}
