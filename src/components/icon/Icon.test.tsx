/**
 * Test dell'icona.
 */
import { render } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { Icon } from './Icon';
import { ICON_PATHS } from './iconPaths';

describe('Icon', () => {
  it('disegna tutti i tratti dell’icona ed è nascosta ai lettori di schermo', () => {
    const { container } = render(<Icon name="flip" />);

    const svg = container.querySelector('svg');
    expect(svg?.getAttribute('aria-hidden')).toBe('true');
    expect(svg?.querySelectorAll('path')).toHaveLength(ICON_PATHS.flip.length);
  });
});
