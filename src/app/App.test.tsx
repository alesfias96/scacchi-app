/**
 * Test del componente App: avvio completo con impostazioni e router.
 * La comunicazione con Rust è simulata.
 */
import { render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { App } from './App';

vi.mock('../services/settingsService', () => ({
  fetchSettings: () =>
    Promise.resolve({
      theme: 'studio',
      board: 'rovere',
      pieces: 'icona',
      notation: 'figurine',
      sounds: 'on',
      gameSetup: {
        opponent: 'computer',
        color: 'white',
        engine: 'stockfish',
        level: 4,
        maiaLevel: 5,
        timeControl: null,
      },
      trainingSetup: {
        mode: 'depth',
        depth: 2,
        theme: null,
        difficulty: 'normal',
        visualizationSeconds: 5,
      },
      statsFilter: { period: 'all', opponent: 'all' },
      updateCheck: 'manual',
    }),
  saveSettings: () => Promise.resolve(),
}));

// Posizione e partita restano "in caricamento": qui si controlla solo la struttura.
vi.mock('../services/positionService', () => ({
  fetchInitialPosition: () => new Promise(() => undefined),
}));

vi.mock('../services/gameService', () => ({
  fetchGame: () => new Promise(() => undefined),
}));

describe('App', () => {
  it("mostra la barra laterale con il nome dell'app", async () => {
    render(<App />);

    const navigation = await screen.findByRole('navigation', { name: "Sezioni dell'app" });

    expect(navigation.textContent).toContain('Scacchi');
  });

  it('apre la sezione Gioca', async () => {
    render(<App />);

    expect(await screen.findByRole('heading', { level: 1, name: 'Gioca' })).toBeDefined();
  });
});
