/**
 * ArchiveFilters — i filtri dell'archivio delle partite.
 *
 * Cosa fa: permette di vedere solo alcune partite.
 * - **Avversario**: tutte, solo contro il computer, solo a due giocatori.
 * - **Esito**: tutte, vinte, perse, patte, in corso ("vinte" e "perse"
 *   riguardano le partite contro il computer).
 * - **Cerca**: un testo da trovare nel nome dell'apertura (in inglese, come
 *   nell'elenco di Lichess: ad esempio "Sicilian") o nel suo codice ECO ("B90").
 *
 * Riceve:
 * - `filter`: i filtri attuali;
 * - `onChange`: chiamata con i filtri nuovi a ogni cambiamento.
 * Restituisce: i filtri.
 */
import type { GameFilter } from '../../../bindings/GameFilter';
import type { OpponentKind } from '../../../bindings/OpponentKind';
import type { OutcomeFilter } from '../../../bindings/OutcomeFilter';
import { ChoiceGroup, type Choice } from '../../../components/choice-group/ChoiceGroup';
import styles from './ArchiveFilters.module.css';

/** Il valore della scelta "tutte" (nessun filtro). */
const ALL = 'all';

/** Le scelte dell'avversario. */
const OPPONENT_CHOICES: readonly Choice<OpponentKind | typeof ALL>[] = [
  { value: ALL, label: 'Tutte' },
  { value: 'computer', label: 'Computer' },
  { value: 'twoPlayers', label: 'Due giocatori' },
];

/** Le scelte dell'esito. */
const OUTCOME_CHOICES: readonly Choice<OutcomeFilter | typeof ALL>[] = [
  { value: ALL, label: 'Tutte' },
  { value: 'won', label: 'Vinte' },
  { value: 'lost', label: 'Perse' },
  { value: 'drawn', label: 'Patte' },
  { value: 'unfinished', label: 'In corso' },
];

export interface ArchiveFiltersProps {
  /** I filtri attuali. */
  filter: GameFilter;
  /** Chiamata con i filtri nuovi. */
  onChange: (filter: GameFilter) => void;
}

export function ArchiveFilters({ filter, onChange }: ArchiveFiltersProps) {
  return (
    <div className={styles.filters}>
      <ChoiceGroup
        legend="Avversario"
        name="archive-opponent"
        options={OPPONENT_CHOICES}
        value={filter.opponent ?? ALL}
        onChange={(value) => {
          onChange({ ...filter, opponent: value === ALL ? null : value });
        }}
      />
      <ChoiceGroup
        legend="Esito"
        name="archive-outcome"
        options={OUTCOME_CHOICES}
        value={filter.outcome ?? ALL}
        onChange={(value) => {
          onChange({ ...filter, outcome: value === ALL ? null : value });
        }}
      />
      <label className={styles.search}>
        <span className={styles.label}>Cerca un'apertura</span>
        <input
          className={styles.input}
          type="search"
          placeholder="es. Sicilian, Italian, B90"
          value={filter.text ?? ''}
          onChange={(event) => {
            const text = event.target.value;
            onChange({ ...filter, text: text === '' ? null : text });
          }}
        />
      </label>
    </div>
  );
}
