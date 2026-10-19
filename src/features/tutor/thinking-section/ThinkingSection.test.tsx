/**
 * Test di «Prima di muovere».
 */
import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { THINKING } from '../../../test/tutorExplanations';
import { ThinkingSection } from './ThinkingSection';

describe('ThinkingSection', () => {
  it('mostra i passi in ordine con le mosse nella notazione scelta', () => {
    render(<ThinkingSection thinking={THINKING} notation="italiana" />);

    const steps = screen.getAllByRole('listitem');
    expect(steps[0]?.textContent).toContain('I tuoi scacchi');
    expect(screen.getByText('Dxf7+')).toBeDefined();
    expect(screen.getByText('Il controllo finale')).toBeDefined();
  });
});
