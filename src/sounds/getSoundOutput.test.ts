/**
 * Test dell'uscita audio comune.
 */
import { afterEach, describe, expect, it, vi } from 'vitest';
import { FakeAudioContext } from '../test/fakeAudioContext';
import { getSoundOutput } from './getSoundOutput';

describe('getSoundOutput', () => {
  afterEach(() => {
    vi.unstubAllGlobals();
  });

  // Va per primo: dopo, il mixer esiste già e viene riusato.
  it('restituisce null se il motore web non sa produrre suoni', () => {
    expect(getSoundOutput()).toBeNull();
  });

  it('crea un solo mixer e lo riattiva se è sospeso', () => {
    vi.stubGlobal('AudioContext', FakeAudioContext);

    const first = getSoundOutput();
    const context = FakeAudioContext.instances[0];
    if (context !== undefined) {
      context.state = 'suspended';
    }
    const second = getSoundOutput();

    expect(first).not.toBeNull();
    expect(second).toBe(first);
    expect(FakeAudioContext.instances).toHaveLength(1);
    expect(context?.resumeCalls).toBe(1);
  });
});
