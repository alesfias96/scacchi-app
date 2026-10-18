/**
 * Test del pannello della posizione.
 */
import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import type { PositionView } from '../../../bindings/PositionView';
import { PositionPanel } from './PositionPanel';

/** Posizione dopo 1.e4: muove il nero, nessuno scacco. */
const AFTER_E4: PositionView = {
  fen: 'rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1',
  boardFen: 'rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR',
  turn: 'black',
  inCheck: false,
  legalMoves: [
    { from: 'e7', to: 'e5', uci: 'e7e5' },
    { from: 'c7', to: 'c5', uci: 'c7c5' },
  ],
};

describe('PositionPanel', () => {
  it('dice chi deve muovere', () => {
    render(<PositionPanel position={AFTER_E4} />);

    expect(screen.getByText('Tocca al Nero')).toBeDefined();
  });

  it('mostra il numero di mosse possibili e lo scacco', () => {
    render(<PositionPanel position={AFTER_E4} />);

    expect(screen.getByText('2')).toBeDefined();
    expect(screen.getByText('No')).toBeDefined();
  });

  it('non mostra più la FEN', () => {
    render(<PositionPanel position={AFTER_E4} />);

    expect(screen.queryByText(AFTER_E4.fen)).toBeNull();
  });
});
