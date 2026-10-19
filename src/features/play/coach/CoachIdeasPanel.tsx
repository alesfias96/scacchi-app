/**
 * CoachIdeasPanel — lo "sguardo lungo" della modalità Tutor, nella colonna di destra.
 *
 * Cosa fa: dopo ogni mossa mostra due schede:
 * - «L'idea della tua mossa»: cosa ottiene la tua ultima mossa guardando
 *   avanti e, se Stockfish preferiva un'altra mossa, perché (con la sua idea);
 * - «Le idee per continuare»: le migliori varianti della posizione, spiegate;
 * - «Le strade dell'avversario»: cosa potrebbe fare se toccasse a lui.
 * Dopo un errore, sotto il confronto, quale passo del metodo «Prima di
 * muovere» avrebbe trovato la mossa migliore.
 * Ogni idea si può guardare sulla scacchiera. Mentre il tutor calcola, lo dice.
 *
 * Riceve:
 * - `ideas`: le idee pronte (vedi `useCoachIdeas`);
 * - `notation`: la notazione scelta nelle Impostazioni.
 * Restituisce: le due schede.
 */
import type { Notation } from '../../../bindings/Notation';
import { PanelCard } from '../../../components/panel-card/PanelCard';
import { formatSan } from '../../../lib/formatSan';
import { IdeaCard } from '../../tutor/idea-card/IdeaCard';
import { OpponentPathsView } from '../../tutor/opponent-paths/OpponentPathsView';
import { PositionIdeasView } from '../../tutor/position-ideas/PositionIdeasView';
import styles from './CoachIdeasPanel.module.css';
import type { CoachIdeas } from './useCoachIdeas';

export interface CoachIdeasPanelProps {
  ideas: CoachIdeas;
  notation: Notation;
}

export function CoachIdeasPanel({ ideas, notation }: CoachIdeasPanelProps) {
  const { move, position, paths, isThinking } = ideas;
  const better = move?.idea.better ?? null;
  const betterMove = better?.moves[0];

  return (
    <>
      {move !== null && (
        <PanelCard title="L’idea della tua mossa">
          <IdeaCard
            idea={move.idea.played}
            title={`${String(move.record.moveNumber)}${move.record.side === 'white' ? '.' : '…'} ${formatSan(move.record.san, notation)}`}
            notation={notation}
          />
          {better !== null && (
            <>
              {move.idea.comparison !== null && (
                <p className={styles.comparison}>{move.idea.comparison}</p>
              )}
              {move.idea.lesson !== null && <p className={styles.lesson}>{move.idea.lesson}</p>}
              <IdeaCard
                idea={better}
                title={`La mossa migliore${betterMove === undefined ? '' : `: ${formatSan(betterMove, notation)}`}`}
                notation={notation}
              />
            </>
          )}
        </PanelCard>
      )}

      {(position !== null || isThinking) && (
        <PanelCard title="Le idee per continuare">
          {position !== null && <PositionIdeasView ideas={position} notation={notation} />}
          {isThinking && (
            <p className={styles.message} aria-live="polite">
              Il tutor sta guardando avanti…
            </p>
          )}
        </PanelCard>
      )}

      {paths !== null && (
        <PanelCard title="Le strade dell’avversario">
          <OpponentPathsView paths={paths} notation={notation} />
        </PanelCard>
      )}
    </>
  );
}
