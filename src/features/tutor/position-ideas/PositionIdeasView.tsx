/**
 * PositionIdeasView — le idee della posizione: le migliori varianti di Stockfish, spiegate.
 *
 * Cosa fa: per ogni variante mostra la sua idea (vedi `IdeaCard`), dalla
 * preferita da Stockfish, e sotto il confronto tra la prima e la seconda
 * (perché la prima è migliore, o se si equivalgono).
 *
 * Riceve:
 * - `ideas`: le idee della posizione (da Rust);
 * - `notation`: la notazione scelta nelle Impostazioni.
 * Restituisce: le idee.
 */
import type { Notation } from '../../../bindings/Notation';
import type { PositionIdeas } from '../../../bindings/PositionIdeas';
import { IdeaCard } from '../idea-card/IdeaCard';
import styles from './PositionIdeasView.module.css';

export interface PositionIdeasViewProps {
  ideas: PositionIdeas;
  notation: Notation;
}

/** Il titoletto di ogni idea. */
function ideaTitle(index: number): string {
  return index === 0 ? 'Idea 1 · la preferita da Stockfish' : `Idea ${String(index + 1)}`;
}

export function PositionIdeasView({ ideas, notation }: PositionIdeasViewProps) {
  if (ideas.ideas.length === 0) {
    return <p className={styles.note}>Nessuna idea: la partita in questa posizione è finita.</p>;
  }
  return (
    <div className={styles.ideas}>
      {ideas.ideas.map((idea, index) => (
        <IdeaCard
          key={`${String(index)}-${idea.moves.join(' ')}`}
          idea={idea}
          title={ideaTitle(index)}
          notation={notation}
        />
      ))}
      {ideas.comparison !== null && <p className={styles.note}>{ideas.comparison}</p>}
    </div>
  );
}
