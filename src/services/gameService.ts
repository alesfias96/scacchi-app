/**
 * Servizio della partita: le chiamate al nucleo Rust che riguardano la partita in corso.
 *
 * Insieme agli altri file di questa cartella, è l'unico punto dell'interfaccia
 * che conosce i nomi dei comandi e degli eventi Rust. Ogni funzione restituisce
 * la partita aggiornata (`SessionView`), così l'interfaccia ha sempre la
 * situazione completa.
 *
 * Tutte le funzioni lanciano un errore se la comunicazione con Rust non riesce
 * (ad esempio se l'interfaccia è aperta nel browser invece che nell'app desktop).
 */
import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import type { GameSetup } from '../bindings/GameSetup';
import type { SessionView } from '../bindings/SessionView';

/** L'evento con cui Rust annuncia una partita cambiata da sola (vedi src-tauri/src/events/). */
const GAME_UPDATED_EVENT = 'game-updated';

/**
 * Chiede a Rust la partita in corso.
 *
 * @returns la partita in corso (all'avvio: una partita nuova a due giocatori).
 */
export function fetchGame(): Promise<SessionView> {
  return invoke<SessionView>('get_game');
}

/**
 * Comincia una nuova partita.
 *
 * @param setup - avversario, colore, livello del computer e tempo.
 * @returns la nuova partita.
 */
export function startNewGame(setup: GameSetup): Promise<SessionView> {
  return invoke<SessionView>('new_game', { setup });
}

/**
 * Gioca una mossa nella posizione mostrata.
 *
 * @param uci - la mossa in notazione UCI (es. `"e2e4"`, `"e7e8q"`).
 * @returns la partita aggiornata.
 * @throws se la mossa non è accettata (non legale, partita finita, tocca al computer…), con il motivo.
 */
export function playMove(uci: string): Promise<SessionView> {
  return invoke<SessionView>('play_move', { uci });
}

/**
 * Ritira l'ultima mossa (contro il computer anche la sua risposta).
 *
 * @returns la partita aggiornata.
 * @throws se le mosse non si possono ritirare (partita con l'orologio).
 */
export function undoMove(): Promise<SessionView> {
  return invoke<SessionView>('undo_move');
}

/**
 * Mostra la posizione dopo un certo numero di mosse.
 *
 * @param ply - quante mosse (0 = posizione di partenza).
 * @returns la partita con la posizione richiesta.
 */
export function goToPly(ply: number): Promise<SessionView> {
  return invoke<SessionView>('go_to_ply', { ply });
}

/**
 * Abbandona la partita.
 *
 * @returns la partita finita.
 * @throws se la partita era già finita.
 */
export function resignGame(): Promise<SessionView> {
  return invoke<SessionView>('resign_game');
}

/**
 * Sostituisce la partita con quella descritta da un testo FEN o PGN.
 *
 * @param text - il testo incollato dall'utente.
 * @returns la nuova partita.
 * @throws se il testo non è una posizione o una partita valida (con il motivo).
 */
export function importGame(text: string): Promise<SessionView> {
  return invoke<SessionView>('import_game_text', { text });
}

/**
 * Chiede a Rust la partita in formato PGN.
 *
 * @returns il testo PGN, pronto da copiare.
 */
export function exportPgn(): Promise<string> {
  return invoke<string>('export_pgn');
}

/**
 * Salva la partita in un file `.pgn`, scelto con la finestra del sistema.
 *
 * @returns `true` se il file è stato salvato, `false` se l'utente ha annullato.
 * @throws se il file non si può scrivere (con il motivo).
 */
export function savePgnFile(): Promise<boolean> {
  return invoke<boolean>('save_pgn_file');
}

/**
 * Apre un file `.pgn`, scelto con la finestra del sistema, al posto della partita in corso.
 *
 * @returns la nuova partita, oppure `null` se l'utente ha annullato.
 * @throws se il file non si legge o non contiene una partita valida (con il motivo).
 */
export function openPgnFile(): Promise<SessionView | null> {
  return invoke<SessionView | null>('open_pgn_file');
}

/**
 * Ascolta le partite cambiate "da sole": mossa del computer, tempo scaduto.
 *
 * @param onUpdate - chiamata con la partita aggiornata a ogni cambiamento.
 * @returns la funzione che smette di ascoltare.
 */
export function listenToGameUpdates(onUpdate: (view: SessionView) => void): Promise<UnlistenFn> {
  return listen<SessionView>(GAME_UPDATED_EVENT, (event) => {
    onUpdate(event.payload);
  });
}
