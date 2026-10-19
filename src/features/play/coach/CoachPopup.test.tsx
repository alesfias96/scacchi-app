/**
 * Test del messaggio del tutor sopra la scacchiera.
 */
import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import type { CoachMessage } from './buildCoachMessages';
import { CoachPopup } from './CoachPopup';

function message(text: string, cardId: string | null = null): CoachMessage {
  return {
    heading: 'Il piano',
    fact: { category: 'activity', side: 'white', text, squares: [], arrows: [], moves: [] },
    line: null,
    cardId,
  };
}

const MESSAGES = [message('Primo messaggio', 'principles/center'), message('Secondo messaggio')];

function renderPopup(index: number) {
  const onIndexChange = vi.fn<(index: number) => void>();
  const onClose = vi.fn<() => void>();
  const onOpenCard = vi.fn<(cardId: string) => void>();
  render(
    <CoachPopup
      messages={MESSAGES}
      index={index}
      notation="italiana"
      onIndexChange={onIndexChange}
      onClose={onClose}
      onOpenCard={onOpenCard}
    />,
  );
  return { onIndexChange, onClose, onOpenCard };
}

describe('CoachPopup', () => {
  it('mostra un messaggio alla volta, con il conteggio', () => {
    renderPopup(0);

    expect(screen.getByText('Primo messaggio')).toBeDefined();
    expect(screen.getByText('1 di 2')).toBeDefined();
    expect(screen.queryByText('Secondo messaggio')).toBeNull();
  });

  it('va avanti con il pulsante o con Invio, e chiude con Esc', () => {
    const { onIndexChange, onClose } = renderPopup(0);

    fireEvent.click(screen.getByRole('button', { name: 'Avanti' }));
    expect(onIndexChange).toHaveBeenLastCalledWith(1);

    fireEvent.keyDown(window, { key: 'Enter' });
    expect(onIndexChange).toHaveBeenCalledTimes(2);

    fireEvent.keyDown(window, { key: 'Escape' });
    expect(onClose).toHaveBeenCalledOnce();
  });

  it('all’ultimo messaggio «Fine» chiude', () => {
    const { onClose } = renderPopup(1);

    fireEvent.click(screen.getByRole('button', { name: 'Fine' }));

    expect(onClose).toHaveBeenCalledOnce();
  });

  it('apre la scheda con «Approfondisci» e si sposta in alto', () => {
    const { onOpenCard } = renderPopup(0);

    fireEvent.click(screen.getByRole('button', { name: 'Approfondisci' }));
    expect(onOpenCard).toHaveBeenCalledWith('principles/center');

    fireEvent.click(screen.getByRole('button', { name: 'Sposta in alto' }));
    expect(screen.getByRole('button', { name: 'Sposta in basso' })).toBeDefined();
  });
});
