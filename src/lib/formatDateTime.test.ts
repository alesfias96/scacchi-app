/**
 * Test della scrittura di data e ora.
 */
import { describe, expect, it } from 'vitest';
import { formatDateTime } from './formatDateTime';

describe('formatDateTime', () => {
  it('scrive giorno, mese, anno e ora in italiano', () => {
    const text = formatDateTime(new Date(2026, 9, 18, 16, 35).getTime());

    expect(text).toContain('18');
    expect(text).toContain('ott');
    expect(text).toContain('2026');
    expect(text).toContain('16:35');
  });
});
