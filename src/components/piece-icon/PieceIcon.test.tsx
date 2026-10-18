/**
 * Test dell'icona di un pezzo.
 *
 * L'immagine mostrata si confronta con quella restituita da `pieceImageUrl`
 * per lo stile e il pezzo attesi, non con il nome del file: Vite inserisce le
 * immagini piccole direttamente nel codice (indirizzi `data:`), quindi il
 * nome del file non compare nell'indirizzo.
 */
import { render, screen } from '@testing-library/react';
import { beforeEach, describe, expect, it } from 'vitest';
import { useSettingsStore } from '../../state/settingsStore';
import { PieceIcon } from './PieceIcon';
import { pieceImageUrl } from './pieceImageUrl';

describe('PieceIcon', () => {
  beforeEach(() => {
    useSettingsStore.setState({
      settings: {
        theme: 'studio',
        board: 'rovere',
        pieces: 'legno',
        notation: 'figurine',
        sounds: 'on',
      },
    });
  });

  it('usa lo stile scelto nelle impostazioni', () => {
    render(<PieceIcon piece="wQ" label="Donna" />);

    expect(screen.getByRole('img', { name: 'Donna' }).getAttribute('src')).toBe(
      pieceImageUrl('legno', 'wQ'),
    );
  });

  it('usa lo stile indicato, se presente', () => {
    render(<PieceIcon piece="bN" style="metallo" label="Cavallo" />);

    expect(screen.getByRole('img', { name: 'Cavallo' }).getAttribute('src')).toBe(
      pieceImageUrl('metallo', 'bN'),
    );
  });
});
