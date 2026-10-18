/**
 * Test del suono della presa (con il finto mixer audio).
 */
import { afterAll, beforeAll, describe, expect, it, vi } from 'vitest';
import { FakeAudioContext } from '../test/fakeAudioContext';
import { playCaptureSound } from './playCaptureSound';

describe('playCaptureSound', () => {
  beforeAll(() => {
    vi.stubGlobal('AudioContext', FakeAudioContext);
  });
  afterAll(() => {
    vi.unstubAllGlobals();
  });

  it('suona due colpi ravvicinati, il secondo subito dopo il primo', () => {
    playCaptureSound();

    const starts = FakeAudioContext.instances[0]?.started.map((source) => source.at) ?? [];
    expect(starts).toHaveLength(4);
    const [first, , second] = starts;
    expect(first).toBe(10);
    expect(second).toBeCloseTo(10.035);
  });
});
