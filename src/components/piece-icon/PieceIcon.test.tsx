/**
 * Test dell'icona di un pezzo.
 */
import { render, screen } from '@testing-library/react';
import { beforeEach, describe, expect, it } from 'vitest';
import { useSettingsStore } from '../../state/settingsStore';
import { PieceIcon } from './PieceIcon';

describe('PieceIcon', () => {
  beforeEach(() => {
    useSettingsStore.setState({
      settings: { theme: 'studio', board: 'rovere', pieces: 'legno', notation: 'figurine' },
    });
  });

  it('usa lo stile scelto nelle impostazioni', () => {
    render(<PieceIcon piece="wQ" label="Donna" />);

    expect(screen.getByRole<HTMLImageElement>('img', { name: 'Donna' }).src).toContain(
      'legno/wQ.svg',
    );
  });

  it('usa lo stile indicato, se presente', () => {
    render(<PieceIcon piece="bN" style="metallo" label="Cavallo" />);

    expect(screen.getByRole<HTMLImageElement>('img', { name: 'Cavallo' }).src).toContain(
      'metallo/bN.svg',
    );
  });
});
