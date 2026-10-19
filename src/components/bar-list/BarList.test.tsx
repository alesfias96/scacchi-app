/**
 * Test dell'elenco di barre.
 */
import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { BarList } from './BarList';

describe('BarList', () => {
  it('mette i tratti uno dopo l’altro senza superare la barra', () => {
    const { container } = render(
      <BarList
        label="Risultati"
        items={[
          {
            key: 'white',
            label: 'Bianco',
            detail: '3 vinte · 1 patta',
            segments: [
              { percent: 75, tone: 'good' },
              { percent: 50, tone: 'neutral' },
            ],
          },
        ]}
      />,
    );

    expect(screen.getByRole('list', { name: 'Risultati' })).toBeDefined();
    expect(screen.getByText('3 vinte · 1 patta')).toBeDefined();
    const segments = container.querySelectorAll('rect ~ rect');
    expect(Array.from(segments, (rect) => rect.getAttribute('width'))).toEqual(['75%', '25%']);
    expect(segments[1]?.getAttribute('x')).toBe('75%');
  });
});
