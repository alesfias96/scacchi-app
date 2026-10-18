/**
 * Test della scheda "Analisi".
 */
import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import type { AnalysisUpdate } from '../../../bindings/AnalysisUpdate';
import { AnalysisPanel } from './AnalysisPanel';

const UPDATE: AnalysisUpdate = {
  depth: 18,
  evaluation: { kind: 'centipawns', value: 34 },
  line: { fen: '', turn: 'black', moveNumber: 1, san: ['c5', 'Nf3'], firstMove: null },
};

describe('AnalysisPanel', () => {
  it('spenta mostra solo l’interruttore e lo comunica quando si accende', () => {
    const onToggle = vi.fn();
    render(
      <AnalysisPanel
        enabled={false}
        onToggle={onToggle}
        update={UPDATE}
        isGameOver={false}
        notation="figurine"
      />,
    );

    expect(screen.queryByText('profondità 18')).toBeNull();
    fireEvent.click(screen.getByRole('switch', { name: 'Mostra analisi di Stockfish' }));

    expect(onToggle).toHaveBeenCalledWith(true);
  });

  it('accesa mostra valutazione, profondità e variante', () => {
    render(
      <AnalysisPanel
        enabled
        onToggle={vi.fn()}
        update={UPDATE}
        isGameOver={false}
        notation="italiana"
      />,
    );

    expect(screen.getByText('+0.3')).toBeDefined();
    expect(screen.getByText('profondità 18')).toBeDefined();
    expect(screen.getByText('1… c5 2. Cf3')).toBeDefined();
  });

  it('aspettando il primo giudizio lo dice', () => {
    render(
      <AnalysisPanel
        enabled
        onToggle={vi.fn()}
        update={null}
        isGameOver={false}
        notation="figurine"
      />,
    );

    expect(screen.getByText('Stockfish sta guardando la posizione…')).toBeDefined();
  });
});
