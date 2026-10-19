/**
 * GamesTab — la scheda «Partite»: i risultati e dove si sbaglia.
 *
 * Cosa fa: mostra
 * - i risultati contro il computer, in tutto e con ciascun colore;
 * - i risultati per livello del computer;
 * - la precisione e gli errori in apertura, nel mediogioco e nel finale
 *   (solo partite analizzate);
 * - quanto si sbaglia con poco tempo sull'orologio (meno di un quinto del
 *   tempo iniziale), rispetto al resto della partita.
 *
 * Riceve: `results`, le statistiche della scheda.
 * Restituisce: la scheda.
 */
import type { GameStats } from '../../../bindings/GameStats';
import type { TimePressure } from '../../../bindings/TimePressure';
import { BarList, type BarItem } from '../../../components/bar-list/BarList';
import { PanelCard } from '../../../components/panel-card/PanelCard';
import {
  countWords,
  formatPercent,
  percent,
  phaseName,
  scoreDetail,
  scoreSegments,
  scoreTotal,
  sideName,
} from '../describeStats';
import styles from '../ProgressPage.module.css';

export interface GamesTabProps {
  results: GameStats;
}

/** Le righe dei risultati contro il computer: in tutto, poi con il Bianco e con il Nero. */
function scoreItems(results: GameStats): BarItem[] {
  return [
    {
      key: 'all',
      label: 'In tutto',
      detail: scoreDetail(results.againstComputer),
      segments: scoreSegments(results.againstComputer),
    },
    ...results.byColor.map((color) => ({
      key: color.side,
      label: `Con il ${sideName(color.side)}`,
      detail: scoreDetail(color.score),
      segments: scoreSegments(color.score),
    })),
  ];
}

/** Le righe della pressione del tempo: la percentuale di mosse sbagliate. */
function timeItems(time: TimePressure): BarItem[] {
  const row = (key: string, label: string, errors: number, moves: number): BarItem => ({
    key,
    label,
    detail: `${formatPercent(percent(errors, moves))} · ${String(errors)} su ${countWords(moves, 'mossa', 'mosse')}`,
    segments: [{ percent: percent(errors, moves) ?? 0, tone: 'bad' }],
  });
  return [
    row('low', 'Con poco tempo', time.lowTimeErrors, time.lowTimeMoves),
    row('other', 'Con più tempo', time.otherErrors, time.otherMoves),
  ];
}

export function GamesTab({ results }: GamesTabProps) {
  return (
    <div className={styles.cards}>
      <PanelCard title="Contro il computer">
        {scoreTotal(results.againstComputer) === 0 ? (
          <p className={styles.note}>Nessuna partita finita contro il computer nel periodo.</p>
        ) : (
          <BarList label="Risultati contro il computer" items={scoreItems(results)} />
        )}
        {results.twoPlayerGames > 0 && (
          <p className={styles.note}>
            E {countWords(results.twoPlayerGames, 'partita', 'partite')} a due giocatori.
          </p>
        )}
      </PanelCard>

      <PanelCard title="Per livello del computer">
        {results.byLevel.length === 0 ? (
          <p className={styles.note}>Nessuna partita finita contro il computer nel periodo.</p>
        ) : (
          <BarList
            label="Risultati per livello"
            items={results.byLevel.map((level) => ({
              key: String(level.level),
              label: `${level.name} (${String(level.elo)})`,
              detail: scoreDetail(level.score),
              segments: scoreSegments(level.score),
            }))}
          />
        )}
      </PanelCard>

      <PanelCard title="Fasi della partita">
        {results.phases.length === 0 ? (
          <p className={styles.note}>
            Analizza qualche partita per vedere come giochi in apertura, nel mediogioco e nel
            finale.
          </p>
        ) : (
          <BarList
            label="Precisione per fase"
            items={results.phases.map((phase) => ({
              key: phase.phase,
              label: phaseName(phase.phase),
              detail: `${formatPercent(phase.accuracy)} · ${String(phase.inaccuracies)} ?! · ${String(phase.mistakes)} ? · ${String(phase.blunders)} ??`,
              segments: [{ percent: phase.accuracy ?? 0, tone: 'accent' }],
            }))}
          />
        )}
      </PanelCard>

      <PanelCard title="Con poco tempo">
        {results.timePressure === null ? (
          <p className={styles.note}>
            Compare dopo qualche partita analizzata giocata con l&apos;orologio.
          </p>
        ) : (
          <>
            <BarList label="Mosse sbagliate" items={timeItems(results.timePressure)} />
            <p className={styles.note}>
              Le mosse sbagliate (anche le imprecisioni) quando sull&apos;orologio resta meno di un
              quinto del tempo iniziale, e nel resto della partita.
            </p>
          </>
        )}
      </PanelCard>
    </div>
  );
}
