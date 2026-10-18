/**
 * Test della barra di valutazione.
 */
import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { EvalBar } from './EvalBar';

describe('EvalBar', () => {
  it('resta a metà mentre aspetta il primo giudizio', () => {
    render(<EvalBar evaluation={null} orientation="white" />);

    const meter = screen.getByRole('meter', { name: 'Valutazione della posizione' });
    expect(meter.getAttribute('aria-valuenow')).toBe('50');
  });

  it('cresce dalla parte del Bianco e scrive il valore', () => {
    render(<EvalBar evaluation={{ kind: 'centipawns', value: 250 }} orientation="white" />);

    const meter = screen.getByRole('meter');
    expect(Number(meter.getAttribute('aria-valuenow'))).toBeGreaterThan(50);
    expect(meter.getAttribute('aria-valuetext')).toBe('+2.5');
  });

  it('con la scacchiera girata la parte bianca parte dall’alto', () => {
    const { container } = render(
      <EvalBar evaluation={{ kind: 'centipawns', value: 0 }} orientation="black" />,
    );

    const white = container.querySelectorAll('rect')[1];
    expect(white?.getAttribute('y')).toBe('0');
  });
});
