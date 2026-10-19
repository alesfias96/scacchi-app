/**
 * Servizio del tutor: le chiamate al nucleo Rust per spiegare le posizioni
 * e le idee che si vedono guardando avanti (le varianti di Stockfish).
 *
 * Insieme agli altri file di questa cartella, è l'unico punto dell'interfaccia
 * che conosce i nomi dei comandi Rust.
 */
import { invoke } from '@tauri-apps/api/core';
import type { MoveIdea } from '../bindings/MoveIdea';
import type { PositionIdeas } from '../bindings/PositionIdeas';
import type { TutorExplanation } from '../bindings/TutorExplanation';
import type { WhatIfView } from '../bindings/WhatIfView';

/**
 * Chiede a Rust la spiegazione di una posizione.
 *
 * @param startFen - la posizione di partenza della partita (FEN).
 * @param moves - le mosse fino alla posizione da spiegare (UCI).
 * @param withEngine - se chiedere anche il giudizio di Stockfish (circa mezzo secondo).
 * @returns la spiegazione.
 * @throws se la posizione non è valida.
 */
export function explainPosition(
  startFen: string,
  moves: readonly string[],
  withEngine: boolean,
): Promise<TutorExplanation> {
  return invoke<TutorExplanation>('explain_position', { startFen, moves, withEngine });
}

/**
 * Prova una mossa ("e se…") e chiede cosa cambia. La partita vera non cambia.
 *
 * @param startFen - la posizione di partenza della partita (FEN).
 * @param moves - le mosse fino alla posizione in cui si prova (UCI), comprese quelle già provate.
 * @param uci - la mossa da provare.
 * @param withEngine - se chiedere anche il giudizio di Stockfish, prima e dopo la mossa.
 * @returns lo scenario.
 * @throws se la mossa non è legale.
 */
export function tryWhatIf(
  startFen: string,
  moves: readonly string[],
  uci: string,
  withEngine: boolean,
): Promise<WhatIfView> {
  return invoke<WhatIfView>('try_what_if', { startFen, moves, uci, withEngine });
}

/** Il messaggio con cui Rust dice che un calcolo è stato annullato da una richiesta più nuova. */
const CANCELLED_MESSAGE = 'la ricerca del motore è stata annullata';

/**
 * `true` se l'errore dice solo che il calcolo è stato annullato (perché nel
 * frattempo è partita una richiesta più nuova, ad esempio dopo un'altra mossa).
 * Non è un problema da mostrare.
 *
 * @param error - l'errore ricevuto da Rust.
 */
export function isCancelled(error: unknown): boolean {
  return error === CANCELLED_MESSAGE;
}

/**
 * Chiede a Rust le idee della posizione: le migliori varianti di Stockfish,
 * guardando lontano, ognuna spiegata.
 *
 * @param startFen - la posizione di partenza della partita (FEN).
 * @param moves - le mosse fino alla posizione (UCI).
 * @param thinkMs - quanto può pensare il motore, in millesimi di secondo.
 * @returns le idee, dalla migliore.
 * @throws se il motore non c'è, se il calcolo viene annullato (vedi `isCancelled`).
 */
export function fetchPositionIdeas(
  startFen: string,
  moves: readonly string[],
  thinkMs: number,
): Promise<PositionIdeas> {
  return invoke<PositionIdeas>('position_ideas', { startFen, moves, thinkMs });
}

/**
 * Chiede a Rust l'idea della mossa appena giocata e, se Stockfish preferiva
 * un'altra mossa, l'idea di quella.
 *
 * @param startFen - la posizione di partenza della partita (FEN).
 * @param movesBefore - le mosse prima della mossa giocata (UCI).
 * @param played - la mossa giocata (UCI).
 * @param thinkMs - quanto può pensare il motore per ciascuna posizione (prima e dopo).
 * @returns l'idea della mossa.
 * @throws come `fetchPositionIdeas`.
 */
export function fetchMoveIdea(
  startFen: string,
  movesBefore: readonly string[],
  played: string,
  thinkMs: number,
): Promise<MoveIdea> {
  return invoke<MoveIdea>('move_idea', { startFen, movesBefore, played, thinkMs });
}
