/**
 * Test del suono della mossa (con il finto mixer audio).
 */
import { afterAll, beforeAll, describe, expect, it, vi } from 'vitest';
import { FakeAudioContext } from '../test/fakeAudioContext';
import { playMoveSound } from './playMoveSound';

describe('playMoveSound', () => {
  beforeAll(() => {
    vi.stubGlobal('AudioContext', FakeAudioContext);
  });
  afterAll(() => {
    vi.unstubAllGlobals();
  });

  it('suona un colpo (clic e corpo) dopo il ritardo indicato', () => {
    playMoveSound(0.5);

    expect(FakeAudioContext.instances[0]?.started).toEqual([
      { kind: 'buffer', at: 10.5 },
      { kind: 'oscillator', at: 10.5 },
    ]);
  });
});
