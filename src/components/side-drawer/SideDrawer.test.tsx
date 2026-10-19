/**
 * Test della barra laterale a comparsa.
 */
import { act, fireEvent, render, screen } from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { SideDrawer } from './SideDrawer';

/** Disegna la barra con un contenuto di prova. */
function renderDrawer(shortcutEnabled = true) {
  return render(
    <SideDrawer title="Partita" shortcut="i" shortcutEnabled={shortcutEnabled}>
      <p>Contenuto della barra</p>
    </SideDrawer>,
  );
}

/** La barra (l'elemento `aside`). */
function drawer(): HTMLElement {
  return screen.getByRole('complementary', { name: 'Partita' });
}

/** La linguetta sul bordo. */
function handle(): HTMLElement {
  return screen.getByRole('button', { name: 'Partita' });
}

describe('SideDrawer', () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });
  afterEach(() => {
    vi.useRealTimers();
  });

  it('parte chiusa e si apre portando il mouse sul bordo', () => {
    const { container } = renderDrawer();
    expect(handle().getAttribute('aria-expanded')).toBe('false');

    fireEvent.mouseEnter(container.firstElementChild as HTMLElement);

    expect(handle().getAttribute('aria-expanded')).toBe('true');
    expect(screen.getByText('Contenuto della barra')).toBeDefined();
  });

  it('si richiude poco dopo che il mouse l’ha lasciata', () => {
    renderDrawer();
    fireEvent.click(handle());

    fireEvent.mouseLeave(drawer());
    expect(handle().getAttribute('aria-expanded')).toBe('true');
    act(() => {
      vi.advanceTimersByTime(400);
    });

    expect(handle().getAttribute('aria-expanded')).toBe('false');
  });

  it('fissata resta aperta anche senza il mouse', () => {
    renderDrawer();
    fireEvent.click(handle());
    fireEvent.click(screen.getByRole('button', { name: 'Fissa' }));

    fireEvent.mouseLeave(drawer());
    act(() => {
      vi.advanceTimersByTime(1000);
    });

    expect(handle().getAttribute('aria-expanded')).toBe('true');
    expect(screen.getByRole('button', { name: 'Fissata' }).getAttribute('aria-pressed')).toBe(
      'true',
    );
  });

  it('il tasto della scorciatoia apre e chiude, Esc chiude', () => {
    renderDrawer();

    fireEvent.keyDown(window, { key: 'i' });
    expect(handle().getAttribute('aria-expanded')).toBe('true');
    fireEvent.keyDown(window, { key: 'Escape' });
    expect(handle().getAttribute('aria-expanded')).toBe('false');
    fireEvent.keyDown(window, { key: 'I' });
    fireEvent.keyDown(window, { key: 'i' });
    expect(handle().getAttribute('aria-expanded')).toBe('false');
  });

  it('ignora la scorciatoia quando è disattivata o si sta scrivendo', () => {
    renderDrawer(false);
    fireEvent.keyDown(window, { key: 'i' });
    expect(handle().getAttribute('aria-expanded')).toBe('false');
  });

  it('avvisa quando si apre e quando si chiude', () => {
    const onOpenChange = vi.fn<(isOpen: boolean) => void>();
    render(
      <SideDrawer title="Tutor" shortcut="t" onOpenChange={onOpenChange}>
        <p>Contenuto</p>
      </SideDrawer>,
    );
    expect(onOpenChange).toHaveBeenLastCalledWith(false);

    fireEvent.keyDown(window, { key: 't' });
    expect(onOpenChange).toHaveBeenLastCalledWith(true);

    fireEvent.keyDown(window, { key: 'Escape' });
    expect(onOpenChange).toHaveBeenLastCalledWith(false);
  });
});
