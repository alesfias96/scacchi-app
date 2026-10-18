/**
 * Test della sezione Impostazioni. Rust e chessground sono simulati.
 */
import { fireEvent, render, screen } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { useSettingsStore } from '../../state/settingsStore';
import { SettingsPage } from './SettingsPage';

const settingsServiceMock = vi.hoisted(() => ({
  fetchSettings: vi.fn(),
  saveSettings: vi.fn(),
}));

vi.mock('../../services/settingsService', () => settingsServiceMock);

// La posizione dell'anteprima resta "in caricamento": qui si controllano le scelte.
vi.mock('../../services/positionService', () => ({
  fetchInitialPosition: () => new Promise(() => undefined),
}));

describe('SettingsPage', () => {
  beforeEach(() => {
    settingsServiceMock.saveSettings.mockReset();
    settingsServiceMock.saveSettings.mockResolvedValue(undefined);
    useSettingsStore.setState({
      settings: { theme: 'studio', board: 'rovere', pieces: 'icona', notation: 'figurine' },
      saveError: null,
    });
  });

  it('mostra le scelte attuali come selezionate', () => {
    render(<SettingsPage />);

    expect(screen.getByRole<HTMLInputElement>('radio', { name: /Studio/ }).checked).toBe(true);
    expect(screen.getByRole<HTMLInputElement>('radio', { name: /Rovere/ }).checked).toBe(true);
    expect(screen.getByRole<HTMLInputElement>('radio', { name: /Icona/ }).checked).toBe(true);
  });

  it('applica e salva una nuova scacchiera', () => {
    render(<SettingsPage />);

    fireEvent.click(screen.getByRole('radio', { name: /Marmo/ }));

    expect(useSettingsStore.getState().settings?.board).toBe('marmo');
    expect(settingsServiceMock.saveSettings).toHaveBeenCalledWith({
      theme: 'studio',
      board: 'marmo',
      pieces: 'icona',
      notation: 'figurine',
    });
  });

  it('applica e salva un nuovo stile dei pezzi', () => {
    render(<SettingsPage />);

    fireEvent.click(screen.getByRole('radio', { name: /Gettoni/ }));

    expect(useSettingsStore.getState().settings?.pieces).toBe('gettoni');
  });

  it('spiega il problema se il salvataggio non riesce', () => {
    useSettingsStore.setState({ saveError: 'disco pieno' });

    render(<SettingsPage />);

    expect(screen.getByRole('alert').textContent).toContain('disco pieno');
  });

  it('applica e salva una nuova notazione', () => {
    render(<SettingsPage />);

    fireEvent.click(screen.getByRole('radio', { name: /Italiana/ }));

    expect(useSettingsStore.getState().settings?.notation).toBe('italiana');
  });
});
