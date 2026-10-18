/**
 * Test di resolveTheme.
 */
import { describe, expect, it } from 'vitest';
import { resolveTheme } from './resolveTheme';

describe('resolveTheme', () => {
  it('lascia invariati i temi concreti', () => {
    expect(resolveTheme('studio', true)).toBe('studio');
    expect(resolveTheme('giorno', true)).toBe('giorno');
    expect(resolveTheme('sera', false)).toBe('sera');
  });

  it('con "auto" usa Sera se il sistema è in modalità scura', () => {
    expect(resolveTheme('auto', true)).toBe('sera');
  });

  it('con "auto" usa Giorno se il sistema è in modalità chiara', () => {
    expect(resolveTheme('auto', false)).toBe('giorno');
  });
});
