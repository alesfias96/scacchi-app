/**
 * Test dell'annuncio di fine partita.
 */
import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { GameResultBanner } from './GameResultBanner';

describe('GameResultBanner', () => {
  it('annuncia lo scacco matto con vincitore e punteggio', () => {
    render(<GameResultBanner result={{ winner: 'white', reason: 'checkmate', score: '1-0' }} />);

    const banner = screen.getByRole('status');
    expect(banner.textContent).toContain('Scacco matto');
    expect(banner.textContent).toContain('Vince il Bianco');
    expect(banner.textContent).toContain('1-0');
  });

  it('scrive la patta con il mezzo punto', () => {
    render(<GameResultBanner result={{ winner: null, reason: 'stalemate', score: '1/2-1/2' }} />);

    expect(screen.getByText('½-½')).toBeDefined();
  });
});
