/**
 * ProgressFilters — i filtri delle statistiche: il periodo e l'avversario.
 *
 * Cosa fa: due gruppi di scelte. Le scelte si salvano nelle impostazioni,
 * così alla prossima apertura i filtri sono quelli lasciati.
 *
 * Riceve:
 * - `filter`: i filtri attuali;
 * - `onChange`: chiamata con i filtri nuovi.
 * Restituisce: i filtri.
 */
import type { StatsFilter } from '../../../bindings/StatsFilter';
import type { StatsOpponent } from '../../../bindings/StatsOpponent';
import type { StatsPeriod } from '../../../bindings/StatsPeriod';
import { ChoiceGroup, type Choice } from '../../../components/choice-group/ChoiceGroup';
import styles from './ProgressFilters.module.css';

export interface ProgressFiltersProps {
  filter: StatsFilter;
  onChange: (filter: StatsFilter) => void;
}

const PERIODS: readonly Choice<StatsPeriod>[] = [
  { value: 'month', label: 'Ultimi 30 giorni' },
  { value: 'quarter', label: 'Ultimi 90 giorni' },
  { value: 'all', label: 'Sempre' },
];

const OPPONENTS: readonly Choice<StatsOpponent>[] = [
  { value: 'all', label: 'Tutte le partite' },
  { value: 'computer', label: 'Contro il computer' },
  { value: 'twoPlayers', label: 'A due giocatori' },
];

export function ProgressFilters({ filter, onChange }: ProgressFiltersProps) {
  return (
    <div className={styles.filters}>
      <ChoiceGroup
        legend="Periodo"
        name="stats-period"
        options={PERIODS}
        value={filter.period}
        onChange={(period) => {
          onChange({ ...filter, period });
        }}
      />
      <ChoiceGroup
        legend="Partite"
        name="stats-opponent"
        options={OPPONENTS}
        value={filter.opponent}
        onChange={(opponent) => {
          onChange({ ...filter, opponent });
        }}
      />
    </div>
  );
}
