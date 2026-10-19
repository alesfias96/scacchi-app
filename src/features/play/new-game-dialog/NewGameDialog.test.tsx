/**
 * Test della finestra "Nuova partita". L'elenco dei livelli arriva da Rust (simulato).
 */
import { fireEvent, render, screen } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import type { GameSetup } from '../../../bindings/GameSetup';
import { NewGameDialog } from './NewGameDialog';

const engineServiceMock = vi.hoisted(() => ({
  fetchEngineLevels: vi.fn(),
  fetchMaiaLevels: vi.fn(),
}));

vi.mock('../../../services/engineService', () => engineServiceMock);

const LAST_SETUP: GameSetup = {
  opponent: 'computer',
  color: 'white',
  engine: 'stockfish',
  level: 4,
  maiaLevel: 5,
  timeControl: { initialSeconds: 300, incrementSeconds: 3 },
};

describe('NewGameDialog', () => {
  beforeEach(() => {
    engineServiceMock.fetchEngineLevels.mockResolvedValue([
      { level: 1, name: 'Principiante', elo: 800 },
      { level: 4, name: 'Circolo', elo: 1400 },
    ]);
    engineServiceMock.fetchMaiaLevels.mockResolvedValue([
      { level: 1, name: 'Maia 1100', elo: 1100 },
      { level: 5, name: 'Maia 1500', elo: 1500 },
    ]);
  });

  it('propone le scelte dell’ultima partita', async () => {
    render(<NewGameDialog initialSetup={LAST_SETUP} onStart={vi.fn()} onClose={vi.fn()} />);

    expect(screen.getByRole<HTMLInputElement>('radio', { name: /Computer/ }).checked).toBe(true);
    expect(screen.getByRole<HTMLInputElement>('radio', { name: /5\+3/ }).checked).toBe(true);
    const club = await screen.findByRole<HTMLInputElement>('radio', { name: /Circolo/ });
    expect(club.checked).toBe(true);
  });

  it('avvia la partita con le scelte fatte', async () => {
    const onStart = vi.fn();
    render(<NewGameDialog initialSetup={LAST_SETUP} onStart={onStart} onClose={vi.fn()} />);

    fireEvent.click(screen.getByRole('radio', { name: /Nero/ }));
    fireEvent.click(await screen.findByRole('radio', { name: /Principiante/ }));
    fireEvent.click(screen.getByRole('radio', { name: /Senza orologio/ }));
    fireEvent.click(screen.getByRole('button', { name: 'Inizia la partita' }));

    expect(onStart).toHaveBeenCalledWith({
      opponent: 'computer',
      color: 'black',
      engine: 'stockfish',
      level: 1,
      maiaLevel: 5,
      timeControl: null,
    });
  });

  it('contro Maia propone i suoi livelli e li ricorda a parte', async () => {
    const onStart = vi.fn();
    render(<NewGameDialog initialSetup={LAST_SETUP} onStart={onStart} onClose={vi.fn()} />);

    fireEvent.click(screen.getByRole('radio', { name: /Umano/ }));
    expect(await screen.findByRole('group', { name: 'Livello di Maia' })).toBeDefined();
    const level1500 = await screen.findByRole<HTMLInputElement>('radio', { name: /1500/ });
    expect(level1500.checked).toBe(true);
    fireEvent.click(screen.getByRole('radio', { name: /1100/ }));
    fireEvent.click(screen.getByRole('button', { name: 'Inizia la partita' }));

    expect(onStart).toHaveBeenCalledWith(
      expect.objectContaining({ opponent: 'computer', engine: 'maia', level: 4, maiaLevel: 1 }),
    );
  });

  it('a due giocatori non chiede colore e livello', () => {
    render(<NewGameDialog initialSetup={LAST_SETUP} onStart={vi.fn()} onClose={vi.fn()} />);

    fireEvent.click(screen.getByRole('radio', { name: /Due giocatori/ }));

    expect(screen.queryByRole('group', { name: 'Il tuo colore' })).toBeNull();
  });

  it('con "Personalizzato" si scelgono minuti e incremento', () => {
    const onStart = vi.fn();
    render(<NewGameDialog initialSetup={LAST_SETUP} onStart={onStart} onClose={vi.fn()} />);

    fireEvent.click(screen.getByRole('radio', { name: /Personalizzato/ }));
    fireEvent.change(screen.getByLabelText('Minuti a testa'), { target: { value: '7' } });
    fireEvent.change(screen.getByLabelText('Secondi in più a mossa'), { target: { value: '2' } });
    fireEvent.click(screen.getByRole('button', { name: 'Inizia la partita' }));

    expect(onStart).toHaveBeenCalledWith(
      expect.objectContaining({ timeControl: { initialSeconds: 420, incrementSeconds: 2 } }),
    );
  });

  it('Esc chiude senza avviare nulla', () => {
    const onClose = vi.fn();
    const onStart = vi.fn();
    render(<NewGameDialog initialSetup={LAST_SETUP} onStart={onStart} onClose={onClose} />);

    fireEvent.keyDown(window, { key: 'Escape' });

    expect(onClose).toHaveBeenCalled();
    expect(onStart).not.toHaveBeenCalled();
  });
});
