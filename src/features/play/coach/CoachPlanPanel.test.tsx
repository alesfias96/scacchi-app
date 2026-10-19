/**
 * Test di «Il piano» della modalità Tutor.
 */
import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { STRATEGY } from '../../../test/tutorExplanations';
import { CoachPlanPanel } from './CoachPlanPanel';

/** La stessa strategia con un altro piano. */
const OTHER_PLAN = {
  ...STRATEGY,
  plans: STRATEGY.plans.map((plan) => ({
    ...plan,
    key: 'middlegame/space-advantage',
    title: 'Lo spazio',
  })),
};

describe('CoachPlanPanel', () => {
  it('mostra il piano e quello dell’avversario', () => {
    render(<CoachPlanPanel strategy={STRATEGY} confirmedPlans={[]} />);

    expect(screen.getByRole('region', { name: 'Il piano' })).toBeDefined();
    expect(screen.getByText("L'attacco di minoranza")).toBeDefined();
    expect(screen.getByText('Il piano dell’avversario')).toBeDefined();
    expect(screen.queryByText('Piano nuovo')).toBeNull();
  });

  it('segnala il piano nuovo solo quando cambia', () => {
    const { rerender } = render(<CoachPlanPanel strategy={STRATEGY} confirmedPlans={[]} />);

    rerender(<CoachPlanPanel strategy={{ ...STRATEGY }} confirmedPlans={[]} />);
    expect(screen.queryByText('Piano nuovo')).toBeNull();

    rerender(<CoachPlanPanel strategy={OTHER_PLAN} confirmedPlans={[]} />);
    expect(screen.getByText('Piano nuovo')).toBeDefined();
    expect(screen.getByText('Lo spazio')).toBeDefined();

    // Mentre il tutor prepara la posizione nuova resta il piano di prima.
    rerender(<CoachPlanPanel strategy={null} confirmedPlans={[]} />);
    expect(screen.getByText('Lo spazio')).toBeDefined();
  });

  it('non mostra niente finché non c’è un piano', () => {
    const { container } = render(<CoachPlanPanel strategy={null} confirmedPlans={[]} />);

    expect(container.childElementCount).toBe(0);
  });
});
