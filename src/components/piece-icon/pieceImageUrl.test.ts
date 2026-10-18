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
  it('trova un’immagine per ogni pezzo di ogni stile', () => {
    for (const style of STYLES) {
      for (const piece of PIECES) {
        expect(pieceImageUrl(style, piece)).toContain(`${piece}.svg`);
      }
    }
  });
});
