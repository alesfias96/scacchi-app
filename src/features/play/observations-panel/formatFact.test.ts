/**
 * Test della scrittura dei fatti dei rilevatori.
 */
import { describe, expect, it } from 'vitest';
import type { Fact } from '../../../bindings/Fact';
import { formatFact } from './formatFact';

/** Un fatto di prova. */
function fact(text: string, moves: string[]): Fact {
  return { category: 'threats', side: 'white', text, squares: [], arrows: [], moves };
}

describe('formatFact', () => {
  it('lascia com’è una frase senza mosse', () => {
    expect(formatFact(fact('Il pedone bianco in d4 è isolato', []), 'figurine')).toBe(
      'Il pedone bianco in d4 è isolato',
    );
  });

  it('aggiunge le mosse nella notazione scelta', () => {
    const mate = fact('Il Bianco può dare scacco matto in una mossa con', ['Qxf7#', 'Bxf7#']);

    expect(formatFact(mate, 'italiana')).toBe(
      'Il Bianco può dare scacco matto in una mossa con Dxf7# oppure Axf7#',
    );
  });
});
