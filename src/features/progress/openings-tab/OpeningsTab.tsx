/**
 * OpeningsTab — la scheda «Aperture»: le aperture più giocate, con il Bianco e con il Nero.
 *
 * Cosa fa: per ogni colore elenca le aperture (la famiglia, es. "Sicilian
 * Defense"), dalla più giocata, con quante partite, i risultati contro il
 * computer e la precisione media nelle partite analizzate.
 *
 * Riceve: `openings`, le aperture giocate (già ordinate da Rust).
 * Restituisce: la scheda.
 */
import type { OpeningStats } from '../../../bindings/OpeningStats';
import type { Side } from '../../../bindings/Side';
import { BarList } from '../../../components/bar-list/BarList';
import { PanelCard } from '../../../components/panel-card/PanelCard';
import {
  countWords,
  formatPercent,
  scoreDetail,
  scoreSegments,
  scoreTotal,
  sideName,
} from '../describeStats';
import styles from '../ProgressPage.module.css';

export interface OpeningsTabProps {
  openings: readonly OpeningStats[];
}

const SIDES: readonly Side[] = ['white', 'black'];

/** Il dettaglio di un'apertura: partite, risultati (se contro il computer) e precisione. */
function openingDetail(opening: OpeningStats): string {
  const parts = [countWords(opening.games, 'partita', 'partite')];
  if (scoreTotal(opening.score) > 0) {
    parts.push(scoreDetail(opening.score));
  }
  if (opening.accuracy !== null) {
    parts.push(`precisione ${formatPercent(opening.accuracy)}`);
  }
  return parts.join(' · ');
}

export function OpeningsTab({ openings }: OpeningsTabProps) {
  return (
    <div className={styles.tab}>
      <div className={styles.cards}>
        {SIDES.map((side) => {
          const played = openings.filter((opening) => opening.side === side);
          return (
            <PanelCard key={side} title={`Con il ${sideName(side)}`}>
              {played.length === 0 ? (
                <p className={styles.note}>Nessuna apertura riconosciuta nel periodo.</p>
              ) : (
                <BarList
                  label={`Aperture con il ${sideName(side)}`}
                  items={played.map((opening) => ({
                    key: opening.name,
                    label: opening.name,
                    detail: openingDetail(opening),
                    segments: scoreSegments(opening.score),
                  }))}
                />
              )}
            </PanelCard>
          );
        })}
      </div>
      <p className={styles.note}>
        I nomi sono quelli dell&apos;elenco delle aperture di Lichess. La barra mostra vittorie,
        patte e sconfitte contro il computer.
      </p>
    </div>
  );
}
