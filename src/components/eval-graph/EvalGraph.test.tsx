/**
 * Test del grafico della partita.
 */
import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { EvalGraph } from './EvalGraph';

describe('EvalGraph', () => {
  it('segna gli errori e si scorre con le frecce', () => {
    const onSelect = vi.fn<(position: number) => void>();
    const { container } = render(
      <EvalGraph
        chances={[50, 55, 20]}
        marks={[{ ply: 2, severity: 'blunder' }]}
        current={1}
        onSelect={onSelect}
      />,
    );

    expect(container.querySelectorAll('circle')).toHaveLength(1);
    const graph = screen.getByRole('slider', { name: 'Grafico della partita' });
    fireEvent.keyDown(graph, { key: 'ArrowRight' });
    expect(onSelect).toHaveBeenLastCalledWith(2);
    fireEvent.keyDown(graph, { key: 'ArrowLeft' });
    expect(onSelect).toHaveBeenLastCalledWith(0);
  });
});
