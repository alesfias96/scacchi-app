/**
 * CoachPlanPanel — «Il piano» della modalità Tutor, nella colonna di destra.
 *
 * Cosa fa: mostra il piano consigliato a chi muove (uno o due, vedi
 * `StrategyPlan`) e quello dell'avversario. Il piano di solito resta lo
 * stesso per molte mosse: quando invece cambia rispetto alla posizione
 * precedente compare l'etichetta «Piano nuovo», così si nota subito che la
 * partita ha preso un'altra strada. Mentre il tutor prepara la posizione
 * nuova si continua a vedere il piano di prima.
 *
 * Riceve:
 * - `strategy`: la strategia della posizione, o `null` se non è ancora pronta;
 * - `confirmedPlans`: i piani che Stockfish conferma guardando avanti.
 * Restituisce: la scheda (niente, finché non c'è un piano).
 */
import { useState } from 'react';
import type { StrategyView } from '../../../bindings/StrategyView';
import { PanelCard } from '../../../components/panel-card/PanelCard';
import { StrategyPlan } from '../../tutor/strategy-section/StrategyPlan';
import styles from './CoachPlanPanel.module.css';

export interface CoachPlanPanelProps {
  strategy: StrategyView | null;
  confirmedPlans: readonly string[];
}

/** Ciò che si ricorda tra una posizione e l'altra. */
interface Shown {
  strategy: StrategyView | null;
  keys: string;
  isNew: boolean;
}

/** I piani, in una stringa che si confronta facilmente. */
function planKeys(strategy: StrategyView): string {
  return strategy.plans.map((plan) => plan.key).join('|');
}

export function CoachPlanPanel({ strategy, confirmedPlans }: CoachPlanPanelProps) {
  const [shown, setShown] = useState<Shown>(() => ({
    strategy,
    keys: strategy === null ? '' : planKeys(strategy),
    isNew: false,
  }));
  // Arriva una posizione nuova: si confrontano i piani con quelli di prima
  // (un aggiornamento durante il disegno, come suggerisce React).
  if (strategy !== null && strategy !== shown.strategy) {
    const keys = planKeys(strategy);
    setShown({ strategy, keys, isNew: shown.keys !== '' && keys !== shown.keys });
  }

  const current = shown.strategy;
  if (current === null || (current.plans.length === 0 && current.opponentPlan === null)) {
    return null;
  }

  return (
    <PanelCard title="Il piano">
      {shown.isNew && <span className={styles.badge}>Piano nuovo</span>}
      {current.plans.map((plan) => (
        <StrategyPlan key={plan.key} plan={plan} confirmed={confirmedPlans.includes(plan.key)} />
      ))}
      {current.opponentPlan !== null && (
        <StrategyPlan
          plan={current.opponentPlan}
          confirmed={false}
          label="Il piano dell’avversario"
        />
      )}
    </PanelCard>
  );
}
