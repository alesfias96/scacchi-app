/**
 * Test del grafico a linea.
 */
import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { LineChart } from './LineChart';

describe('LineChart', () => {
  it('disegna la linea con i valori più alto e più basso', () => {
    const { container } = render(
      <LineChart
        label="Rating dei puzzle"
        points={[
          { timeMs: 0, value: 1500 },
          { timeMs: 1, value: 1480 },
          { timeMs: 2, value: 1530 },
        ]}
      />,
    );

    expect(screen.getByRole('img', { name: 'Rating dei puzzle: da 1500 a 1530' })).toBeDefined();
    expect(container.querySelector('polyline')?.getAttribute('points')?.split(' ')).toHaveLength(3);
    expect(screen.getByText('1530')).toBeDefined();
    expect(screen.getByText('1480')).toBeDefined();
  });

  it('con un solo punto mostra un messaggio', () => {
    render(<LineChart label="Precisione" points={[{ timeMs: 0, value: 80 }]} />);

    expect(screen.getByText(/Servono almeno due risultati/)).toBeDefined();
  });
});
