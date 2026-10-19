/**
 * Le parole e le barre comuni alle schede di «Progressi»: risultati,
 * percentuali, colori e fasi di gioco.
 */
import type { GamePhase } from '../../bindings/GamePhase';
import type { Score } from '../../bindings/Score';
import type { Side } from '../../bindings/Side';
import type { BarSegment } from '../../components/bar-list/BarList';

/** Quante partite in tutto (vinte, patte e perse). */
export function scoreTotal(score: Score): number {
  return score.wins + score.draws + score.losses;
}

/** La percentuale (0–100, arrotondata) di `part` su `whole`; `null` se `whole` è zero. */
export function percent(part: number, whole: number): number | null {
  return whole === 0 ? null : Math.round((part / whole) * 100);
}

/** Una percentuale scritta: "72%", oppure "—" se manca. */
export function formatPercent(value: number | null): string {
  return value === null ? '—' : `${String(value)}%`;
}

/** Un numero con la parola al singolare o al plurale: "1 partita", "3 partite". */
export function countWords(value: number, one: string, many: string): string {
  return `${String(value)} ${value === 1 ? one : many}`;
}

/** I risultati in parole: "3 vinte · 1 patta · 2 perse". */
export function scoreDetail(score: Score): string {
  return [
    countWords(score.wins, 'vinta', 'vinte'),
    countWords(score.draws, 'patta', 'patte'),
    countWords(score.losses, 'persa', 'perse'),
  ].join(' · ');
}

/** I tratti della barra dei risultati: vittorie (verde), patte (grigio), sconfitte (rosso). */
export function scoreSegments(score: Score): BarSegment[] {
  const total = scoreTotal(score);
  if (total === 0) {
    return [];
  }
  return [
    { percent: (score.wins / total) * 100, tone: 'good' },
    { percent: (score.draws / total) * 100, tone: 'neutral' },
    { percent: (score.losses / total) * 100, tone: 'bad' },
  ];
}

/** Il nome del colore: "Bianco" o "Nero". */
export function sideName(side: Side): string {
  return side === 'white' ? 'Bianco' : 'Nero';
}

/** Il nome della fase di gioco: "Apertura", "Mediogioco", "Finale". */
export function phaseName(phase: GamePhase): string {
  switch (phase) {
    case 'opening':
      return 'Apertura';
    case 'middlegame':
      return 'Mediogioco';
    case 'endgame':
      return 'Finale';
  }
}
