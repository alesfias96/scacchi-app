/**
 * Quanta parte della barra di valutazione spetta al Bianco.
 *
 * Il vantaggio in centipedoni viene trasformato in una "probabilità di
 * vittoria" con la stessa curva usata da Lichess: vicino alla parità la
 * barra si muove molto, con vantaggi grandi si avvicina ai bordi senza mai
 * toccarli. Uno scacco matto in vista riempie la barra.
 *
 * Questo calcolo serve solo a disegnare: il giudizio sulla posizione lo dà Rust.
 */
import type { Evaluation } from '../../bindings/Evaluation';

/** La pendenza della curva (valore di Lichess, ricavato da milioni di partite). */
const SLOPE = 0.00368208;

/**
 * @param evaluation - la valutazione dal punto di vista del Bianco.
 * @returns un numero da 0 (tutto Nero) a 1 (tutto Bianco); 0,5 = parità.
 */
export function whiteShare(evaluation: Evaluation): number {
  if (evaluation.kind === 'mate') {
    if (evaluation.value === 0) {
      return 0.5;
    }
    return evaluation.value > 0 ? 1 : 0;
  }
  return 1 / (1 + Math.exp(-SLOPE * evaluation.value));
}
