/**
 * Test della striscia del giocatore.
 */
import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { PlayerBar } from './PlayerBar';

describe('PlayerBar', () => {
  it('mostra nome, dettaglio e tempo', () => {
    render(
      <PlayerBar
        side="black"
        name="Stockfish"
        detail="Circolo · 1400 Elo"
        clock={{ ms: 247_000, running: false }}
        thinking={false}
      />,
    );

    expect(screen.getByText('Stockfish')).toBeDefined();
    expect(screen.getByText('Circolo · 1400 Elo')).toBeDefined();
    expect(screen.getByLabelText('Tempo del Nero').textContent).toBe('4:07');
  });

  it('avvisa quando il computer sta pensando', () => {
    render(<PlayerBar side="white" name="Stockfish" clock={null} thinking />);

    expect(screen.getByRole('status').textContent).toContain('sta pensando');
  });

  it('senza orologio non mostra il tempo', () => {
    render(<PlayerBar side="white" name="Tu" clock={null} thinking={false} />);

    expect(screen.queryByLabelText('Tempo del Bianco')).toBeNull();
  });
});
