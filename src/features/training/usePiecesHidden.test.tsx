/**
 * Test dei pezzi nascosti nella visualizzazione.
 */
import { act, renderHook } from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import type { AttemptView } from '../../bindings/AttemptView';
import { READY_ATTEMPT, SOLVED_ATTEMPT } from '../../test/puzzleAttempts';
import { usePiecesHidden } from './usePiecesHidden';

const VISUALIZATION: AttemptView = { ...READY_ATTEMPT, mode: 'visualization' };

describe('usePiecesHidden', () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it('nasconde i pezzi dopo i secondi scelti', () => {
    const { result } = renderHook(() => usePiecesHidden(VISUALIZATION, 5));
    expect(result.current).toEqual({ hidden: false, memorizing: true });

    act(() => {
      vi.advanceTimersByTime(5000);
    });

    expect(result.current).toEqual({ hidden: true, memorizing: false });
  });

  it('a puzzle finito i pezzi tornano visibili', () => {
    const { result, rerender } = renderHook(
      ({ attempt }: { attempt: AttemptView }) => usePiecesHidden(attempt, 2),
      { initialProps: { attempt: VISUALIZATION } },
    );
    act(() => {
      vi.advanceTimersByTime(2000);
    });

    rerender({ attempt: { ...SOLVED_ATTEMPT, mode: 'visualization' } });

    expect(result.current.hidden).toBe(false);
  });

  it('fuori dalla visualizzazione non nasconde nulla', () => {
    const { result } = renderHook(() => usePiecesHidden(READY_ATTEMPT, 2));

    act(() => {
      vi.advanceTimersByTime(10_000);
    });

    expect(result.current).toEqual({ hidden: false, memorizing: false });
  });
});
