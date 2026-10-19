/**
 * Test della voce «Tutor» in cima alla colonna di destra.
 */
import { act, fireEvent, renderHook } from '@testing-library/react';
import { beforeEach, describe, expect, it } from 'vitest';
import { useTutorStore } from '../state/tutorStore';
import { useTutorTab } from './useTutorTab';

describe('useTutorTab', () => {
  beforeEach(() => {
    useTutorStore.setState({ isOpen: false, target: null });
  });

  it('il tasto T passa al tutor e torna indietro, e il tutor sa quando è mostrato', () => {
    const { result } = renderHook(() => useTutorTab());
    expect(result.current[0]).toBe('main');

    act(() => {
      fireEvent.keyDown(window, { key: 't' });
    });
    expect(result.current[0]).toBe('tutor');
    expect(useTutorStore.getState().isOpen).toBe(true);

    act(() => {
      fireEvent.keyDown(window, { key: 'T' });
    });
    expect(result.current[0]).toBe('main');
    expect(useTutorStore.getState().isOpen).toBe(false);
  });

  it('chiudendo la sezione il tutor non è più mostrato', () => {
    const { result, unmount } = renderHook(() => useTutorTab());
    act(() => {
      result.current[1]('tutor');
    });

    unmount();

    expect(useTutorStore.getState().isOpen).toBe(false);
  });
});
