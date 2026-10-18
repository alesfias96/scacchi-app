/**
 * Test dell'elenco delle mosse.
 */
import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import type { MoveRecord } from '../../../bindings/MoveRecord';
import { MoveList } from './MoveList';

/** 1. e4 e5 2. Nf3 */
const MOVES: MoveRecord[] = [
  { ply: 1, moveNumber: 1, side: 'white', san: 'e4', uci: 'e2e4', from: 'e2', to: 'e4' },
  { ply: 2, moveNumber: 1, side: 'black', san: 'e5', uci: 'e7e5', from: 'e7', to: 'e5' },
  { ply: 3, moveNumber: 2, side: 'white', san: 'Nf3', uci: 'g1f3', from: 'g1', to: 'f3' },
];

describe('MoveList', () => {
  it('invita a muovere se non ci sono mosse', () => {
    render(<MoveList moves={[]} cursor={0} notation="figurine" onSelect={vi.fn()} />);

    expect(screen.getByText(/muovi un pezzo per cominciare/)).toBeDefined();
  });

  it('scrive le mosse nella notazione scelta', () => {
    const { rerender } = render(
      <MoveList moves={MOVES} cursor={3} notation="italiana" onSelect={vi.fn()} />,
    );
    expect(screen.getByRole('button', { name: 'Cf3' })).toBeDefined();

    rerender(<MoveList moves={MOVES} cursor={3} notation="figurine" onSelect={vi.fn()} />);
    expect(screen.getByRole('button', { name: '♞f3' })).toBeDefined();
  });

  it('evidenzia la mossa della posizione mostrata', () => {
    render(<MoveList moves={MOVES} cursor={2} notation="internazionale" onSelect={vi.fn()} />);

    expect(screen.getByRole('button', { name: 'e5' }).getAttribute('aria-current')).toBe('step');
    expect(screen.getByRole('button', { name: 'e4' }).getAttribute('aria-current')).toBeNull();
  });

  it('un clic su una mossa chiede di mostrare quella posizione', () => {
    const onSelect = vi.fn();
    render(<MoveList moves={MOVES} cursor={3} notation="internazionale" onSelect={onSelect} />);

    fireEvent.click(screen.getByRole('button', { name: 'e4' }));

    expect(onSelect).toHaveBeenCalledWith(1);
  });
});
