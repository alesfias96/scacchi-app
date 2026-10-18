/**
 * Test della copia negli appunti.
 */
import { describe, expect, it, vi } from 'vitest';
import { copyToClipboard } from './copyToClipboard';

describe('copyToClipboard', () => {
  it('scrive il testo negli appunti del sistema', async () => {
    const writeText = vi.fn(() => Promise.resolve());
    Object.defineProperty(navigator, 'clipboard', { value: { writeText }, configurable: true });

    await copyToClipboard('1. e4 *');

    expect(writeText).toHaveBeenCalledWith('1. e4 *');
  });
});
