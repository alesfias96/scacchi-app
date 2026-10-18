/**
 * Il suono della fine della partita.
 *
 * - **Partita decisa** (scacco matto): quattro note che salgono (Do, Mi, Sol,
 *   Do acuto), come una piccola sigla che chiude la partita.
 * - **Patta**: due note calme che scendono (Mi, Do), più sobrie: nessuno ha vinto.
 * Sono note morbide, a volume più basso dei colpi dei pezzi.
 */
import { getSoundOutput } from './getSoundOutput';
import { playSoftNote } from './playSoftNote';

/** Il tipo di fine partita, che decide quale melodia suonare. */
export type GameEndSound = 'decisive' | 'draw';

/** Le note di ogni melodia, in hertz. */
const MELODIES: Record<GameEndSound, readonly number[]> = {
  decisive: [523.25, 659.25, 783.99, 1046.5],
  draw: [659.25, 523.25],
};
/** Distanza tra una nota e la successiva, in secondi. */
const NOTE_SPACING: Record<GameEndSound, number> = {
  decisive: 0.11,
  draw: 0.2,
};

/**
 * Fa sentire il suono di fine partita. Se il motore web non sa produrre
 * suoni, non fa nulla.
 *
 * @param kind - partita decisa o patta.
 * @param delay - dopo quanti secondi suonare (predefinito: subito).
 */
export function playGameEndSound(kind: GameEndSound, delay = 0) {
  const output = getSoundOutput();
  if (output === null) {
    return;
  }
  const start = output.context.currentTime + delay;
  MELODIES[kind].forEach((frequency, index) => {
    playSoftNote(output, {
      at: start + index * NOTE_SPACING[kind],
      frequency,
      level: 0.3,
      duration: 1.4,
    });
  });
}
