/**
 * Dice com'è andata una partita dell'archivio, in una parola.
 *
 * - Non finita: "In corso".
 * - Patta: "Patta".
 * - Contro il computer: "Vinta" o "Persa", dal punto di vista della persona.
 * - A due giocatori: chi ha vinto ("Vince il Bianco").
 *
 * @param game - il riassunto della partita.
 * @returns il testo e il "tono" (che decide il colore dell'etichetta).
 */
import type { GameSummary } from '../../../bindings/GameSummary';

/** Il tono dell'etichetta: vinta, persa, patta, in corso o decisa (a due giocatori). */
export type OutcomeTone = 'won' | 'lost' | 'drawn' | 'unfinished' | 'decided';

/** Il testo e il tono dell'esito. */
export interface OutcomeDescription {
  /** Il testo da mostrare. */
  label: string;
  /** Il tono, per il colore. */
  tone: OutcomeTone;
}

export function describeOutcome(game: GameSummary): OutcomeDescription {
  const { result, computerSide } = game;
  if (result === null) {
    return { label: 'In corso', tone: 'unfinished' };
  }
  if (result.winner === null) {
    return { label: 'Patta', tone: 'drawn' };
  }
  if (computerSide !== null) {
    return result.winner === computerSide
      ? { label: 'Persa', tone: 'lost' }
      : { label: 'Vinta', tone: 'won' };
  }
  return {
    label: `Vince il ${result.winner === 'white' ? 'Bianco' : 'Nero'}`,
    tone: 'decided',
  };
}
