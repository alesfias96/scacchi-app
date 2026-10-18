/**
 * Test della scheda con titolo.
 */
import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { PanelCard } from './PanelCard';

describe('PanelCard', () => {
  it('mostra titolo e contenuto, collegati tra loro', () => {
    render(
      <PanelCard title="Mosse">
        <p>contenuto</p>
      </PanelCard>,
    );

    const card = screen.getByRole('region', { name: 'Mosse' });
    expect(card.textContent).toContain('contenuto');
  });
});
