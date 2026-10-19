/**
 * StrategySection — «La strategia»: in cima alla spiegazione del tutor.
 *
 * Cosa fa: dice com'è il centro e per chi sono i piani, poi mostra il
 * bilancio «Chi ha cosa» (le voci del Bianco e quelle del Nero: materiale,
 * pedoni, spazio, alfieri, re…), i piani consigliati a chi muove (vedi
 * `StrategyPlan`) e il piano dell'avversario, da tenere d'occhio.
 *
 * Riceve:
 * - `strategy`: la strategia della posizione (la prepara Rust);
 * - `confirmedPlans`: i piani che la variante migliore di Stockfish segue
 *   davvero (dopo «Guarda avanti»; vuoto se non si è guardato avanti).
 * Restituisce: la sezione.
 */
import type { Side } from '../../../bindings/Side';
import type { StrategyView } from '../../../bindings/StrategyView';
import styles from './StrategySection.module.css';
import { StrategyPlan } from './StrategyPlan';

export interface StrategySectionProps {
  strategy: StrategyView;
  confirmedPlans: readonly string[];
}

/** I due lati, con il loro nome. */
const SIDES: readonly { side: Side; name: string }[] = [
  { side: 'white', name: 'Il Bianco' },
  { side: 'black', name: 'Il Nero' },
];

export function StrategySection({ strategy, confirmedPlans }: StrategySectionProps) {
  const { center, imbalances, plans, opponentPlan } = strategy;

  return (
    <section className={styles.section} aria-label="La strategia">
      <h3 className={styles.title}>La strategia</h3>
      <p className={styles.center}>{center}</p>

      <h4 className={styles.subtitle}>Chi ha cosa</h4>
      {imbalances.length === 0 ? (
        <p className={styles.message}>
          Nessuno squilibrio importante: la posizione è in equilibrio.
        </p>
      ) : (
        <div className={styles.balance}>
          {SIDES.map(({ side, name }) => {
            const items = imbalances.filter((item) => item.side === side);
            return (
              <div key={side} className={styles.column}>
                <span className={styles.sideName}>{name}</span>
                {items.length === 0 ? (
                  <p className={styles.message}>niente di particolare</p>
                ) : (
                  <ul className={styles.list}>
                    {items.map((item) => (
                      <li key={item.text}>{item.text}</li>
                    ))}
                  </ul>
                )}
              </div>
            );
          })}
        </div>
      )}

      {plans.length > 0 && (
        <>
          <h4 className={styles.subtitle}>
            {plans.length === 1 ? 'Il piano consigliato' : 'I piani consigliati'}
          </h4>
          {plans.map((plan) => (
            <StrategyPlan
              key={plan.key}
              plan={plan}
              confirmed={confirmedPlans.includes(plan.key)}
            />
          ))}
        </>
      )}

      {opponentPlan !== null && (
        <>
          <h4 className={styles.subtitle}>Cosa vuole fare l’avversario</h4>
          <StrategyPlan plan={opponentPlan} confirmed={false} label="Il piano dell’avversario" />
        </>
      )}
    </section>
  );
}
