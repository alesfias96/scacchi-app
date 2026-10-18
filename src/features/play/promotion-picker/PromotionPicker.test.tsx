/**
 * Test della scelta della promozione.
 */
import { fireEvent, render, screen } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { pieceImageUrl } from '../../../components/piece-icon/pieceImageUrl';
import { useSettingsStore } from '../../../state/settingsStore';
import { PromotionPicker } from './PromotionPicker';

describe('PromotionPicker', () => {
  beforeEach(() => {
    useSettingsStore.setState({
      settings: {
        theme: 'studio',
        board: 'rovere',
        pieces: 'gettoni',
        notation: 'figurine',
        sounds: 'on',
      },
    });
  });

  it('mostra i quattro pezzi del colore giusto, nello stile scelto', () => {
    render(<PromotionPicker side="black" onChoose={vi.fn()} onCancel={vi.fn()} />);

    const knightImage = screen.getByRole('button', { name: 'Cavallo' }).querySelector('img');
    // Si confronta con l'immagine attesa, non con il nome del file: Vite
    // inserisce le immagini piccole direttamente nel codice (indirizzi `data:`).
    expect(knightImage?.getAttribute('src')).toBe(pieceImageUrl('gettoni', 'bN'));
    expect(screen.getAllByRole('button')).toHaveLength(4);
  });

  it('comunica il pezzo scelto', () => {
    const onChoose = vi.fn();
    render(<PromotionPicker side="white" onChoose={onChoose} onCancel={vi.fn()} />);

    fireEvent.click(screen.getByRole('button', { name: 'Cavallo' }));

    expect(onChoose).toHaveBeenCalledWith('n');
  });

  it('Esc annulla', () => {
    const onCancel = vi.fn();
    render(<PromotionPicker side="white" onChoose={vi.fn()} onCancel={onCancel} />);

    fireEvent.keyDown(window, { key: 'Escape' });

    expect(onCancel).toHaveBeenCalled();
  });

  it('la donna riceve subito il focus', () => {
    render(<PromotionPicker side="white" onChoose={vi.fn()} onCancel={vi.fn()} />);

    expect(document.activeElement).toBe(screen.getByRole('button', { name: 'Donna' }));
  });
});
