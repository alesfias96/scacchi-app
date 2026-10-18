/**
 * Test della ricerca delle immagini dei pezzi.
 */
import { describe, expect, it } from 'vitest';
import type { PieceStyle } from '../../bindings/PieceStyle';
import type { PieceCode } from './pieceCode';
import { pieceImageUrl } from './pieceImageUrl';

/** Tutti gli stili, come definiti in Rust. */
const STYLES: PieceStyle[] = ['icona', 'morbido', 'legno', 'metallo', 'gettoni', 'classico'];
/** Tutti i dodici pezzi. */
const PIECES: PieceCode[] = [
  'wK',
  'wQ',
  'wR',
  'wB',
  'wN',
  'wP',
  'bK',
  'bQ',
  'bR',
  'bB',
  'bN',
  'bP',
];

describe('pieceImageUrl', () => {
  // Vite inserisce le immagini piccole direttamente nel codice (indirizzi
  // `data:`), quindi non si può controllare il nome del file. Si verifica invece
  // che ogni combinazione abbia un'immagine e che siano tutte diverse: così si
  // scopre anche un file copiato per sbaglio da un altro stile o pezzo.
  it('trova un’immagine diversa per ogni pezzo di ogni stile', () => {
    const urls = new Set<string>();
    for (const style of STYLES) {
      for (const piece of PIECES) {
        urls.add(pieceImageUrl(style, piece));
      }
    }

    expect(urls.size).toBe(STYLES.length * PIECES.length);
  });
});
