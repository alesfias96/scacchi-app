/**
 * Test della richiesta di ridurre le animazioni.
 */
import { describe, expect, it, vi } from 'vitest';
import { prefersReducedMotion } from './prefersReducedMotion';

describe('prefersReducedMotion', () => {
  it('segue l’opzione del sistema', () => {
    expect(prefersReducedMotion()).toBe(false);

    vi.spyOn(window, 'matchMedia').mockReturnValue({ matches: true } as MediaQueryList);

    expect(prefersReducedMotion()).toBe(true);
  });
});
