/**
 * ObservationsPanel — la scheda "Osservazioni" accanto alla scacchiera.
 *
 * Cosa fa: un interruttore accende o spegne i rilevatori di Rust, che
 * guardano la posizione mostrata e trovano i fatti verificabili: minacce
 * (matti in una mossa, pezzi in presa), tattiche (forchette, inchiodature,
 * infilate, scoperte, sovraccarico), pedoni, sicurezza dei re, attività dei
 * pezzi e materiale. Sono divisi per gruppo; il pallino accanto a ogni fatto
 * dice a chi si riferisce (bianco o nero). Passando il mouse su un fatto (o
 * raggiungendolo con la tastiera) la scacchiera mostra frecce e cerchi sulle
 * case coinvolte.
 *
 * Contro il computer la scheda parte spenta: vederla mentre si gioca
 * toglierebbe l'esercizio di calcolo.
 *
 * Riceve:
 * - `enabled` e `onToggle`: lo stato dell'interruttore e cosa fare quando cambia;
 * - `report`: il rapporto della posizione mostrata, oppure `null` mentre arriva;
 * - `notation`: la notazione scelta nelle Impostazioni (per le mosse citate);
 * - `onHighlight`: chiamata con il fatto da evidenziare, oppure `null`.
 * Restituisce: la scheda.
 */
import type { Fact } from '../../../bindings/Fact';
import type { FactCategory } from '../../../bindings/FactCategory';
import type { GamePhase } from '../../../bindings/GamePhase';
import type { Notation } from '../../../bindings/Notation';
import type { PositionReport } from '../../../bindings/PositionReport';
import { PanelCard } from '../../../components/panel-card/PanelCard';
import { Switch } from '../../../components/switch/Switch';
import { classNames } from '../../../lib/classNames';
import { formatFact } from '../../../lib/formatFact';
import styles from './ObservationsPanel.module.css';

/** I titoli dei gruppi, nell'ordine in cui compaiono. */
const CATEGORY_TITLES: readonly [FactCategory, string][] = [
  ['threats', 'Minacce'],
  ['tactics', 'Tattiche'],
  ['pawns', 'Pedoni'],
  ['kings', 'Re'],
  ['activity', 'Pezzi'],
  ['material', 'Materiale'],
];

/** Il nome di ogni fase di gioco. */
const PHASE_NAMES: Record<GamePhase, string> = {
  opening: 'apertura',
  middlegame: 'mediogioco',
  endgame: 'finale',
};

export interface ObservationsPanelProps {
  /** `true` se le osservazioni sono accese. */
  enabled: boolean;
  /** Chiamata quando l'utente accende o spegne le osservazioni. */
  onToggle: (enabled: boolean) => void;
  /** Il rapporto della posizione mostrata, oppure `null`. */
  report: PositionReport | null;
  /** La notazione scelta nelle Impostazioni. */
  notation: Notation;
  /** Chiamata con il fatto da evidenziare sulla scacchiera, oppure `null`. */
  onHighlight: (fact: Fact | null) => void;
}

export function ObservationsPanel({
  enabled,
  onToggle,
  report,
  notation,
  onHighlight,
}: ObservationsPanelProps) {
  return (
    <PanelCard title="Osservazioni">
      <Switch label="Mostra osservazioni" checked={enabled} onChange={onToggle} />
      {enabled && report === null && <p className={styles.message}>Guardo la posizione…</p>}
      {enabled && report !== null && (
        <div className={styles.report}>
          <p className={styles.summary}>Fase di gioco: {PHASE_NAMES[report.phase]}</p>
          {report.facts.length === 0 && (
            <p className={styles.message}>Niente di particolare da segnalare.</p>
          )}
          {CATEGORY_TITLES.map(([category, title]) => {
            const facts = report.facts.filter((fact) => fact.category === category);
            if (facts.length === 0) {
              return null;
            }
            return (
              <section key={category} className={styles.group} aria-label={title}>
                <h3 className={styles.groupTitle}>{title}</h3>
                <ul className={styles.list}>
                  {facts.map((fact) => (
                    <li key={`${fact.side}-${fact.text}`}>
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
                        <span
                          className={classNames(
                            styles.dot,
                            fact.side === 'white' ? styles.white : styles.black,
                          )}
                          aria-label={fact.side === 'white' ? 'Bianco' : 'Nero'}
                        />
                        <span>{formatFact(fact, notation)}</span>
                      </button>
                    </li>
                  ))}
                </ul>
              </section>
            );
          })}
        </div>
      )}
    </PanelCard>
  );
}
