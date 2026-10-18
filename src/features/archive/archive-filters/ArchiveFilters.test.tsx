/**
 * Test dei filtri dell'archivio.
 */
import { fireEvent, render, screen, within } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import type { GameFilter } from '../../../bindings/GameFilter';
import { ArchiveFilters } from './ArchiveFilters';

const NO_FILTER: GameFilter = { opponent: null, outcome: null, text: null };

describe('ArchiveFilters', () => {
  it('senza filtri seleziona "Tutte"', () => {
    render(<ArchiveFilters filter={NO_FILTER} onChange={vi.fn()} />);

    const groups = screen.getAllByRole<HTMLInputElement>('radio', { name: 'Tutte' });
    expect(groups.every((radio) => radio.checked)).toBe(true);
  });

  it('comunica avversario, esito e testo scelti', () => {
    const onChange = vi.fn();
    render(<ArchiveFilters filter={NO_FILTER} onChange={onChange} />);

    fireEvent.click(screen.getByRole('radio', { name: 'Computer' }));
    fireEvent.click(screen.getByRole('radio', { name: 'Perse' }));
    fireEvent.change(screen.getByRole('searchbox'), { target: { value: 'Sicilian' } });

    expect(onChange).toHaveBeenNthCalledWith(1, { ...NO_FILTER, opponent: 'computer' });
    expect(onChange).toHaveBeenNthCalledWith(2, { ...NO_FILTER, outcome: 'lost' });
    expect(onChange).toHaveBeenNthCalledWith(3, { ...NO_FILTER, text: 'Sicilian' });
  });

  it('"Tutte" toglie il filtro', () => {
    const onChange = vi.fn();
    render(<ArchiveFilters filter={{ ...NO_FILTER, opponent: 'computer' }} onChange={onChange} />);

    const opponentGroup = screen.getByRole('group', { name: 'Avversario' });
    fireEvent.click(within(opponentGroup).getByRole('radio', { name: 'Tutte' }));

    expect(onChange).toHaveBeenCalledWith(NO_FILTER);
  });
});
