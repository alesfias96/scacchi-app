/**
 * Il suono di una presa: due colpi ravvicinati.
 *
 * Prima un colpo leggero e acuto (il pezzo catturato viene urtato e tolto),
 * poi, subito dopo, un colpo più grave e deciso (il pezzo che cattura si
 * appoggia). Si distingue a orecchio da una mossa normale senza essere invadente.
 */
import { getSoundOutput } from './getSoundOutput';
import { playWoodTap } from './playWoodTap';

/** Distanza tra i due colpi, in secondi: abbastanza vicini da sembrare un gesto solo. */
const SECOND_TAP_DELAY = 0.035;

/**
 * Fa sentire il suono della presa. Se il motore web non sa produrre suoni,
 * non fa nulla.
 *
 * @param delay - dopo quanti secondi suonare (predefinito: subito).
 */
export function playCaptureSound(delay = 0) {
  const output = getSoundOutput();
  if (output === null) {
    return;
  }
  const start = output.context.currentTime + delay;
  playWoodTap(output, { at: start, pitch: 330, level: 0.45 });
  playWoodTap(output, { at: start + SECOND_TAP_DELAY, pitch: 185, level: 0.9 });
}
