/**
 * AnalysisPanel — la scheda "Analisi" accanto alla scacchiera.
 *
 * Cosa fa: un interruttore accende o spegne l'analisi di Stockfish sulla
 * posizione mostrata. Quando è accesa mostra la valutazione (`+0.4`, `M3`),
 * la profondità raggiunta e la variante migliore, scritta nella notazione
 * scelta. Sulla scacchiera compaiono intanto la barra di valutazione e la
 * freccia della mossa migliore (le disegna `PlayPage`).
 *
 * Contro il computer l'analisi parte spenta: vederla mentre si gioca
 * toglierebbe l'esercizio di calcolo.
 *
 * Riceve:
 * - `enabled` e `onToggle`: lo stato dell'interruttore e cosa fare quando cambia;
 * - `update`: l'ultimo giudizio del motore sulla posizione mostrata, oppure
 *   `null` mentre si aspetta il primo;
 * - `isGameOver`: `true` se nella posizione mostrata la partita è finita
 *   (non c'è nulla da analizzare);
 * - `notation`: la notazione scelta nelle Impostazioni.
 * Restituisce: la scheda.
 */
import type { AnalysisUpdate } from '../../../bindings/AnalysisUpdate';
import type { Notation } from '../../../bindings/Notation';
import { formatEvaluation } from '../../../lib/formatEvaluation';
import { PanelCard } from '../../../components/panel-card/PanelCard';
import { Switch } from '../../../components/switch/Switch';
import styles from './AnalysisPanel.module.css';
import { formatLine } from '../../../lib/formatLine';

/** Quante mezze mosse della variante mostrare. */
const LINE_LENGTH = 12;

export interface AnalysisPanelProps {
  /** `true` se l'analisi è accesa. */
  enabled: boolean;
  /** Chiamata quando l'utente accende o spegne l'analisi. */
  onToggle: (enabled: boolean) => void;
  /** L'ultimo giudizio sulla posizione mostrata, oppure `null`. */
  update: AnalysisUpdate | null;
  /** `true` se la partita è finita nella posizione mostrata. */
  isGameOver: boolean;
  /** La notazione scelta nelle Impostazioni. */
  notation: Notation;
}

export function AnalysisPanel({
  enabled,
  onToggle,
  update,
  isGameOver,
  notation,
}: AnalysisPanelProps) {
  return (
    <PanelCard title="Analisi">
      <Switch label="Mostra analisi di Stockfish" checked={enabled} onChange={onToggle} />
      {enabled && isGameOver && <p className={styles.message}>La partita è finita qui.</p>}
      {enabled && !isGameOver && update === null && (
        <p className={styles.message}>Stockfish sta guardando la posizione…</p>
      )}
      {enabled && !isGameOver && update !== null && (
        <div className={styles.result} aria-live="polite">
          <p className={styles.summary}>
            <span className={styles.evaluation}>{formatEvaluation(update.evaluation)}</span>
            <span className={styles.depth}>profondità {update.depth}</span>
          </p>
          <p className={styles.line}>{formatLine(update.line, notation, LINE_LENGTH)}</p>
        </div>
      )}
    </PanelCard>
  );
}
