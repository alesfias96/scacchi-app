/**
 * Test dell'avviso di una versione nuova all'avvio. Il servizio verso Rust è simulato.
 */
import { fireEvent, render, screen, waitFor } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import type { UpdateCheck } from '../../bindings/UpdateCheck';
import { useSettingsStore } from '../../state/settingsStore';
import { useUpdateStore } from '../../state/updateStore';
import { UpdateNotice } from './UpdateNotice';

const updateMock = vi.hoisted(() => ({
  fetchAppVersion: vi.fn(),
  checkForUpdate: vi.fn(),
  installUpdate: vi.fn(),
}));

vi.mock('../../services/updateService', () => updateMock);

/** Prepara le impostazioni con la scelta del controllo indicata. */
function withUpdateCheck(updateCheck: UpdateCheck) {
  useSettingsStore.setState({
    settings: {
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
      updateCheck,
    },
    saveError: null,
  });
}

describe('UpdateNotice', () => {
  beforeEach(() => {
    vi.resetAllMocks();
    useUpdateStore.setState({ status: { kind: 'idle' }, noticeDismissed: false });
  });

  it('non si collega a Internet se non lo si è chiesto', () => {
    withUpdateCheck('manual');
    render(<UpdateNotice />);

    expect(updateMock.checkForUpdate).not.toHaveBeenCalled();
    expect(screen.queryByRole('complementary')).toBeNull();
  });

  it('all’avvio propone la versione nuova, e «Più tardi» la nasconde', async () => {
    withUpdateCheck('onStart');
    updateMock.checkForUpdate.mockResolvedValue('0.11.0');
    render(<UpdateNotice />);

    expect(await screen.findByText(/È disponibile la versione 0.11.0/)).toBeDefined();
    fireEvent.click(screen.getByRole('button', { name: 'Più tardi' }));

    await waitFor(() => {
      expect(screen.queryByRole('complementary')).toBeNull();
    });
    expect(updateMock.checkForUpdate).toHaveBeenCalledTimes(1);
  });
});
