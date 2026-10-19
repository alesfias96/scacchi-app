/**
 * Test della barra laterale: sezione attiva, Impostazioni e collegamenti alle sezioni.
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

  it('porta alla sezione Progressi', async () => {
    renderWithRouter(<Sidebar />, '/progressi');

    const link = await screen.findByRole('link', { name: 'Progressi' });

    expect(link.getAttribute('aria-current')).toBe('page');
  });

  it('porta agli appunti', async () => {
    renderWithRouter(<Sidebar />, '/appunti');

    const link = await screen.findByRole('link', { name: 'Appunti' });

    expect(link.getAttribute('aria-current')).toBe('page');
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
