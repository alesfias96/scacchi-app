/**
 * Test del gruppo di scelte a pastiglie.
 */
import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { ChoiceGroup } from './ChoiceGroup';

const OPTIONS = [
  { value: 'white', label: 'Bianco' },
  { value: 'black', label: 'Nero', detail: 'muove per secondo' },
] as const;

describe('ChoiceGroup', () => {
  it('mostra la scelta attuale e comunica quella nuova', () => {
    const onChange = vi.fn();
    render(
      <ChoiceGroup
        legend="Colore"
        name="color"
        options={OPTIONS}
        value="white"
        onChange={onChange}
      />,
    );

    expect(screen.getByRole('group', { name: 'Colore' })).toBeDefined();
    expect(screen.getByRole<HTMLInputElement>('radio', { name: 'Bianco' }).checked).toBe(true);

    fireEvent.click(screen.getByRole('radio', { name: /Nero/ }));

    expect(onChange).toHaveBeenCalledWith('black');
  });

  it('scrive il dettaglio sotto il nome', () => {
    render(
      <ChoiceGroup
        legend="Colore"
        name="color"
        options={OPTIONS}
        value="white"
        onChange={vi.fn()}
      />,
    );

    expect(screen.getByText('muove per secondo')).toBeDefined();
  });
});
