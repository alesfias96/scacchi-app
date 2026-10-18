/**
 * Il suono di una mossa normale: un pezzo di legno appoggiato sulla casa.
 */
import { getSoundOutput } from './getSoundOutput';
import { playWoodTap } from './playWoodTap';

/**
 * Fa sentire il suono della mossa. Se il motore web non sa produrre suoni,
 * non fa nulla.
 *
 * @param delay - dopo quanti secondi suonare (predefinito: subito).
 */
export function playMoveSound(delay = 0) {
  const output = getSoundOutput();
  if (output === null) {
    return;
  }
  playWoodTap(output, { at: output.context.currentTime + delay, pitch: 220, level: 0.7 });
}
