/**
 * Test della scheda "Osservazioni".
 */
import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import type { PositionReport } from '../../../bindings/PositionReport';
import { ObservationsPanel } from './ObservationsPanel';

const MATE_FACT = {
  category: 'threats' as const,
  side: 'white' as const,
  text: 'Il Bianco può dare scacco matto in una mossa con',
  squares: [],
  arrows: [{ from: 'f3', to: 'f7' }],
  moves: ['Qxf7#'],
};

const REPORT: PositionReport = {
  fen: 'qualsiasi',
  turn: 'white',
  phase: 'opening',
  material: {
    white: { pawns: 8, knights: 2, bishops: 2, rooks: 2, queens: 1, points: 39 },
    black: { pawns: 8, knights: 2, bishops: 2, rooks: 2, queens: 1, points: 39 },
    advantage: 0,
  },
  facts: [
    MATE_FACT,
    {
      category: 'pawns',
      side: 'black',
      text: 'Il pedone nero in a5 è passato: nessun pedone nemico può fermarlo',
      squares: ['a5'],
      arrows: [],
      moves: [],
    },
  ],
};

describe('ObservationsPanel', () => {
  it('spenta mostra solo l’interruttore', () => {
    const onToggle = vi.fn();
    render(
      <ObservationsPanel
        enabled={false}
        onToggle={onToggle}
        report={REPORT}
        notation="figurine"
        onHighlight={vi.fn()}
      />,
    );

    expect(screen.queryByText('Minacce')).toBeNull();
    fireEvent.click(screen.getByRole('switch', { name: 'Mostra osservazioni' }));
    expect(onToggle).toHaveBeenCalledWith(true);
  });

  it('accesa mostra i fatti divisi per gruppo, con le mosse nella notazione scelta', () => {
    render(
      <ObservationsPanel
        enabled
        onToggle={vi.fn()}
        report={REPORT}
        notation="italiana"
        onHighlight={vi.fn()}
      />,
    );

    expect(screen.getByText('Fase di gioco: apertura')).toBeDefined();
    expect(screen.getByRole('region', { name: 'Minacce' })).toBeDefined();
    expect(screen.getByRole('region', { name: 'Pedoni' })).toBeDefined();
    expect(screen.queryByRole('region', { name: 'Tattiche' })).toBeNull();
    expect(
      screen.getByText('Il Bianco può dare scacco matto in una mossa con Dxf7#'),
    ).toBeDefined();
  });

  it('passando su un fatto lo evidenzia, uscendo lo toglie', () => {
    const onHighlight = vi.fn();
    render(
      <ObservationsPanel
        enabled
        onToggle={vi.fn()}
        report={REPORT}
        notation="figurine"
        onHighlight={onHighlight}
      />,
    );
    const item = screen.getByRole('button', { name: /scacco matto/ });

    fireEvent.mouseEnter(item);
    fireEvent.mouseLeave(item);

    expect(onHighlight).toHaveBeenNthCalledWith(1, MATE_FACT);
    expect(onHighlight).toHaveBeenNthCalledWith(2, null);
  });

  it('aspettando il rapporto lo dice', () => {
    render(
      <ObservationsPanel
        enabled
        onToggle={vi.fn()}
        report={null}
        notation="figurine"
        onHighlight={vi.fn()}
      />,
    );

    expect(screen.getByText('Guardo la posizione…')).toBeDefined();
  });
});
