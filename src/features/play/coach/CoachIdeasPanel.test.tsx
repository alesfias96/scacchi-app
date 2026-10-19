/**
 * Test dello sguardo lungo della modalità Tutor.
 */
import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import type { MoveRecord } from '../../../bindings/MoveRecord';
import { MOVE_IDEA_WITH_BETTER, POSITION_IDEAS } from '../../../test/ideas';
import { CoachIdeasPanel } from './CoachIdeasPanel';

const RECORD: MoveRecord = {
  ply: 3,
  moveNumber: 2,
  side: 'white',
  san: 'd3',
  uci: 'd2d3',
  from: 'd2',
  to: 'd3',
  capture: false,
};

describe('CoachIdeasPanel', () => {
  it('spiega la tua mossa, la migliore e le idee per continuare', () => {
    render(
      <CoachIdeasPanel
        ideas={{
          move: { record: RECORD, idea: MOVE_IDEA_WITH_BETTER },
          position: POSITION_IDEAS,
          isThinking: false,
        }}
        notation="internazionale"
      />,
    );

    expect(screen.getByRole('region', { name: 'L’idea della tua mossa' })).toBeDefined();
    expect(screen.getByText(/Stockfish preferiva un'altra mossa/)).toBeDefined();
    expect(screen.getByText('La mossa migliore: Nf3')).toBeDefined();
    expect(screen.getByRole('region', { name: 'Le idee per continuare' })).toBeDefined();
    expect(screen.getByText('Idea 1 · la preferita da Stockfish')).toBeDefined();
  });

  it('dice quando sta ancora guardando avanti', () => {
    render(
      <CoachIdeasPanel
        ideas={{ move: null, position: null, isThinking: true }}
        notation="internazionale"
      />,
    );

    expect(screen.getByText('Il tutor sta guardando avanti…')).toBeDefined();
  });
});
