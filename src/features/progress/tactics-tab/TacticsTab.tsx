/**
 * TacticsTab — la scheda «Calcolo e tattica»: come vanno i puzzle dell'Allenamento.
 *
 * Cosa fa: mostra quanti puzzle si sono risolti (senza contare i ripassi),
 * quanti sono da ripassare oggi, i risultati per numero di mosse da trovare
 * (cioè quanto lontano si riesce a calcolare), la profondità media dei puzzle
 * risolti giorno per giorno e i temi in cui si sbaglia di più.
 *
 * Riceve: `tactics`, le statistiche della scheda.
 * Restituisce: la scheda.
 */
import type { TacticsStats } from '../../../bindings/TacticsStats';
import { BarList } from '../../../components/bar-list/BarList';
import { LineChart } from '../../../components/line-chart/LineChart';
import { PanelCard } from '../../../components/panel-card/PanelCard';
import { countWords, formatPercent, percent } from '../describeStats';
import { StatFigure } from '../stat-figure/StatFigure';
import styles from '../ProgressPage.module.css';

export interface TacticsTabProps {
  tactics: TacticsStats;
}

export function TacticsTab({ tactics }: TacticsTabProps) {
  return (
    <div className={styles.tab}>
      <div className={styles.figures}>
        <StatFigure
          label="Puzzle risolti"
          value={formatPercent(percent(tactics.solved, tactics.attempts))}
          note={`${String(tactics.solved)} su ${countWords(tactics.attempts, 'tentativo', 'tentativi')}.`}
        />
        <StatFigure
          label="Da ripassare oggi"
          value={String(tactics.dueReviews)}
          note="I puzzle sbagliati tornano a distanza di giorni."
        />
      </div>

      <div className={styles.cards}>
        <PanelCard title="Per numero di mosse da trovare">
          {tactics.byDepth.length === 0 ? (
            <p className={styles.note}>Nessun puzzle nel periodo: prova l&apos;Allenamento.</p>
          ) : (
            <BarList
              label="Puzzle risolti per numero di mosse"
              items={tactics.byDepth.map((depth) => ({
                key: String(depth.depth),
                label: countWords(depth.depth, 'mossa', 'mosse'),
                detail: `${formatPercent(percent(depth.solved, depth.attempts))} · ${String(depth.solved)} su ${String(depth.attempts)}`,
                segments: [{ percent: percent(depth.solved, depth.attempts) ?? 0, tone: 'good' }],
              }))}
            />
          )}
        </PanelCard>

        <PanelCard title="Quanto lontano calcoli">
          <LineChart
            label="Mosse dei puzzle risolti, in media"
            points={tactics.depthHistory}
            formatValue={(value) => value.toFixed(1)}
          />
          <p className={styles.note}>
            Quante mosse avevano, in media, i puzzle risolti in ogni giorno di allenamento.
          </p>
        </PanelCard>

        <PanelCard title="Temi da allenare">
          {tactics.weakestThemes.length === 0 ? (
            <p className={styles.note}>
              Compaiono i temi con almeno 3 tentativi, dal più difficile per te.
            </p>
          ) : (
            <BarList
              label="Temi in cui sbagli di più"
              items={tactics.weakestThemes.map((theme) => ({
                key: theme.key,
                label: theme.name,
                detail: `${formatPercent(percent(theme.solved, theme.attempts))} · ${String(theme.solved)} su ${String(theme.attempts)}`,
                segments: [{ percent: percent(theme.solved, theme.attempts) ?? 0, tone: 'good' }],
              }))}
            />
          )}
        </PanelCard>
      </div>
    </div>
  );
}
