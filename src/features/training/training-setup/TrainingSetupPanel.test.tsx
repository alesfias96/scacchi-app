/**
 * Test della scheda con le scelte dell'allenamento.
 */
import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import type { TrainingSetup } from '../../../bindings/TrainingSetup';
import { TrainingSetupPanel } from './TrainingSetupPanel';

const SETUP: TrainingSetup = {
  mode: 'depth',
  depth: 2,
  theme: null,
  difficulty: 'normal',
  visualizationSeconds: 5,
};

const THEMES = [{ key: 'fork', name: 'Forchetta', count: 1234 }];

function renderPanel(setup: TrainingSetup, dueReviews = 0) {
  const onChange = vi.fn();
  const onStart = vi.fn();
  render(
    <TrainingSetupPanel
      setup={setup}
      themes={THEMES}
      dueReviews={dueReviews}
      onChange={onChange}
      onStart={onStart}
    />,
  );
  return { onChange, onStart };
}

describe('TrainingSetupPanel', () => {
  it('cambia profondità, tema e difficoltà', () => {
    const { onChange } = renderPanel(SETUP);

    fireEvent.click(screen.getByRole('radio', { name: '4' }));
    expect(onChange).toHaveBeenLastCalledWith({ ...SETUP, depth: 4 });

    fireEvent.change(screen.getByRole('combobox', { name: 'Tema' }), {
      target: { value: 'fork' },
    });
    expect(onChange).toHaveBeenLastCalledWith({ ...SETUP, theme: 'fork' });

    fireEvent.click(screen.getByRole('radio', { name: 'Più difficile' }));
    expect(onChange).toHaveBeenLastCalledWith({ ...SETUP, difficulty: 'harder' });
  });

  it('mostra i secondi solo nella visualizzazione', () => {
    renderPanel({ ...SETUP, mode: 'visualization' });

    expect(screen.getByRole('spinbutton', { name: /Secondi/ })).toBeDefined();
  });

  it('in "cosa minaccia?" non chiede quante mosse trovare', () => {
    renderPanel({ ...SETUP, mode: 'threat' });

    expect(screen.queryByRole('radio', { name: '4' })).toBeNull();
  });

  it('senza puzzle da ripassare il ripasso non parte', () => {
    const { onStart } = renderPanel({ ...SETUP, mode: 'review' }, 0);

    const button = screen.getByRole('button', { name: 'Ripassa' });
    expect(button.hasAttribute('disabled')).toBe(true);
    fireEvent.click(button);
    expect(onStart).not.toHaveBeenCalled();
    expect(screen.getByText(/Oggi non ci sono puzzle da ripassare/)).toBeDefined();
  });

  it('avvia un nuovo puzzle', () => {
    const { onStart } = renderPanel(SETUP);

    fireEvent.click(screen.getByRole('button', { name: 'Nuovo puzzle' }));

    expect(onStart).toHaveBeenCalledOnce();
  });
});
