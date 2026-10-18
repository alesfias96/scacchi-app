/**
 * Test del pulsante.
 */
import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { Button } from './Button';

describe('Button', () => {
  it('è un pulsante che non invia moduli e risponde al clic', () => {
    const onClick = vi.fn();
    render(<Button onClick={onClick}>Nuova partita</Button>);

    const button = screen.getByRole('button', { name: 'Nuova partita' });
    fireEvent.click(button);

    expect(button.getAttribute('type')).toBe('button');
    expect(onClick).toHaveBeenCalled();
  });

  it('con sola icona usa il nome anche come suggerimento', () => {
    render(<Button variant="icon" aria-label="Gira scacchiera" />);

    expect(screen.getByRole('button', { name: 'Gira scacchiera' }).title).toBe('Gira scacchiera');
  });
});
