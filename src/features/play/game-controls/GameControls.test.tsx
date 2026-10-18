/**
 * Test dei comandi della partita.
 */
import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { GameControls, type GameControlsProps } from './GameControls';

/** Disegna i comandi con funzioni simulate, modificabili da ogni test. */
function renderControls(overrides: Partial<GameControlsProps> = {}) {
  const props: GameControlsProps = {
    cursor: 2,
    totalMoves: 4,
    onGoTo: vi.fn(),
    onNewGame: vi.fn(),
    onUndo: vi.fn(),
    onFlip: vi.fn(),
    ...overrides,
  };
  render(<GameControls {...props} />);
  return props;
}

describe('GameControls', () => {
  it('scorre la partita avanti, indietro, all’inizio e alla fine', () => {
    const { onGoTo } = renderControls();

    fireEvent.click(screen.getByRole('button', { name: /Mossa precedente/ }));
    fireEvent.click(screen.getByRole('button', { name: /Mossa successiva/ }));
    fireEvent.click(screen.getByRole('button', { name: /Posizione iniziale/ }));
    fireEvent.click(screen.getByRole('button', { name: /Ultima mossa/ }));

    expect(onGoTo).toHaveBeenNthCalledWith(1, 1);
    expect(onGoTo).toHaveBeenNthCalledWith(2, 3);
    expect(onGoTo).toHaveBeenNthCalledWith(3, 0);
    expect(onGoTo).toHaveBeenNthCalledWith(4, 4);
  });

  it('disattiva i comandi che non avrebbero effetto', () => {
    renderControls({ cursor: 0, totalMoves: 0 });

    expect(
      screen.getByRole<HTMLButtonElement>('button', { name: /Posizione iniziale/ }).disabled,
    ).toBe(true);
    expect(screen.getByRole<HTMLButtonElement>('button', { name: /Ultima mossa/ }).disabled).toBe(
      true,
    );
    expect(screen.getByRole<HTMLButtonElement>('button', { name: /Annulla/ }).disabled).toBe(true);
  });

  it('nuova partita, annulla e gira scacchiera chiamano le rispettive azioni', () => {
    const { onNewGame, onUndo, onFlip } = renderControls();

    fireEvent.click(screen.getByRole('button', { name: /Nuova partita/ }));
    fireEvent.click(screen.getByRole('button', { name: /Annulla/ }));
    fireEvent.click(screen.getByRole('button', { name: /Gira/ }));

    expect(onNewGame).toHaveBeenCalled();
    expect(onUndo).toHaveBeenCalled();
    expect(onFlip).toHaveBeenCalled();
  });
});
