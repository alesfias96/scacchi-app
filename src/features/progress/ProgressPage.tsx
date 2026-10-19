/**
 * ProgressPage — la sezione «Progressi»: le statistiche su partite e allenamento.
 *
 * Cosa fa: in cima i filtri (periodo e partite), salvati nelle impostazioni;
 * poi su quante partite si basano le statistiche, con il pulsante «Analizza
 * le partite mancanti»; infine quattro schede:
 * - «Livello»: Elo stimato, rating dei puzzle e precisione nel tempo;
 * - «Partite»: risultati per colore e livello, errori per fase e con poco tempo;
 * - «Aperture»: le più giocate con il Bianco e con il Nero;
 * - «Calcolo e tattica»: puzzle per numero di mosse e per tema, ripassi.
 * Tutti i calcoli li fa Rust (crates/stats): qui si mostrano soltanto.
 * Quando finisce l'analisi di una partita, le statistiche si rileggono.
 *
 * Riceve: niente.
 * Restituisce: la pagina della sezione.
 */
import { useEffect, useState } from 'react';
import type { StatsFilter } from '../../bindings/StatsFilter';
import { TabSwitch, type TabOption } from '../../components/tab-switch/TabSwitch';
import { useReviewStore } from '../../state/reviewStore';
import { useLoadedSettings, useSettingsStore } from '../../state/settingsStore';
import { AnalysisQueue } from './analysis-queue/AnalysisQueue';
import { GamesTab } from './games-tab/GamesTab';
import { LevelTab } from './level-tab/LevelTab';
import { OpeningsTab } from './openings-tab/OpeningsTab';
import { ProgressFilters } from './progress-filters/ProgressFilters';
import styles from './ProgressPage.module.css';
import { TacticsTab } from './tactics-tab/TacticsTab';
import { useStats } from './useStats';

/** Le quattro schede della sezione. */
type ProgressTab = 'level' | 'games' | 'openings' | 'tactics';

const TABS: readonly TabOption<ProgressTab>[] = [
  { value: 'level', label: 'Livello' },
  { value: 'games', label: 'Partite' },
  { value: 'openings', label: 'Aperture' },
  { value: 'tactics', label: 'Calcolo e tattica' },
];

export function ProgressPage() {
  const { statsFilter } = useLoadedSettings();
  const updateSettings = useSettingsStore((state) => state.update);
  const listenToReviews = useReviewStore((state) => state.listen);
  const lastFinished = useReviewStore((state) => state.lastFinished);
  const [tab, setTab] = useState<ProgressTab>('level');
  const { report, error, reload } = useStats(statsFilter);

  // Gli eventi dell'analisi servono per sapere se la coda è già in corso.
  useEffect(() => {
    void listenToReviews();
  }, [listenToReviews]);

  // Finita l'analisi di una partita, le statistiche cambiano: si rileggono.
  useEffect(() => {
    if (lastFinished !== null) {
      reload();
    }
  }, [lastFinished, reload]);

  const changeFilter = (filter: StatsFilter) => {
    void updateSettings({ statsFilter: filter });
  };

  return (
    <section className={styles.page}>
      <header className={styles.header}>
        <h1 className={styles.title}>Progressi</h1>
        <p className={styles.subtitle}>
          Come stai giocando e come cambia nel tempo, dalle tue partite e dall&apos;Allenamento.
        </p>
      </header>

      <ProgressFilters filter={statsFilter} onChange={changeFilter} />

      {error !== null && (
        <p className={styles.error} role="alert">
          {error}
        </p>
      )}

      {report !== null && (
        <>
          <AnalysisQueue
            games={report.games}
            analyzed={report.analyzedGames}
            missing={report.unanalyzedGames}
          />

          <TabSwitch label="Statistiche" options={TABS} value={tab} onChange={setTab} />

          {tab === 'level' && <LevelTab level={report.level} />}
          {tab === 'games' && <GamesTab results={report.results} />}
          {tab === 'openings' && <OpeningsTab openings={report.openings} />}
          {tab === 'tactics' && <TacticsTab tactics={report.tactics} />}
        </>
      )}
    </section>
  );
}
