/**
 * TrainingSetupPanel — la scheda "Allenamento": come allenarsi.
 *
 * Cosa fa: fa scegliere la modalità (profondità, visualizzazione, "cosa
 * minaccia?", ripasso), quante mosse trovare, il tema dei puzzle, la
 * difficoltà e, nella visualizzazione, per quanti secondi si vede la
 * posizione. Il pulsante "Nuovo puzzle" comincia un puzzle con queste scelte.
 * Le scelte si ricordano (sono salvate con le impostazioni).
 *
 * Riceve:
 * - `setup`: le scelte attuali;
 * - `themes`: i temi disponibili, con quanti puzzle ha ciascuno;
 * - `dueReviews`: quanti puzzle sono da ripassare oggi;
 * - `onChange`: chiamata con le nuove scelte;
 * - `onStart`: comincia un nuovo puzzle.
 * Restituisce: la scheda.
 */
import type { Difficulty } from '../../../bindings/Difficulty';
import type { ThemeInfo } from '../../../bindings/ThemeInfo';
import type { TrainingMode } from '../../../bindings/TrainingMode';
import type { TrainingSetup } from '../../../bindings/TrainingSetup';
import { Button } from '../../../components/button/Button';
import { ChoiceGroup, type Choice } from '../../../components/choice-group/ChoiceGroup';
import { PanelCard } from '../../../components/panel-card/PanelCard';
import styles from './TrainingSetupPanel.module.css';

export interface TrainingSetupPanelProps {
  setup: TrainingSetup;
  themes: readonly ThemeInfo[];
  dueReviews: number;
  onChange: (setup: TrainingSetup) => void;
  onStart: () => void;
}

/** Le profondità possibili: da 1 a 6 mosse da trovare. */
const DEPTH_CHOICES: readonly Choice<string>[] = ['1', '2', '3', '4', '5', '6'].map((depth) => ({
  value: depth,
  label: depth,
}));

const DIFFICULTY_CHOICES: readonly Choice<Difficulty>[] = [
  { value: 'easier', label: 'Più facile' },
  { value: 'normal', label: 'Normale' },
  { value: 'harder', label: 'Più difficile' },
];

/** I limiti dei secondi della visualizzazione. */
const MIN_SECONDS = 2;
const MAX_SECONDS = 15;

/** Il valore della scelta "tutti i temi" nella tendina. */
const ALL_THEMES = '';

export function TrainingSetupPanel({
  setup,
  themes,
  dueReviews,
  onChange,
  onStart,
}: TrainingSetupPanelProps) {
  const modeChoices: readonly Choice<TrainingMode>[] = [
    { value: 'depth', label: 'Calcolo', detail: 'Trova tutte le mosse' },
    { value: 'visualization', label: 'Visualizzazione', detail: 'I pezzi spariscono' },
    { value: 'threat', label: 'Cosa minaccia?', detail: 'La mossa dell’avversario' },
    { value: 'review', label: 'Ripasso', detail: `${String(dueReviews)} da ripassare` },
  ];
  /** La profondità conta solo dove si gioca tutta la soluzione. */
  const usesDepth = setup.mode === 'depth' || setup.mode === 'visualization';
  const isReviewEmpty = setup.mode === 'review' && dueReviews === 0;

  return (
    <PanelCard title="Allenamento">
      <ChoiceGroup
        legend="Modalità"
        name="training-mode"
        options={modeChoices}
        value={setup.mode}
        onChange={(mode) => {
          onChange({ ...setup, mode });
        }}
      />

      {usesDepth && (
        <ChoiceGroup
          legend="Mosse da trovare"
          name="training-depth"
          options={DEPTH_CHOICES}
          value={String(setup.depth)}
          onChange={(depth) => {
            onChange({ ...setup, depth: Number(depth) });
          }}
        />
      )}

      {setup.mode === 'visualization' && (
        <label className={styles.field}>
          <span className={styles.label}>Secondi per memorizzare la posizione</span>
          <input
            className={styles.input}
            type="number"
            min={MIN_SECONDS}
            max={MAX_SECONDS}
            value={setup.visualizationSeconds}
            onChange={(event) => {
              const seconds = Math.round(Number(event.target.value));
              if (seconds >= MIN_SECONDS && seconds <= MAX_SECONDS) {
                onChange({ ...setup, visualizationSeconds: seconds });
              }
            }}
          />
        </label>
      )}

      <label className={styles.field}>
        <span className={styles.label}>Tema</span>
        <select
          className={styles.input}
          value={setup.theme ?? ALL_THEMES}
          onChange={(event) => {
            const theme = event.target.value;
            onChange({ ...setup, theme: theme === ALL_THEMES ? null : theme });
          }}
        >
          <option value={ALL_THEMES}>Tutti i temi</option>
          {themes.map((theme) => (
            <option key={theme.key} value={theme.key}>
              {theme.name} ({theme.count})
            </option>
          ))}
        </select>
      </label>

      {setup.mode !== 'review' && (
        <ChoiceGroup
          legend="Difficoltà"
          name="training-difficulty"
          options={DIFFICULTY_CHOICES}
          value={setup.difficulty}
          onChange={(difficulty) => {
            onChange({ ...setup, difficulty });
          }}
        />
      )}

      <Button variant="primary" onClick={onStart} disabled={isReviewEmpty}>
        {setup.mode === 'review' ? 'Ripassa' : 'Nuovo puzzle'}
      </Button>
      {isReviewEmpty && (
        <p className={styles.hint}>Oggi non ci sono puzzle da ripassare: torna domani.</p>
      )}
    </PanelCard>
  );
}
