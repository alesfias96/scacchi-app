/**
 * Test dei tempi proposti nella finestra "Nuova partita".
 */
import { describe, expect, it } from 'vitest';
import { CUSTOM_PRESET_ID, TIME_PRESETS, presetIdFor } from './timePresets';

describe('timePresets', () => {
  it('propone "senza orologio" e i tempi più comuni', () => {
    const ids = TIME_PRESETS.map((preset) => preset.id);

    expect(ids[0]).toBe('none');
    expect(ids).toContain('3+2');
    expect(ids).toContain('15+10');
  });

  it('riconosce un tempo proposto, oppure lo considera personalizzato', () => {
    expect(presetIdFor(null)).toBe('none');
    expect(presetIdFor({ initialSeconds: 300, incrementSeconds: 3 })).toBe('5+3');
    expect(presetIdFor({ initialSeconds: 420, incrementSeconds: 2 })).toBe(CUSTOM_PRESET_ID);
  });
});
