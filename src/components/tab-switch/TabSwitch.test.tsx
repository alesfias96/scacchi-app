/**
 * Test del selettore a schede.
 */
import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { TabSwitch } from './TabSwitch';

const OPTIONS = [
  { value: 'main', label: 'Partita' },
  { value: 'tutor', label: 'Tutor' },
] as const;

describe('TabSwitch', () => {
  it('indica la voce attiva e comunica quella scelta', () => {
    const onChange = vi.fn<(value: 'main' | 'tutor') => void>();
    render(<TabSwitch label="Colonna" options={OPTIONS} value="main" onChange={onChange} />);

    expect(screen.getByRole('tab', { name: 'Partita' }).getAttribute('aria-selected')).toBe('true');
    expect(screen.getByRole('tab', { name: 'Tutor' }).getAttribute('aria-selected')).toBe('false');

    fireEvent.click(screen.getByRole('tab', { name: 'Tutor' }));

    expect(onChange).toHaveBeenCalledWith('tutor');
  });
});
