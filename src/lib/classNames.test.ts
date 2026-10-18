/**
 * Test di classNames.
 */
import { describe, expect, it } from 'vitest';
import { classNames } from './classNames';

describe('classNames', () => {
  it('unisce i nomi con uno spazio', () => {
    expect(classNames('item', 'active')).toBe('item active');
  });

  it('scarta i valori vuoti o falsi', () => {
    expect(classNames('item', false, undefined, null, '', 'active')).toBe('item active');
  });

  it('restituisce una stringa vuota se non ci sono nomi validi', () => {
    expect(classNames(undefined, false)).toBe('');
  });
});
