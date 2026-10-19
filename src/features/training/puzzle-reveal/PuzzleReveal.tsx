/**
 * PuzzleReveal — la scheda "Soluzione", a puzzle finito.
 *
 * Cosa fa: mostra il risultato per il rating (es. "1516 (+16)"), la difficoltà
 * del puzzle, i suoi temi, la soluzione completa e il perché funziona: i fatti
 * tattici trovati dai rilevatori nella posizione del puzzle (forchette,
 * inchiodature, pezzi in presa…). Passando il mouse su un fatto, la scacchiera
 * mostra le case coinvolte. Sotto, «La combinazione»: la soluzione raccontata
 * dal tutor con i suoi motivi uno dopo l'altro, le schede dei motivi e
 * «Allenati» per altri puzzle dello stesso tema.
 *
 * Riceve:
 * - `reveal`: ciò che si scopre a puzzle finito;
 * - `rating` e `ratingChange`: il rating dopo il puzzle e quanto è cambiato
 *   (`null` nei ripassi, che non cambiano il rating);
 * - `notation`: la notazione scelta nelle Impostazioni;
 * - `onHighlight`: chiamata con il fatto indicato dal mouse, o `null`.
 * Restituisce: la scheda.
 */
import type { Fact } from '../../../bindings/Fact';
import type { Notation } from '../../../bindings/Notation';
import type { Reveal } from '../../../bindings/Reveal';
import { useState } from 'react';
import { KnowledgeCards } from '../../../components/knowledge-cards/KnowledgeCards';
import { PanelCard } from '../../../components/panel-card/PanelCard';
import { classNames } from '../../../lib/classNames';
import { formatFact } from '../../../lib/formatFact';
import { formatLine } from '../../../lib/formatLine';
import { TrainThemeButton } from '../train-theme-button/TrainThemeButton';
import { useTrainTheme } from '../useTrainTheme';
import styles from './PuzzleReveal.module.css';

export interface PuzzleRevealProps {
  reveal: Reveal;
  rating: number;
  ratingChange: number | null;
  notation: Notation;
  onHighlight: (fact: Fact | null) => void;
}

/** Quante mezze mosse della soluzione mostrare al massimo (le soluzioni sono brevi). */
const MAX_SOLUTION_MOVES = 12;

/** Scrive la variazione del rating con il segno: "+16", "−8", "±0". */
function formatChange(change: number): string {
  if (change > 0) {
    return `+${String(change)}`;
  }
  return change < 0 ? `−${String(-change)}` : '±0';
}

export function PuzzleReveal({
  reveal,
  rating,
  ratingChange,
  notation,
  onHighlight,
}: PuzzleRevealProps) {
  const trainTheme = useTrainTheme();
  /** La scheda di un motivo aperta. */
  const [openCardId, setOpenCardId] = useState<string | null>(null);
  const { idea } = reveal;

  return (
    <PanelCard title="Soluzione">
      <dl className={styles.details}>
        <dt>Il tuo rating</dt>
        <dd>
          {rating}
          {ratingChange === null ? (
            ' (ripasso: non cambia)'
          ) : (
            <span
              className={classNames(
                styles.change,
                ratingChange > 0 && styles.up,
                ratingChange < 0 && styles.down,
              )}
            >
              {' '}
              ({formatChange(ratingChange)})
            </span>
          )}
        </dd>
        <dt>Difficoltà del puzzle</dt>
        <dd>{reveal.puzzleRating}</dd>
        <dt>Temi</dt>
        <dd>{reveal.themes.length > 0 ? reveal.themes.join(', ') : '—'}</dd>
        <dt>Soluzione</dt>
        <dd className={styles.solution}>
          {formatLine(reveal.solution, notation, MAX_SOLUTION_MOVES)}
        </dd>
      </dl>

      <section className={styles.why} aria-label="Perché funziona">
        <h3 className={styles.whyTitle}>Perché funziona</h3>
        {reveal.facts.length === 0 ? (
          <p className={styles.empty}>
            I rilevatori non trovano tattiche evidenti nella posizione di partenza: l’idea si vede
            mossa dopo mossa, seguendo la soluzione.
          </p>
        ) : (
          <ul className={styles.list}>
            {reveal.facts.map((fact) => (
              <li key={fact.text}>
                <button
                  type="button"
                  className={styles.fact}
                  onMouseEnter={() => {
                    onHighlight(fact);
                  }}
                  onMouseLeave={() => {
                    onHighlight(null);
                  }}
                  onFocus={() => {
                    onHighlight(fact);
                  }}
                  onBlur={() => {
                    onHighlight(null);
                  }}
                >
                  {formatFact(fact, notation)}
                </button>
              </li>
            ))}
          </ul>
        )}
      </section>

      {idea !== null && (
        <section className={styles.why} aria-label="La combinazione">
          <h3 className={styles.whyTitle}>La combinazione</h3>
          <p>{idea.headline}</p>
          {idea.motifs.length > 0 && (
            <div className={styles.motifs}>
              {idea.motifs.map((motif) =>
                motif.puzzleTheme === null ? null : (
                  <TrainThemeButton key={motif.key} theme={motif.puzzleTheme} name={motif.name} />
                ),
              )}
            </div>
          )}
          {idea.cards.length > 0 && (
            <KnowledgeCards
              cards={idea.cards}
              openCardId={openCardId}
              onToggle={setOpenCardId}
              onTrainTheme={trainTheme}
            />
          )}
        </section>
      )}
    </PanelCard>
  );
}
