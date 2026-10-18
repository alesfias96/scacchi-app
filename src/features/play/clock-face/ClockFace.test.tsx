/**
 * Test del quadrante dell'orologio.
 */
import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { ClockFace } from './ClockFace';

describe('ClockFace', () => {
  it('scrive il tempo rimasto', () => {
    render(<ClockFace reading={{ ms: 247_000, running: false }} side="white" />);

    expect(screen.getByLabelText('Tempo del Bianco').textContent).toBe('4:07');
  });

  it('distingue l’orologio in moto e quello quasi finito', () => {
    const { rerender } = render(<ClockFace reading={{ ms: 60_000, running: true }} side="black" />);
    const runningClass = screen.getByLabelText('Tempo del Nero').className;

    rerender(<ClockFace reading={{ ms: 5_000, running: true }} side="black" />);

    expect(screen.getByLabelText('Tempo del Nero').className).not.toBe(runningClass);
  });
});
