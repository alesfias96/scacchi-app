/**
 * Test della navigazione da tastiera.
 */
import { fireEvent, render } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { useGameKeyboard, type GameKeyboardActions } from './useGameKeyboard';

/** Un componente di prova che usa l'hook. */
function KeyboardProbe({ actions, enabled }: { actions: GameKeyboardActions; enabled?: boolean }) {
  useGameKeyboard(actions, enabled);
  return <input aria-label="campo" />;
}

/** Azioni simulate. */
function createActions(): GameKeyboardActions {
  return { onPrevious: vi.fn(), onNext: vi.fn(), onFirst: vi.fn(), onLast: vi.fn() };
}

describe('useGameKeyboard', () => {
  it('collega frecce, Home e Fine alle azioni', () => {
    const actions = createActions();
    render(<KeyboardProbe actions={actions} />);

    fireEvent.keyDown(window, { key: 'ArrowLeft' });
    fireEvent.keyDown(window, { key: 'ArrowRight' });
    fireEvent.keyDown(window, { key: 'Home' });
    fireEvent.keyDown(window, { key: 'End' });

    expect(actions.onPrevious).toHaveBeenCalledOnce();
    expect(actions.onNext).toHaveBeenCalledOnce();
    expect(actions.onFirst).toHaveBeenCalledOnce();
    expect(actions.onLast).toHaveBeenCalledOnce();
  });

  it('ignora i tasti mentre si scrive in un campo', () => {
    const actions = createActions();
    const { getByLabelText } = render(<KeyboardProbe actions={actions} />);

    fireEvent.keyDown(getByLabelText('campo'), { key: 'ArrowLeft' });

    expect(actions.onPrevious).not.toHaveBeenCalled();
  });

  it('ignora i tasti quando è disattivato', () => {
    const actions = createActions();
    render(<KeyboardProbe actions={actions} enabled={false} />);

    fireEvent.keyDown(window, { key: 'ArrowRight' });

    expect(actions.onNext).not.toHaveBeenCalled();
  });
});
