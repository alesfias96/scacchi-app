/**
 * Test dell'interruttore.
 */
import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { Switch } from './Switch';

describe('Switch', () => {
  it('mostra lo stato e comunica il cambiamento', () => {
    const onChange = vi.fn();
    render(<Switch label="Mostra analisi" checked={false} onChange={onChange} />);

    const control = screen.getByRole<HTMLInputElement>('switch', { name: 'Mostra analisi' });
    expect(control.checked).toBe(false);

    fireEvent.click(control);

    expect(onChange).toHaveBeenCalledWith(true);
  });
});
