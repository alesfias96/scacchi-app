/**
 * ReviewSummary — la scheda «Riepilogo»: com'è andato ciascun giocatore.
 *
 * Cosa fa: per il Bianco e per il Nero mostra la precisione (da 0 a 100),
 * quante imprecisioni, errori ed errori gravi, e la fase di gioco in cui ha
 * perso di più.
 *
 * Riceve: `white`, `black`, i riepiloghi dei due giocatori.
 * Restituisce: la scheda.
 */
import type { PlayerSummary } from '../../../bindings/PlayerSummary';
import { PanelCard } from '../../../components/panel-card/PanelCard';
import { phasePlace } from '../describeQuality';
import styles from './ReviewSummary.module.css';

export interface ReviewSummaryProps {
  white: PlayerSummary;
  black: PlayerSummary;
}

/** Un numero con la parola al singolare o al plurale: "1 errore", "3 errori". */
function count(value: number, one: string, many: string): string {
  return `${String(value)} ${value === 1 ? one : many}`;
}

/** Una riga del riepilogo. */
function PlayerRow({ name, summary }: { name: string; summary: PlayerSummary }) {
  return (
    <div className={styles.player}>
      <p className={styles.name}>
        {name}
        <span className={styles.accuracy}>
          {summary.accuracy === null ? '—' : `${String(summary.accuracy)}%`}
          <span className={styles.label}> precisione</span>
        </span>
      </p>
      <p className={styles.counts}>
        <span className={styles.inaccuracy}>
          {count(summary.inaccuracies, 'imprecisione', 'imprecisioni')} (?!)
        </span>
        <span className={styles.mistake}>{count(summary.mistakes, 'errore', 'errori')} (?)</span>
        <span className={styles.blunder}>
          {count(summary.blunders, 'errore grave', 'errori gravi')} (??)
        </span>
      </p>
      {summary.worstPhase !== null && (
        <p className={styles.phase}>Ha sbagliato di più {phasePlace(summary.worstPhase)}.</p>
      )}
    </div>
  );
}

export function ReviewSummary({ white, black }: ReviewSummaryProps) {
  return (
    <PanelCard title="Riepilogo">
      <PlayerRow name="Bianco" summary={white} />
      <PlayerRow name="Nero" summary={black} />
    </PanelCard>
  );
}
