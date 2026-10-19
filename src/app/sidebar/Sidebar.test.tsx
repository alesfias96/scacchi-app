/**
 * Test della barra laterale: sezione attiva, Impostazioni e sezioni in arrivo.
 */
import { screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { renderWithRouter } from '../../test/renderWithRouter';
import { Sidebar } from './Sidebar';

describe('Sidebar', () => {
  it('segna come pagina corrente la sezione aperta', async () => {
    renderWithRouter(<Sidebar />, '/');

    const playLink = await screen.findByRole('link', { name: 'Gioca' });

    expect(playLink.getAttribute('aria-current')).toBe('page');
  });

  it('segna le Impostazioni quando sono aperte', async () => {
    renderWithRouter(<Sidebar />, '/impostazioni');

    const settingsLink = await screen.findByRole('link', { name: 'Impostazioni' });
    const playLink = screen.getByRole('link', { name: 'Gioca' });

    expect(settingsLink.getAttribute('aria-current')).toBe('page');
    expect(playLink.getAttribute('aria-current')).toBeNull();
  });

  it('disattiva le sezioni non ancora disponibili e indica la fase', async () => {
    renderWithRouter(<Sidebar />, '/');

    const progressButton = await screen.findByRole('button', { name: /Progressi/ });

    expect(progressButton.hasAttribute('disabled')).toBe(true);
    expect(progressButton.textContent).toContain('Fase 9');
  });

  it('porta alla modalità Tutor', async () => {
    renderWithRouter(<Sidebar />, '/');

    const link = await screen.findByRole('link', { name: 'Tutor' });

    expect(link.getAttribute('href')).toContain('/tutor');
  });

  it('porta all’archivio delle partite', async () => {
    renderWithRouter(<Sidebar />, '/archivio');

    const archiveLink = await screen.findByRole('link', { name: 'Archivio' });

    expect(archiveLink.getAttribute('aria-current')).toBe('page');
  });
});
