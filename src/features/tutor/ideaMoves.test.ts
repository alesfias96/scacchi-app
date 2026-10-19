/**
 * Test delle mosse di un'idea.
 */
import { describe, expect, it } from 'vitest';
import type { IdeaView } from '../../bindings/IdeaView';
import { ideaMoveLabel } from './ideaMoves';

/** Un'idea che comincia con una mossa del Nero alla dodicesima mossa. */
const IDEA: IdeaView = {
  side: 'black',
  headline: '',
  keyPly: null,
  evaluation: null,
  moves: ['e5', 'Nf3', 'Nc6'],
  firstMoveNumber: 12,
  firstSide: 'black',
  moments: [],
  frames: [],
  cards: [],
};

describe('ideaMoveLabel', () => {
  it('numera le mosse come nei libri, anche se la variante comincia con il Nero', () => {
    expect(ideaMoveLabel(IDEA, 0, 'internazionale')).toBe('12… e5');
    expect(ideaMoveLabel(IDEA, 1, 'internazionale')).toBe('13. Nf3');
    expect(ideaMoveLabel(IDEA, 2, 'internazionale')).toBe('13… Nc6');
    expect(ideaMoveLabel(IDEA, 5, 'internazionale')).toBe('');
  });
});
