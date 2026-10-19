/**
 * Test della sezione «La strategia» del tutor.
 */
import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { STRATEGY } from '../../../test/tutorExplanations';
import { StrategySection } from './StrategySection';

describe('StrategySection', () => {
  it('mostra il centro, chi ha cosa e i piani dei due lati', () => {
    render(<StrategySection strategy={STRATEGY} confirmedPlans={[]} />);

    expect(screen.getByText(/Centro mobile/)).toBeDefined();
    expect(screen.getByText('più spazio (9 case contro 4)')).toBeDefined();
    expect(screen.getByText('la coppia degli alfieri')).toBeDefined();
    expect(screen.getByText('Il piano consigliato')).toBeDefined();
    expect(screen.getByRole('article', { name: "L'attacco di minoranza" })).toBeDefined();
    expect(screen.getByText('Per il Bianco')).toBeDefined();
    expect(screen.getByText('Il piano dell’avversario')).toBeDefined();
    expect(screen.getByText('Quando no')).toBeDefined();
    expect(screen.getByText('Il tuo re è sotto attacco.')).toBeDefined();
    expect(screen.queryByText('✓ Stockfish conferma')).toBeNull();
  });

  it('dice quando Stockfish conferma il piano', () => {
    render(<StrategySection strategy={STRATEGY} confirmedPlans={['middlegame/minority-attack']} />);

    expect(screen.getByText('✓ Stockfish conferma')).toBeDefined();
  });

  it('senza squilibri dice che la posizione è in equilibrio', () => {
    render(
      <StrategySection
        strategy={{ ...STRATEGY, imbalances: [], opponentPlan: null }}
        confirmedPlans={[]}
      />,
    );

    expect(screen.getByText(/la posizione è in equilibrio/)).toBeDefined();
    expect(screen.queryByText('Il piano dell’avversario')).toBeNull();
  });
});
