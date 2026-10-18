/**
 * Test dello scorrere dell'orologio sullo schermo.
 */
import { act, render, screen } from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { useCountdown } from './useCountdown';

/** Un componente di prova che scrive il tempo rimasto. */
function CountdownProbe({ ms, running }: { ms: number; running: boolean }) {
  return <p>{Math.round(useCountdown(ms, running))}</p>;
}

describe('useCountdown', () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });
  afterEach(() => {
    vi.useRealTimers();
  });

  it('fa scorrere il tempo dell’orologio in moto', () => {
    render(<CountdownProbe ms={5000} running />);

    act(() => {
      vi.advanceTimersByTime(1000);
    });

    expect(Number(screen.getByRole('paragraph').textContent)).toBeLessThanOrEqual(4000);
  });

  it('lascia fermo l’orologio fermo', () => {
    render(<CountdownProbe ms={5000} running={false} />);

    act(() => {
      vi.advanceTimersByTime(1000);
    });

    expect(screen.getByRole('paragraph').textContent).toBe('5000');
  });
});
