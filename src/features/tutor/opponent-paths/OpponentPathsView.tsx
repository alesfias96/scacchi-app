/**
 * OpponentPathsView — «Le strade dell'avversario»: cosa potrebbe fare se toccasse a lui.
 *
 * Cosa fa: non dice «l'avversario vuole fare questo», ma mostra le strade
 * **possibili**: la frase d'introduzione (quanto sono pericolose) e una
 * scheda per strada (vedi `IdeaCard`), dalla più forte secondo Stockfish,
 * ognuna con i suoi motivi e la variante da guardare sulla scacchiera.
 *
 * Riceve:
 * - `paths`: le strade (da Rust);
 * - `notation`: la notazione scelta nelle Impostazioni.
 * Restituisce: le strade.
 */
import type { Notation } from '../../../bindings/Notation';
import type { OpponentPaths } from '../../../bindings/OpponentPaths';
import { IdeaCard } from '../idea-card/IdeaCard';
import styles from './OpponentPathsView.module.css';

export interface OpponentPathsViewProps {
  paths: OpponentPaths;
  notation: Notation;
}

export function OpponentPathsView({ paths, notation }: OpponentPathsViewProps) {
  return (
    <div className={styles.paths}>
      <p className={styles.intro}>{paths.intro}</p>
      {paths.paths.map((path, index) => (
        <IdeaCard
          key={`${String(index)}-${path.moves.join(' ')}`}
          idea={path}
          title={`Strada ${String(index + 1)}`}
          notation={notation}
        />
      ))}
    </div>
  );
}
