/**
 * Test del suono di fine partita (con il finto mixer audio).
 */
import { afterAll, beforeAll, beforeEach, describe, expect, it, vi } from 'vitest';
import { FakeAudioContext } from '../test/fakeAudioContext';
import { playGameEndSound } from './playGameEndSound';

/** Ogni nota è fatta di tre generatori: la nota e due armoniche. */
const SOURCES_PER_NOTE = 3;

/** Gli istanti di partenza dei generatori avviati, senza ripetizioni. */
function noteStarts(): number[] {
  return [...new Set(FakeAudioContext.instances[0]?.started.map((source) => source.at))];
}

describe('playGameEndSound', () => {
  beforeAll(() => {
    vi.stubGlobal('AudioContext', FakeAudioContext);
  });
  afterAll(() => {
    vi.unstubAllGlobals();
  });
  beforeEach(() => {
    FakeAudioContext.instances[0]?.started.splice(0);
  });

  it('per una partita decisa suona quattro note una dopo l’altra', () => {
    playGameEndSound('decisive', 0.2);

    expect(FakeAudioContext.instances[0]?.started).toHaveLength(4 * SOURCES_PER_NOTE);
    const starts = noteStarts();
    expect(starts).toHaveLength(4);
    expect(starts[0]).toBeCloseTo(10.2);
  });

  it('per una patta suona due note', () => {
    playGameEndSound('draw');

    expect(FakeAudioContext.instances[0]?.started).toHaveLength(2 * SOURCES_PER_NOTE);
    expect(noteStarts()).toHaveLength(2);
  });
});
