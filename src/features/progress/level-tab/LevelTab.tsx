/**
 * LevelTab — la scheda «Livello»: a che livello si gioca e come cambia nel tempo.
 *
 * Cosa fa: mostra tre numeri (l'Elo stimato, il rating dei puzzle, la
 * precisione media nelle partite analizzate) e due grafici: il rating dei
 * puzzle dopo ogni tentativo e la precisione di ogni partita analizzata.
 * L'Elo stimato compare con almeno 5 partite finite contro il computer.
 *
 * Riceve: `level`, le statistiche della scheda.
 * Restituisce: la scheda.
 */
import type { LevelStats } from '../../../bindings/LevelStats';
import { LineChart } from '../../../components/line-chart/LineChart';
import { PanelCard } from '../../../components/panel-card/PanelCard';
import { countWords, formatPercent } from '../describeStats';
import { StatFigure } from '../stat-figure/StatFigure';
import styles from '../ProgressPage.module.css';

export interface LevelTabProps {
  level: LevelStats;
}

/** Quante partite contro il computer servono per stimare l'Elo (come in crates/stats). */
const ELO_MIN_GAMES = 5;

export function LevelTab({ level }: LevelTabProps) {
  return (
    <div className={styles.tab}>
      <div className={styles.figures}>
        <StatFigure
          label="Elo stimato"
          value={level.estimatedElo === null ? '—' : String(level.estimatedElo)}
          note={
            level.estimatedElo === null
              ? `Servono almeno ${String(ELO_MIN_GAMES)} partite finite contro il computer (finora ${String(level.eloGames)}).`
              : `Dai risultati di ${countWords(level.eloGames, 'partita', 'partite')} contro il computer.`
          }
        />
        <StatFigure
          label="Rating dei puzzle"
          value={String(level.puzzleRating)}
          note="Dall'Allenamento."
        />
        <StatFigure
          label="Precisione media"
          value={formatPercent(level.accuracy)}
          note="Nelle partite analizzate."
        />
      </div>

      <p className={styles.note}>
        L&apos;Elo stimato è indicativo: confronta i tuoi risultati con l&apos;Elo dei livelli del
        computer contro cui hai giocato. Più partite giochi, più la stima è affidabile.
      </p>

      <div className={styles.cards}>
        <PanelCard title="Rating dei puzzle nel tempo">
          <LineChart label="Rating dei puzzle" points={level.puzzleHistory} />
        </PanelCard>
        <PanelCard title="Precisione partita per partita">
          <LineChart
            label="Precisione"
            points={level.accuracyHistory}
            formatValue={(value) => formatPercent(Math.round(value))}
          />
        </PanelCard>
      </div>
    </div>
  );
}
