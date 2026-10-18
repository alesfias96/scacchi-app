/**
 * Servizio della partita: le chiamate al nucleo Rust che riguardano la partita in corso.
 *
 * Insieme agli altri file di questa cartella, è l'unico punto dell'interfaccia
 * che conosce i nomi dei comandi Rust. Ogni funzione restituisce la partita
 * aggiornata (`GameView`), così l'interfaccia ha sempre la situazione completa.
 *
 * Tutte le funzioni lanciano un errore se la comunicazione con Rust non riesce
 * (ad esempio se l'interfaccia è aperta nel browser invece che nell'app desktop).
 */
import { invoke } from '@tauri-apps/api/core';
import type { GameView } from '../bindings/GameView';

/**
 * Chiede a Rust la partita in corso.
 *
 * @returns la partita in corso (all'avvio: una partita nuova).
 */
export function fetchGame(): Promise<GameView> {
  return invoke<GameView>('get_game');
}

/**
 * Comincia una nuova partita dalla posizione iniziale.
 *
 * @returns la nuova partita.
 */
export function startNewGame(): Promise<GameView> {
  return invoke<GameView>('new_game');
}

/**
 * Gioca una mossa nella posizione mostrata.
 * Se la posizione mostrata non è l'ultima, le mosse successive vengono sostituite.
 *
 * @param uci - la mossa in notazione UCI (es. `"e2e4"`, `"e7e8q"`).
 * @returns la partita aggiornata.
 * @throws se la mossa non è legale o la partita è finita (con il motivo).
 */
export function playMove(uci: string): Promise<GameView> {
  return invoke<GameView>('play_move', { uci });
}

/**
 * Ritira l'ultima mossa della partita.
 *
 * @returns la partita aggiornata.
 */
export function undoMove(): Promise<GameView> {
  return invoke<GameView>('undo_move');
}

/**
 * Mostra la posizione dopo un certo numero di mosse.
 *
 * @param ply - quante mosse (0 = posizione di partenza).
 * @returns la partita con la posizione richiesta.
 */
export function goToPly(ply: number): Promise<GameView> {
  return invoke<GameView>('go_to_ply', { ply });
}

/**
 * Sostituisce la partita con quella descritta da un testo FEN o PGN.
 *
 * @param text - il testo incollato dall'utente.
 * @returns la nuova partita.
 * @throws se il testo non è una posizione o una partita valida (con il motivo).
 */
export function importGame(text: string): Promise<GameView> {
  return invoke<GameView>('import_game_text', { text });
}

/**
 * Chiede a Rust la partita in formato PGN.
 *
 * @returns il testo PGN, pronto da copiare.
 */
export function exportPgn(): Promise<string> {
  return invoke<string>('export_pgn');
}
