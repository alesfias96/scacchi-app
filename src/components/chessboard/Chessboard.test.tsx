/**
 * Test del componente Chessboard.
 *
 * chessground viene simulato: qui si verifica che il nostro involucro
 * gli passi la configurazione giusta, lo aggiorni, lo liberi correttamente
 * applichi lo stile di scacchiera e pezzi scelto nelle impostazioni e,
 * quando è interattiva, permetta solo le mosse legali e le comunichi.
 */
import type { Config } from '@lichess-org/chessground/config';
import { render } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { useSettingsStore } from '../../state/settingsStore';
import { Chessboard } from './Chessboard';

/** Funzioni simulate di chessground, condivise tra la simulazione e i test. */
const chessgroundMock = vi.hoisted(() => {
  const set = vi.fn();
  const destroy = vi.fn();
  // Il tipo tra < > dice quali argomenti riceve la funzione vera di chessground,
  // così i test possono leggere la configurazione passata (mock.calls).
  const create = vi.fn<
    (element: HTMLElement, config: Config) => { set: typeof set; destroy: typeof destroy }
  >(() => ({ set, destroy }));
  return { create, set, destroy };
});

vi.mock('@lichess-org/chessground', () => ({
  Chessground: chessgroundMock.create,
}));

const STARTING_BOARD = 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR';
const AFTER_E4_BOARD = 'rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR';

describe('Chessboard', () => {
  beforeEach(() => {
    chessgroundMock.create.mockClear();
    chessgroundMock.set.mockClear();
    chessgroundMock.destroy.mockClear();
    useSettingsStore.setState({
      settings: {
        theme: 'studio',
        board: 'rovere',
        pieces: 'icona',
        notation: 'figurine',
        sounds: 'on',
      },
    });
  });

  it('crea la scacchiera in sola lettura con la posizione ricevuta', () => {
    render(<Chessboard boardFen={STARTING_BOARD} turn="white" inCheck={false} />);

    expect(chessgroundMock.create).toHaveBeenCalledWith(
      expect.any(HTMLDivElement),
      expect.objectContaining({
        fen: STARTING_BOARD,
        turnColor: 'white',
        check: false,
        orientation: 'white',
        viewOnly: true,
      }),
    );
  });

  it('aggiorna la scacchiera esistente quando cambia la posizione', () => {
    const { rerender } = render(
      <Chessboard boardFen={STARTING_BOARD} turn="white" inCheck={false} />,
    );

    rerender(<Chessboard boardFen={AFTER_E4_BOARD} turn="black" inCheck={false} />);

    expect(chessgroundMock.set).toHaveBeenLastCalledWith(
      expect.objectContaining({ fen: AFTER_E4_BOARD, turnColor: 'black' }),
    );
  });

  it('libera chessground quando viene rimossa', () => {
    const { unmount } = render(
      <Chessboard boardFen={STARTING_BOARD} turn="white" inCheck={false} />,
    );

    unmount();

    expect(chessgroundMock.destroy).toHaveBeenCalled();
  });

  it('applica lo stile di scacchiera e pezzi scelto nelle impostazioni', () => {
    useSettingsStore.setState({
      settings: {
        theme: 'studio',
        board: 'marmo',
        pieces: 'gettoni',
        notation: 'figurine',
        sounds: 'on',
      },
    });

    const { container } = render(
      <Chessboard boardFen={STARTING_BOARD} turn="white" inCheck={false} />,
    );

    const frame = container.querySelector('[data-board]');
    expect(frame?.getAttribute('data-board')).toBe('marmo');
    expect(frame?.getAttribute('data-pieces')).toBe('gettoni');
  });

  it('quando è interattiva lascia muovere solo chi è di turno, verso case legali', () => {
    render(
      <Chessboard
        boardFen={STARTING_BOARD}
        turn="white"
        inCheck={false}
        legalMoves={[{ from: 'e2', to: 'e4', uci: 'e2e4' }]}
        onMove={vi.fn()}
      />,
    );

    const config = chessgroundMock.create.mock.calls[0]?.[1];
    expect(config?.viewOnly).toBe(false);
    expect(config?.movable?.color).toBe('white');
    expect(config?.movable?.dests?.get('e2')).toEqual(['e4']);
  });

  it('comunica la mossa fatta sulla scacchiera', () => {
    const onMove = vi.fn();
    render(
      <Chessboard
        boardFen={STARTING_BOARD}
        turn="white"
        inCheck={false}
        legalMoves={[{ from: 'e2', to: 'e4', uci: 'e2e4' }]}
        onMove={onMove}
      />,
    );

    const after = chessgroundMock.create.mock.calls[0]?.[1].movable?.events?.after;
    after?.('e2', 'e4', { premove: false });

    expect(onMove).toHaveBeenCalledWith('e2', 'e4');
  });

  it("evidenzia l'ultima mossa", () => {
    render(
      <Chessboard
        boardFen={AFTER_E4_BOARD}
        turn="black"
        inCheck={false}
        lastMove={{ from: 'e2', to: 'e4', uci: 'e2e4' }}
      />,
    );

    expect(chessgroundMock.create.mock.calls[0]?.[1].lastMove).toEqual(['e2', 'e4']);
  });

  it('riapplica la posizione quando cambia resyncKey', () => {
    const { rerender } = render(
      <Chessboard boardFen={STARTING_BOARD} turn="white" inCheck={false} resyncKey={0} />,
    );
    chessgroundMock.set.mockClear();

    rerender(<Chessboard boardFen={STARTING_BOARD} turn="white" inCheck={false} resyncKey={1} />);

    expect(chessgroundMock.set).toHaveBeenCalledWith(
      expect.objectContaining({ fen: STARTING_BOARD }),
    );
  });

  it('indica il colore che può muovere, per il cursore a mano sui suoi pezzi', () => {
    const { container, rerender } = render(
      <Chessboard boardFen={STARTING_BOARD} turn="black" inCheck={false} onMove={vi.fn()} />,
    );
    expect(container.querySelector('[data-board]')?.getAttribute('data-movable')).toBe('black');

    rerender(<Chessboard boardFen={STARTING_BOARD} turn="black" inCheck={false} />);
    expect(container.querySelector('[data-board]')?.hasAttribute('data-movable')).toBe(false);
  });

  it('anima i pezzi, tranne se il sistema chiede di ridurre le animazioni', () => {
    render(<Chessboard boardFen={STARTING_BOARD} turn="white" inCheck={false} />);
    expect(chessgroundMock.create.mock.calls[0]?.[1].animation?.enabled).toBe(true);

    const matchMedia = vi.spyOn(window, 'matchMedia').mockReturnValue({
      matches: true,
    } as MediaQueryList);
    render(<Chessboard boardFen={STARTING_BOARD} turn="white" inCheck={false} />);
    expect(chessgroundMock.create.mock.calls[1]?.[1].animation?.enabled).toBe(false);
    matchMedia.mockRestore();
  });
});
