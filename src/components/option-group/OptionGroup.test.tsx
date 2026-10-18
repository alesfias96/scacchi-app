/**
 * Test di OptionGroup.
 */
import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { OptionGroup, type Option } from './OptionGroup';

const OPTIONS: readonly Option<'a' | 'b'>[] = [
  { value: 'a', label: 'Prima', description: 'La prima scelta', preview: null },
  { value: 'b', label: 'Seconda', preview: null },
];

describe('OptionGroup', () => {
  it('segna come selezionata la scelta attuale', () => {
    render(
      <OptionGroup legend="Prova" name="prova" options={OPTIONS} value="b" onChange={vi.fn()} />,
    );

    const second = screen.getByRole<HTMLInputElement>('radio', { name: /Seconda/ });

    expect(second.checked).toBe(true);
  });

  it('comunica la nuova scelta quando l’utente clicca', () => {
    const onChange = vi.fn();
    render(
      <OptionGroup legend="Prova" name="prova" options={OPTIONS} value="b" onChange={onChange} />,
    );

    fireEvent.click(screen.getByRole('radio', { name: /Prima/ }));

    expect(onChange).toHaveBeenCalledWith('a');
  });

  it('mostra il titolo del gruppo', () => {
    render(
      <OptionGroup legend="Prova" name="prova" options={OPTIONS} value="a" onChange={vi.fn()} />,
    );

    expect(screen.getByRole('group', { name: 'Prova' })).toBeDefined();
  });
});
