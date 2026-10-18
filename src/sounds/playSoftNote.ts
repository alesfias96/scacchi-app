/**
 * Una nota morbida, simile a una campanella di legno (marimba).
 *
 * È la "nota" dei suoni di fine partita. Si ottiene sommando la nota
 * principale e due sue armoniche più deboli, che danno il timbro; il suono
 * entra dolcemente (niente colpo secco) e si spegne piano.
 */
import type { SoundOutput } from './getSoundOutput';

/** Volume quasi nullo: le dissolvenze esponenziali non possono arrivare a zero. */
const SILENCE = 0.0001;
/** Tempo di entrata della nota, in secondi: abbastanza da non sembrare un colpo. */
const ATTACK_SECONDS = 0.012;
/** Le armoniche: [multiplo della nota, intensità relativa, durata relativa]. */
const PARTIALS: readonly (readonly [number, number, number])[] = [
  [1, 1, 1],
  [2, 0.25, 0.5],
  [3, 0.08, 0.3],
];

/** Come suona la nota. */
export interface SoftNoteOptions {
  /** Quando suona, sull'orologio del mixer, in secondi. */
  at: number;
  /** La nota, in hertz (es. 523,25 = Do). */
  frequency: number;
  /** L'intensità, da 0 a 1. */
  level: number;
  /** In quanti secondi si spegne. */
  duration: number;
}

/**
 * Programma una nota sull'uscita audio.
 *
 * @param output - l'uscita audio (da `getSoundOutput`).
 * @param options - quando, quale nota, con che intensità e per quanto.
 */
export function playSoftNote(
  { context, destination }: SoundOutput,
  { at, frequency, level, duration }: SoftNoteOptions,
) {
  for (const [multiple, strength, length] of PARTIALS) {
    const end = at + duration * length;
    const tone = context.createOscillator();
    tone.type = 'sine';
    tone.frequency.value = frequency * multiple;
    const volume = context.createGain();
    volume.gain.setValueAtTime(SILENCE, at);
    volume.gain.exponentialRampToValueAtTime(level * strength, at + ATTACK_SECONDS);
    volume.gain.exponentialRampToValueAtTime(SILENCE, end);
    tone.connect(volume).connect(destination);
    tone.start(at);
    tone.stop(end);
  }
}
