/**
 * Il messaggio sotto la scacchiera durante un puzzle.
 *
 * @param attempt - il puzzle in corso, come lo descrive Rust.
 * @param memorizing - `true` nella visualizzazione, finché i pezzi sono ancora visibili.
 * @returns il testo e il "tono" (neutro, positivo, negativo) per colorarlo.
 */
import type { AttemptView } from '../../../bindings/AttemptView';

/** Il tono del messaggio. */
export type FeedbackTone = 'neutral' | 'good' | 'bad';

/** Il messaggio da mostrare. */
export interface FeedbackMessage {
  text: string;
  tone: FeedbackTone;
}

/** Il nome italiano del colore. */
function sideName(side: AttemptView['solver']): string {
  return side === 'white' ? 'il Bianco' : 'il Nero';
}

/** La consegna all'inizio del puzzle, diversa per ogni modalità. */
function instructions(attempt: AttemptView, memorizing: boolean): string {
  if (attempt.mode === 'threat') {
    return `Cosa minaccia l'avversario? Trova la mossa più forte per ${sideName(attempt.solver)}.`;
  }
  if (memorizing) {
    return 'Memorizza la posizione: tra poco i pezzi spariscono.';
  }
  const moves = attempt.depth === 1 ? '1 mossa' : `${String(attempt.depth)} mosse`;
  return `Muove ${sideName(attempt.solver)}: trova la mossa migliore (${moves} da trovare).`;
}

export function describeFeedback(attempt: AttemptView, memorizing: boolean): FeedbackMessage {
  switch (attempt.feedback) {
    case 'ready':
      return { text: instructions(attempt, memorizing), tone: 'neutral' };
    case 'correct':
      return { text: 'Giusto! Continua: l’avversario ha risposto.', tone: 'good' };
    case 'wrong':
      return { text: 'Non è la mossa giusta: riprova.', tone: 'bad' };
    case 'solved':
      return attempt.clean
        ? { text: 'Risolto!', tone: 'good' }
        : {
            text: 'Risolto, ma dopo un errore o un aiuto: il puzzle conta come sbagliato.',
            tone: 'neutral',
          };
    case 'solutionShown':
      return { text: 'Ecco la soluzione.', tone: 'neutral' };
  }
}
