/**
 * Test di SettingsGate. La comunicazione con Rust è simulata.
 */
import { render, screen } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { useSettingsStore } from '../../state/settingsStore';
import { SettingsGate } from './SettingsGate';

const serviceMock = vi.hoisted(() => ({
  fetchSettings: vi.fn(),
  saveSettings: vi.fn(),
}));

vi.mock('../../services/settingsService', () => serviceMock);

describe('SettingsGate', () => {
  beforeEach(() => {
    serviceMock.fetchSettings.mockReset();
    useSettingsStore.setState({ settings: null, saveError: null });
  });

  it('mostra l’app dopo aver letto le impostazioni', async () => {
    serviceMock.fetchSettings.mockResolvedValue({
      theme: 'studio',
      board: 'rovere',
      pieces: 'icona',
      notation: 'figurine',
      sounds: 'on',
      gameSetup: { opponent: 'computer', color: 'white', level: 4, timeControl: null },
      trainingSetup: {
        mode: 'depth',
        depth: 2,
        theme: null,
        difficulty: 'normal',
        visualizationSeconds: 5,
      },
    });

    render(
      <SettingsGate>
        <p>App pronta</p>
      </SettingsGate>,
    );

    expect(await screen.findByText('App pronta')).toBeDefined();
  });

  it('spiega il problema se le impostazioni non si possono leggere', async () => {
    serviceMock.fetchSettings.mockRejectedValue('comando non disponibile');

    render(
      <SettingsGate>
        <p>App pronta</p>
      </SettingsGate>,
    );

    const alert = await screen.findByRole('alert');
    expect(alert.textContent).toContain('comando non disponibile');
    expect(screen.queryByText('App pronta')).toBeNull();
  });
});
