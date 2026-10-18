/**
 * Test di describeError.
 */
import { describe, expect, it } from 'vitest';
import { describeError } from './describeError';

describe('describeError', () => {
  it('restituisce il testo così com’è quando l’errore è una stringa', () => {
    expect(describeError('file non trovato')).toBe('file non trovato');
  });

  it('usa il messaggio degli oggetti Error', () => {
    expect(describeError(new Error('connessione persa'))).toBe('connessione persa');
  });

  it('dà un messaggio generico per tutto il resto', () => {
    expect(describeError(42)).toBe('Errore sconosciuto nella comunicazione con il nucleo Rust.');
  });
});
