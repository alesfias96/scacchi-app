/**
 * Il colpo di un pezzo di legno appoggiato sulla scacchiera.
 *
 * È fatto di due parti che suonano insieme:
 * - il **clic**: un fruscio filtrato che dura pochi millesimi di secondo,
 *   come il contatto secco tra legno e legno;
 * - il **corpo**: una nota grave che scende velocemente di tono e si spegne,
 *   come la risonanza della tavola sotto il pezzo.
 * Cambiando tono e intensità si ottengono il suono della mossa e quello della presa.
 */
import { getNoiseBuffer } from './getNoiseBuffer';
import type { SoundOutput } from './getSoundOutput';

/** Volume quasi nullo: le dissolvenze esponenziali non possono arrivare a zero. */
const SILENCE = 0.0001;
/** Durata del clic, in secondi. */
const CLICK_SECONDS = 0.045;
/** Durata del corpo, in secondi. */
const BODY_SECONDS = 0.11;
/** Il clic sta circa tre ottave sopra il corpo: è la "voce" del legno. */
const CLICK_PITCH_RATIO = 8;
/** Il corpo parte un po' più acuto e scende al suo tono, come una tavola colpita. */
const BODY_START_RATIO = 1.6;

/** Come suona il colpo. */
export interface WoodTapOptions {
  /** Quando suona, sull'orologio del mixer (`context.currentTime`), in secondi. */
  at: number;
  /** Il tono del corpo, in hertz: più basso = pezzo più pesante. */
  pitch: number;
  /** L'intensità, da 0 a 1. */
  level: number;
}

/**
 * Programma un colpo sull'uscita audio.
 *
 * @param output - l'uscita audio (da `getSoundOutput`).
 * @param options - quando, con che tono e con che intensità.
 */
export function playWoodTap(
  { context, destination }: SoundOutput,
  { at, pitch, level }: WoodTapOptions,
) {
  // Il clic: fruscio → filtro che lascia passare solo la "voce" del legno → volume.
  const noise = context.createBufferSource();
  noise.buffer = getNoiseBuffer(context);
  const filter = context.createBiquadFilter();
  filter.type = 'bandpass';
  filter.frequency.value = pitch * CLICK_PITCH_RATIO;
  filter.Q.value = 1.2;
  const clickVolume = context.createGain();
  clickVolume.gain.setValueAtTime(SILENCE, at);
  clickVolume.gain.exponentialRampToValueAtTime(level, at + 0.002);
  clickVolume.gain.exponentialRampToValueAtTime(SILENCE, at + CLICK_SECONDS);
  noise.connect(filter).connect(clickVolume).connect(destination);
  noise.start(at);
  noise.stop(at + CLICK_SECONDS);

  // Il corpo: nota che scende di tono → volume che si spegne in fretta.
  const body = context.createOscillator();
  body.type = 'sine';
  body.frequency.setValueAtTime(pitch * BODY_START_RATIO, at);
  body.frequency.exponentialRampToValueAtTime(pitch, at + 0.03);
  const bodyVolume = context.createGain();
  bodyVolume.gain.setValueAtTime(SILENCE, at);
  bodyVolume.gain.exponentialRampToValueAtTime(level * 0.8, at + 0.004);
  bodyVolume.gain.exponentialRampToValueAtTime(SILENCE, at + BODY_SECONDS);
  body.connect(bodyVolume).connect(destination);
  body.start(at);
  body.stop(at + BODY_SECONDS);
}
