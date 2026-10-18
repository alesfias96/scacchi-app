/**
 * Test della scrittura delle mosse nelle tre notazioni.
 */
import { describe, expect, it } from 'vitest';
import { formatSan } from './formatSan';

describe('formatSan', () => {
  it('usa le figurine', () => {
    expect(formatSan('Nf3', 'figurine')).toBe('♞f3');
    expect(formatSan('Qxd8+', 'figurine')).toBe('♛xd8+');
    expect(formatSan('e8=Q#', 'figurine')).toBe('e8=♛#');
  });

  it('usa le iniziali italiane', () => {
    expect(formatSan('Kf1', 'italiana')).toBe('Rf1');
    expect(formatSan('Rxe7', 'italiana')).toBe('Txe7');
    expect(formatSan('Bb5+', 'italiana')).toBe('Ab5+');
    expect(formatSan('Nbd7', 'italiana')).toBe('Cbd7');
    expect(formatSan('Qh4#', 'italiana')).toBe('Dh4#');
  });

  it('lascia invariata la notazione internazionale', () => {
    expect(formatSan('Nf3', 'internazionale')).toBe('Nf3');
  });

  it('non tocca pedoni, colonne e arrocco', () => {
    expect(formatSan('exd5', 'italiana')).toBe('exd5');
    expect(formatSan('O-O-O', 'figurine')).toBe('O-O-O');
    expect(formatSan('bxc8=N', 'italiana')).toBe('bxc8=C');
  });
});
