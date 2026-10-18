/**
 * Test di ThemeController: il tema scelto finisce sull'elemento <html>.
 */
import { render } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { useSettingsStore } from '../../state/settingsStore';
import { ThemeController } from './ThemeController';

/** Simula la modalità del sistema operativo (scura o chiara). */
function simulateSystemDarkMode(isDark: boolean) {
  vi.spyOn(window, 'matchMedia').mockImplementation(
    (query: string) =>
      ({
        matches: isDark,
        media: query,
        addEventListener: () => undefined,
        removeEventListener: () => undefined,
      }) as unknown as MediaQueryList,
  );
}

describe('ThemeController', () => {
  beforeEach(() => {
    document.documentElement.dataset.theme = 'studio';
  });

  it('applica il tema scelto', () => {
    useSettingsStore.setState({
      settings: {
        theme: 'sera',
        board: 'rovere',
        pieces: 'icona',
        notation: 'figurine',
        sounds: 'on',
      },
    });

    render(<ThemeController />);

    expect(document.documentElement.dataset.theme).toBe('sera');
  });

  it('con il tema automatico segue la modalità scura del sistema', () => {
    simulateSystemDarkMode(true);
    useSettingsStore.setState({
      settings: {
        theme: 'auto',
        board: 'rovere',
        pieces: 'icona',
        notation: 'figurine',
        sounds: 'on',
      },
    });

    render(<ThemeController />);

    expect(document.documentElement.dataset.theme).toBe('sera');
  });

  it('con il tema automatico segue la modalità chiara del sistema', () => {
    simulateSystemDarkMode(false);
    useSettingsStore.setState({
      settings: {
        theme: 'auto',
        board: 'rovere',
        pieces: 'icona',
        notation: 'figurine',
        sounds: 'on',
      },
    });

    render(<ThemeController />);

    expect(document.documentElement.dataset.theme).toBe('giorno');
  });
});
