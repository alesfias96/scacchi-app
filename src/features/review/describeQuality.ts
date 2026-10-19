/**
 * Le parole e i simboli dell'analisi: giudizio delle mosse e fasi di gioco.
 */
import type { GamePhase } from '../../bindings/GamePhase';
import type { MoveQuality } from '../../bindings/MoveQuality';

/** Il simbolo di una mossa come nei libri: ?! imprecisione, ? errore, ?? errore grave. */
export function qualitySymbol(quality: MoveQuality | null): string {
  switch (quality) {
    case 'inaccuracy':
      return '?!';
    case 'mistake':
      return '?';
    case 'blunder':
      return '??';
    default:
      return '';
  }
}

/** La gravità di una mossa sbagliata (per i colori), o `null` se non è un errore. */
export function qualitySeverity(
  quality: MoveQuality | null,
): 'inaccuracy' | 'mistake' | 'blunder' | null {
  return quality === 'inaccuracy' || quality === 'mistake' || quality === 'blunder'
    ? quality
    : null;
}

/** Dove si è nella partita: "in apertura", "nel mediogioco", "nel finale". */
export function phasePlace(phase: GamePhase): string {
  switch (phase) {
    case 'opening':
      return 'in apertura';
    case 'middlegame':
      return 'nel mediogioco';
    case 'endgame':
      return 'nel finale';
  }
}
