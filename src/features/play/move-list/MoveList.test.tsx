/**
 * Test della lista delle mosse, con le varianti.
 */
import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { sampleTree } from '../../../test/moveTrees';
import { MoveList } from './MoveList';

/** Disegna la lista con funzioni simulate. */
function renderList(current: number | null, canPromote = true) {
  const onSelect = vi.fn<(node: number) => void>();
  const onPromote = vi.fn<(node: number) => void>();
  const onDelete = vi.fn<(node: number) => void>();
  render(
    <MoveList
      tree={sampleTree(current)}
      notation="italiana"
      canPromote={canPromote}
      onSelect={onSelect}
      onPromote={onPromote}
      onDelete={onDelete}
    />,
  );
  return { onSelect, onPromote, onDelete };
}

describe('MoveList', () => {
  it('senza mosse invita a cominciare', () => {
    render(
      <MoveList
        tree={{ nodes: [], roots: [], current: null }}
        notation="figurine"
        canPromote={false}
        onSelect={vi.fn()}
        onPromote={vi.fn()}
        onDelete={vi.fn()}
      />,
    );

    expect(screen.getByText(/Nessuna mossa/)).toBeDefined();
  });

  it('mostra la linea principale e le varianti, nella notazione scelta', () => {
    renderList(2);

    expect(screen.getByRole('button', { name: 'e5' })).toBeDefined();
    expect(screen.getByRole('button', { name: '1… c5' })).toBeDefined();
    expect(screen.getByRole('button', { name: '2. c3' })).toBeDefined();
    expect(screen.getAllByRole('button', { name: /Cf3/ })).toHaveLength(2);
  });

  it('un clic su una mossa della variante la mostra', () => {
    const { onSelect } = renderList(2);

    fireEvent.click(screen.getByRole('button', { name: '1… c5' }));

    expect(onSelect).toHaveBeenCalledWith(3);
  });

  it('in una variante si può promuovere o eliminare da lì', () => {
    const { onPromote, onDelete } = renderList(3);

    fireEvent.click(screen.getByRole('button', { name: 'Promuovi a linea principale' }));
    fireEvent.click(screen.getByRole('button', { name: 'Elimina da qui' }));

    expect(onPromote).toHaveBeenCalledWith(3);
    expect(onDelete).toHaveBeenCalledWith(3);
  });

  it('sulla linea principale i comandi delle varianti non ci sono', () => {
    renderList(1);

    expect(screen.queryByRole('button', { name: 'Elimina da qui' })).toBeNull();
  });

  it('se la partita non lo permette, la variante non si promuove', () => {
    renderList(3, false);

    expect(screen.queryByRole('button', { name: 'Promuovi a linea principale' })).toBeNull();
    expect(screen.getByRole('button', { name: 'Elimina da qui' })).toBeDefined();
  });
});
