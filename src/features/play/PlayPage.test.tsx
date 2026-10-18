/**
 * Test della sezione "Gioca".
 *
 * Il servizio verso Rust e chessground sono simulati: si verifica che la
 * pagina mostri la partita ricevuta, invii a Rust le mosse fatte sulla
 * scacchiera (promozione compresa), annunci la fine della partita e
 * risponda a tastiera e comandi.
 */
import type { Config } from '@lichess-org/chessground/config';
import type { Key } from '@lichess-org/chessground/types';
import { fireEvent, render, screen, waitFor } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import type { GameView } from '../../bindings/GameView';
import { useGameStore } from '../../state/gameStore';
import { useSettingsStore } from '../../state/settingsStore';
import { PlayPage } from './PlayPage';

const serviceMock = vi.hoisted(() => ({
  fetchGame: vi.fn(),
  startNewGame: vi.fn(),
  playMove: vi.fn(),
  undoMove: vi.fn(),
  goToPly: vi.fn(),
  importGame: vi.fn(),
  exportPgn: vi.fn(),
}));

vi.mock('../../services/gameService', () => serviceMock);

/** chessground simulato: ricorda l'ultima configurazione ricevuta (vedi beforeEach). */
const chessgroundMock = vi.hoisted(() => ({
  state: { config: undefined as Config | undefined },
  create: vi.fn(),
}));

vi.mock('@lichess-org/chessground', () => ({ Chessground: chessgroundMock.create }));

/** Simula l'utente che sposta un pezzo sulla scacchiera. */
function dragPiece(from: Key, to: Key) {
  chessgroundMock.state.config?.movable?.events?.after?.(from, to, { premove: false });
}

/** Partita dopo 1.e4: muove il nero. */
const AFTER_E4: GameView = {
  position: {
    fen: 'rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1',
    boardFen: 'rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR',
    turn: 'black',
    inCheck: false,
    legalMoves: [{ from: 'e7', to: 'e5', uci: 'e7e5' }],
  },
  moves: [{ ply: 1, moveNumber: 1, side: 'white', san: 'e4', uci: 'e2e4', from: 'e2', to: 'e4' }],
  cursor: 1,
  lastMove: { from: 'e2', to: 'e4', uci: 'e2e4' },
  result: null,
  startingFen: 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1',
};

/** Un pedone bianco in e7 pronto a promuovere. */
const PROMOTION: GameView = {
  ...AFTER_E4,
  position: {
    fen: '8/4P3/8/7k/8/8/8/4K3 w - - 0 1',
    boardFen: '8/4P3/8/7k/8/8/8/4K3',
    turn: 'white',
    inCheck: false,
    legalMoves: ['q', 'r', 'b', 'n'].map((piece) => ({
      from: 'e7',
      to: 'e8',
      uci: `e7e8${piece}`,
    })),
  },
};

describe('PlayPage', () => {
  beforeEach(() => {
    vi.resetAllMocks();
    chessgroundMock.create.mockImplementation((_element: HTMLElement, config: Config) => {
      chessgroundMock.state.config = config;
      return {
        set: (next: Config) => {
          chessgroundMock.state.config = next;
        },
        destroy: vi.fn(),
      };
    });
    chessgroundMock.state.config = undefined;
    useGameStore.setState({ game: null, loadError: null, actionError: null });
    useSettingsStore.setState({
      settings: { theme: 'studio', board: 'rovere', pieces: 'icona', notation: 'italiana' },
    });
  });

  it('mostra un messaggio di attesa mentre arriva la partita', () => {
    serviceMock.fetchGame.mockReturnValue(new Promise(() => undefined));

    render(<PlayPage />);

    expect(screen.getByText('Preparo la scacchiera…')).toBeDefined();
  });

  it('mostra la partita ricevuta da Rust', async () => {
    serviceMock.fetchGame.mockResolvedValue(AFTER_E4);

    render(<PlayPage />);

    expect(await screen.findByRole('region', { name: 'Situazione' })).toBeDefined();
    expect(screen.getByRole('button', { name: 'e4' })).toBeDefined();
    expect(chessgroundMock.state.config?.lastMove).toEqual(['e2', 'e4']);
  });

  it('spiega il problema se Rust non risponde', async () => {
    serviceMock.fetchGame.mockRejectedValue('comando non trovato');

    render(<PlayPage />);

    expect((await screen.findByRole('alert')).textContent).toContain('comando non trovato');
  });

  it('invia a Rust la mossa fatta sulla scacchiera', async () => {
    serviceMock.fetchGame.mockResolvedValue(AFTER_E4);
    serviceMock.playMove.mockResolvedValue(AFTER_E4);
    render(<PlayPage />);
    await screen.findByRole('region', { name: 'Situazione' });

    dragPiece('e7', 'e5');

    expect(serviceMock.playMove).toHaveBeenCalledWith('e7e5');
  });

  it('per la promozione chiede il pezzo e invia la mossa completa', async () => {
    serviceMock.fetchGame.mockResolvedValue(PROMOTION);
    serviceMock.playMove.mockResolvedValue(PROMOTION);
    render(<PlayPage />);
    await screen.findByRole('region', { name: 'Situazione' });

    dragPiece('e7', 'e8');
    fireEvent.click(await screen.findByRole('button', { name: 'Cavallo' }));

    expect(serviceMock.playMove).toHaveBeenCalledWith('e7e8n');
    expect(screen.queryByRole('dialog')).toBeNull();
  });

  it('annuncia la fine della partita e blocca la scacchiera', async () => {
    serviceMock.fetchGame.mockResolvedValue({
      ...AFTER_E4,
      result: { winner: 'black', reason: 'checkmate', score: '0-1' },
    });

    render(<PlayPage />);

    expect(await screen.findByText('Scacco matto')).toBeDefined();
    expect(chessgroundMock.state.config?.movable?.dests?.size).toBe(0);
  });

  it('la freccia sinistra torna indietro di una mossa', async () => {
    serviceMock.fetchGame.mockResolvedValue(AFTER_E4);
    serviceMock.goToPly.mockResolvedValue(AFTER_E4);
    render(<PlayPage />);
    await screen.findByRole('region', { name: 'Situazione' });

    fireEvent.keyDown(window, { key: 'ArrowLeft' });

    expect(serviceMock.goToPly).toHaveBeenCalledWith(0);
  });

  it('"Gira la scacchiera" mostra il nero in basso', async () => {
    serviceMock.fetchGame.mockResolvedValue(AFTER_E4);
    render(<PlayPage />);

    fireEvent.click(await screen.findByRole('button', { name: 'Gira la scacchiera' }));

    await waitFor(() => {
      expect(chessgroundMock.state.config?.orientation).toBe('black');
    });
  });
});
