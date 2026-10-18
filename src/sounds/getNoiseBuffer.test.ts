/**
 * Test del fruscio usato per il colpo dei pezzi.
 */
import { describe, expect, it } from 'vitest';
import { FakeAudioContext } from '../test/fakeAudioContext';
import { getNoiseBuffer } from './getNoiseBuffer';

describe('getNoiseBuffer', () => {
  it('calcola il fruscio una volta sola per ogni mixer', () => {
    const context = new FakeAudioContext() as unknown as BaseAudioContext;

    expect(getNoiseBuffer(context)).toBe(getNoiseBuffer(context));
  });

  it('riempie il fruscio con valori tra -1 e 1, non tutti uguali', () => {
    const context = new FakeAudioContext() as unknown as BaseAudioContext;

    const samples = Array.from(getNoiseBuffer(context).getChannelData(0));

    expect(samples.every((sample) => sample >= -1 && sample <= 1)).toBe(true);
    expect(new Set(samples).size).toBeGreaterThan(1);
  });
});
