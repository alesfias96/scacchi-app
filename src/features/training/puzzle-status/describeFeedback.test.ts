/**
 * Test dei messaggi durante un puzzle.
 */
import { describe, expect, it } from 'vitest';
import { READY_ATTEMPT, SOLVED_ATTEMPT } from '../../../test/puzzleAttempts';
import { describeFeedback } from './describeFeedback';

describe('describeFeedback', () => {
  it('all’inizio dice chi muove e quante mosse trovare', () => {
    expect(describeFeedback(READY_ATTEMPT, false)).toEqual({
      text: 'Muove il Bianco: trova la mossa migliore (1 mossa da trovare).',
      tone: 'neutral',
    });
    expect(describeFeedback({ ...READY_ATTEMPT, depth: 3 }, false).text).toContain('3 mosse');
  });

  it('nella visualizzazione invita a memorizzare', () => {
    expect(describeFeedback({ ...READY_ATTEMPT, mode: 'visualization' }, true).text).toBe(
      'Memorizza la posizione: tra poco i pezzi spariscono.',
    );
  });

  it('in "cosa minaccia?" chiede la mossa dell’avversario', () => {
    expect(describeFeedback({ ...READY_ATTEMPT, mode: 'threat' }, false).text).toContain(
      'Cosa minaccia l',
    );
  });

  it('colora le mosse giuste e sbagliate', () => {
    expect(describeFeedback({ ...READY_ATTEMPT, feedback: 'correct' }, false).tone).toBe('good');
    expect(describeFeedback({ ...READY_ATTEMPT, feedback: 'wrong' }, false)).toEqual({
      text: 'Non è la mossa giusta: riprova.',
      tone: 'bad',
    });
  });

  it('distingue un puzzle risolto al primo colpo da uno risolto dopo un errore', () => {
    expect(describeFeedback(SOLVED_ATTEMPT, false)).toEqual({ text: 'Risolto!', tone: 'good' });
    expect(describeFeedback({ ...SOLVED_ATTEMPT, clean: false }, false).text).toContain(
      'conta come sbagliato',
    );
  });
});
