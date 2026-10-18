/**
 * Test dello stato delle impostazioni. La comunicazione con Rust è simulata.
 */
import { beforeEach, describe, expect, it, vi } from 'vitest';
import type { Settings } from '../bindings/Settings';
import { useSettingsStore } from './settingsStore';

const serviceMock = vi.hoisted(() => ({
  fetchSettings: vi.fn(),
  saveSettings: vi.fn(),
}));

vi.mock('../services/settingsService', () => serviceMock);

const DEFAULTS: Settings = {
  theme: 'studio',
  board: 'rovere',
  pieces: 'icona',
  notation: 'figurine',
};

describe('settingsStore', () => {
  beforeEach(() => {
    serviceMock.fetchSettings.mockReset();
    serviceMock.saveSettings.mockReset();
    useSettingsStore.setState({ settings: null, saveError: null });
  });

  it('carica le impostazioni da Rust', async () => {
    serviceMock.fetchSettings.mockResolvedValue(DEFAULTS);

    await useSettingsStore.getState().load();

    expect(useSettingsStore.getState().settings).toEqual(DEFAULTS);
  });

  it('applica subito una modifica e la salva', async () => {
    useSettingsStore.setState({ settings: DEFAULTS });
    serviceMock.saveSettings.mockResolvedValue(undefined);

    await useSettingsStore.getState().update({ board: 'marmo' });

    expect(useSettingsStore.getState().settings).toEqual({ ...DEFAULTS, board: 'marmo' });
    expect(serviceMock.saveSettings).toHaveBeenCalledWith({ ...DEFAULTS, board: 'marmo' });
  });

  it('annulla la modifica e spiega il motivo se il salvataggio fallisce', async () => {
    useSettingsStore.setState({ settings: DEFAULTS });
    serviceMock.saveSettings.mockRejectedValue('disco pieno');

    await useSettingsStore.getState().update({ pieces: 'gettoni' });

    expect(useSettingsStore.getState().settings).toEqual(DEFAULTS);
    expect(useSettingsStore.getState().saveError).toBe('disco pieno');
  });
});
