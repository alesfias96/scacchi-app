/**
 * Servizio del tutor: le chiamate al nucleo Rust per spiegare le posizioni.
 *
 * Insieme agli altri file di questa cartella, è l'unico punto dell'interfaccia
 * che conosce i nomi dei comandi Rust.
 */
import { invoke } from '@tauri-apps/api/core';
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
