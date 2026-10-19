/**
 * Test di una partita nell'elenco dell'archivio.
 */
import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import type { GameSummary } from '../../../bindings/GameSummary';
import { GameRow } from './GameRow';

const FINISHED: GameSummary = {
  id: 'partita-1',
  startedAtMs: new Date(2026, 9, 18, 16, 35).getTime(),
  opponent: 'computer',
  computerSide: 'black',
  level: 4,
  timeControl: { initialSeconds: 300, incrementSeconds: 3 },
  result: { winner: 'white', reason: 'checkmate', score: '1-0' },
  opening: { eco: 'B20', name: 'Sicilian Defense' },
  plyCount: 41,
  analyzed: false,
};

/** Disegna la riga con funzioni simulate. */
function renderRow(game: GameSummary) {
  const onOpen = vi.fn();
  const onReview = vi.fn();
  const onDelete = vi.fn();
  render(
    <ul>
      <GameRow
        game={game}
        levelName="Circolo"
        onOpen={onOpen}
        onReview={onReview}
        onDelete={onDelete}
      />
    </ul>,
  );
  return { onOpen, onReview, onDelete };
}

describe('GameRow', () => {
  it('mostra apertura, avversario, tempo, mosse ed esito', () => {
    renderRow(FINISHED);

    expect(screen.getByText('Sicilian Defense')).toBeDefined();
    expect(screen.getByText('B20')).toBeDefined();
    expect(screen.getByText('Vinta')).toBeDefined();
    const meta = screen.getByText(/Stockfish · Circolo · tu col Bianco/);
    expect(meta.textContent).toContain('5+3');
    expect(meta.textContent).toContain('21 mosse');
  });

  it('una partita finita si rivede, una in corso si continua', () => {
    const { onOpen } = renderRow(FINISHED);
    fireEvent.click(screen.getByRole('button', { name: 'Rivedi' }));
    expect(onOpen).toHaveBeenCalledWith('partita-1');
  });

  it('elimina solo dopo la conferma', () => {
    const { onDelete } = renderRow({ ...FINISHED, result: null });

    expect(screen.getByRole('button', { name: 'Continua' })).toBeDefined();
    fireEvent.click(screen.getByRole('button', { name: 'Elimina' }));
    expect(onDelete).not.toHaveBeenCalled();
    fireEvent.click(screen.getByRole('button', { name: 'Sì, elimina' }));

    expect(onDelete).toHaveBeenCalledWith('partita-1');
  });

  it('propone di analizzare la partita, o di vedere l’analisi già fatta', () => {
    const { onReview } = renderRow(FINISHED);

    fireEvent.click(screen.getByRole('button', { name: 'Analizza' }));
    expect(onReview).toHaveBeenCalledWith(FINISHED.id);
  });

  it('dopo l’analisi offre «Vedi l’analisi»', () => {
    renderRow({ ...FINISHED, analyzed: true });

    expect(screen.getByRole('button', { name: 'Vedi l’analisi' })).toBeDefined();
  });
});
