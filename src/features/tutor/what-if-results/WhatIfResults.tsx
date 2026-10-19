/**
 * WhatIfResults — le mosse provate ("e se…") e cosa cambia con ognuna.
 *
 * Cosa fa: per ogni mossa provata mostra la mossa, il giudizio (se c'è
 * Stockfish: «Errore: la posizione peggiora parecchio», con la valutazione
 * prima e dopo), la risposta migliore dell'avversario e cosa cambia: cosa si
 * concede, cosa si crea, quali minacce si risolvono. La più recente è in alto.
 *
 * Riceve:
 * - `scenarios`: le mosse provate, in ordine;
 * - `notation`: la notazione scelta nelle Impostazioni;
 * - `onHighlight`: chiamata con la frase indicata, o `null`.
 * Restituisce: l'elenco delle mosse provate.
 */
import type { Notation } from '../../../bindings/Notation';
import type { TutorItem } from '../../../bindings/TutorItem';
import type { WhatIfView } from '../../../bindings/WhatIfView';
import { formatEvaluation } from '../../../lib/formatEvaluation';
import { formatLine } from '../../../lib/formatLine';
import { formatSan } from '../../../lib/formatSan';
import { TutorSections } from '../tutor-sections/TutorSections';
import styles from './WhatIfResults.module.css';

export interface WhatIfResultsProps {
  scenarios: readonly WhatIfView[];
  notation: Notation;
  onHighlight: (item: TutorItem | null) => void;
}

/** Quante mezze mosse della risposta migliore mostrare. */
const REPLY_LENGTH = 4;

export function WhatIfResults({ scenarios, notation, onHighlight }: WhatIfResultsProps) {
  const newestFirst = [...scenarios].reverse();
  return (
    <ol className={styles.list} aria-label="Mosse provate">
      {newestFirst.map((scenario, index) => {
        const number = scenarios.length - index;
        const { evaluationBefore, evaluationAfter, bestReply } = scenario;
        return (
          <li key={number} className={styles.scenario}>
            <p className={styles.move}>
              {number}. {formatSan(scenario.san, notation)}
              {scenario.verdict !== null && (
                <span className={styles.verdict}> — {scenario.verdict}</span>
              )}
            </p>
            {evaluationBefore !== null && evaluationAfter !== null && (
              <p className={styles.detail}>
                Valutazione: da {formatEvaluation(evaluationBefore)} a{' '}
                {formatEvaluation(evaluationAfter)}
              </p>
            )}
            {bestReply !== null && (
              <p className={styles.detail}>
                La risposta migliore: {formatLine(bestReply, notation, REPLY_LENGTH)}
              </p>
            )}
            {scenario.sections.length === 0 ? (
              <p className={styles.detail}>
                I rilevatori non vedono minacce nuove né tattiche create.
              </p>
            ) : (
              <TutorSections
                sections={scenario.sections}
                notation={notation}
                onHighlight={onHighlight}
              />
            )}
          </li>
        );
      })}
    </ol>
  );
}
