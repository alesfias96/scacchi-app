/**
 * StrategyPlan — un piano strategico consigliato dal tutor.
 *
 * Cosa fa: mostra per chi è il piano, il titolo e **perché** vale proprio in
 * questa posizione; se Stockfish, guardando avanti, segue davvero questo
 * piano, lo dice («✓ Stockfish conferma»). Aprendo «Come si fa, quando sì e
 * quando no» si leggono i passi tipici, quando conviene, quando no e i
 * segnali che sta funzionando.
 *
 * Riceve:
 * - `plan`: il piano (lo prepara Rust);
 * - `confirmed`: `true` se la variante migliore di Stockfish lo conferma;
 * - `label`: cosa scrivere sopra il titolo (es. «Il piano dell'avversario»);
 *   se manca, «Per il Bianco» o «Per il Nero».
 * Restituisce: il piano.
 */
import type { StrategyPlanView } from '../../../bindings/StrategyPlanView';
import styles from './StrategyPlan.module.css';

export interface StrategyPlanProps {
  plan: StrategyPlanView;
  confirmed: boolean;
  label?: string;
}

interface PlanListProps {
  title: string;
  items: readonly string[];
  ordered?: boolean;
}

/** Un elenco con il suo titolo (niente, se l'elenco è vuoto). */
function PlanList({ title, items, ordered = false }: PlanListProps) {
  if (items.length === 0) {
    return null;
  }
  const entries = items.map((item) => <li key={item}>{item}</li>);
  return (
    <>
      <h5 className={styles.heading}>{title}</h5>
      {ordered ? (
        <ol className={styles.list}>{entries}</ol>
      ) : (
        <ul className={styles.list}>{entries}</ul>
      )}
    </>
  );
}

export function StrategyPlan({ plan, confirmed, label }: StrategyPlanProps) {
  return (
    <article className={styles.plan} aria-label={plan.title}>
      <div className={styles.header}>
        <span className={styles.side}>
          {label ?? `Per il ${plan.side === 'white' ? 'Bianco' : 'Nero'}`}
        </span>
        {confirmed && <span className={styles.confirmed}>✓ Stockfish conferma</span>}
      </div>
      <h4 className={styles.title}>{plan.title}</h4>
      <p className={styles.why}>
        <strong>Perché qui: </strong>
        {plan.why}
      </p>
      <details className={styles.more}>
        <summary className={styles.summary}>Come si fa, quando sì e quando no</summary>
        <div className={styles.body}>
          <PlanList title="Come si fa" items={plan.how} ordered />
          <PlanList title="Quando usarlo" items={plan.useWhen} />
          <PlanList title="Quando no" items={plan.avoidWhen} />
          <PlanList title="Segnali che funziona" items={plan.signs} />
        </div>
      </details>
    </article>
  );
}
