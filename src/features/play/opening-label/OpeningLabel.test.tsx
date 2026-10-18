/**
 * Test del nome dell'apertura.
 */
import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { OpeningLabel } from './OpeningLabel';

describe('OpeningLabel', () => {
  it('mostra codice e nome dell’apertura', () => {
    render(<OpeningLabel opening={{ eco: 'B90', name: 'Sicilian Defense: Najdorf Variation' }} />);

    expect(screen.getByText('B90')).toBeDefined();
    expect(screen.getByText('Sicilian Defense: Najdorf Variation')).toBeDefined();
  });

  it('senza apertura non mostra nulla', () => {
    const { container } = render(<OpeningLabel opening={null} />);

    expect(container.textContent).toBe('');
  });
});
