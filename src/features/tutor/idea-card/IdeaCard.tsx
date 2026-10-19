/**
 * IdeaCard — un'idea del tutor: cosa vuole ottenere una variante, guardando lontano.
 *
 * Cosa fa: mostra la frase principale («L'idea: … — si vede tra 3 mosse») con
 * la mossa da cui si vede, la valutazione di Stockfish, i momenti chiave (ognuno
 * con la sua mossa: un clic lo mostra sulla scacchiera) e la variante intera.
 * «Guarda sulla scacchiera» fa vedere la variante sulla scacchiera della
 * pagina: qui compaiono i pulsanti per andare avanti e indietro, la mossa
 * mostrata, il commento dei momenti chiave e «Torna alla partita».
 *
 * Riceve:
 * - `idea`: l'idea (da Rust);
 * - `title`: il titoletto (es. "Idea 1", "L'idea della tua mossa");
 * - `notation`: la notazione scelta nelle Impostazioni.
 * Sotto i momenti, i motivi tattici della variante (con «Allenati» per i
 * puzzle dello stesso tema) e le schede che l'idea cita, da aprire.
 * Restituisce: la scheda dell'idea.
 */
import { useState } from 'react';
import type { IdeaView } from '../../../bindings/IdeaView';
import type { Notation } from '../../../bindings/Notation';
import { Button } from '../../../components/button/Button';
import { Icon } from '../../../components/icon/Icon';
import { KnowledgeCards } from '../../../components/knowledge-cards/KnowledgeCards';
import { formatEvaluation } from '../../../lib/formatEvaluation';
import { formatLine } from '../../../lib/formatLine';
import { useIdeaReplayStore } from '../../../state/ideaReplayStore';
import { TrainThemeButton } from '../../training/train-theme-button/TrainThemeButton';
import { useTrainTheme } from '../../training/useTrainTheme';
import { ideaMoveLabel } from '../ideaMoves';
import styles from './IdeaCard.module.css';

export interface IdeaCardProps {
  idea: IdeaView;
  title: string;
  notation: Notation;
}

/** Quante mezze mosse della variante scrivere al massimo. */
const MAX_LINE_PLIES = 20;

export function IdeaCard({ idea, title, notation }: IdeaCardProps) {
  const replay = useIdeaReplayStore();
  const trainTheme = useTrainTheme();
  /** La scheda della base di conoscenza aperta. */
  const [openCardId, setOpenCardId] = useState<string | null>(null);
  const isShown = replay.idea === idea;
  const lastFrame = idea.frames.length - 1;
  /** Il momento chiave della posizione mostrata, se c'è. */
  const shownMoment = isShown
    ? idea.moments.find((moment) => moment.ply === replay.frame - 1)
    : undefined;

  return (
    <article className={styles.card} aria-label={title}>
      <p className={styles.title}>{title}</p>
      <p className={styles.headline}>{idea.headline}</p>
      {idea.keyPly !== null && (
        <p className={styles.meta}>
          Si vede dopo {ideaMoveLabel(idea, idea.keyPly, notation)}
          {idea.evaluation !== null && ` · valutazione ${formatEvaluation(idea.evaluation)}`}
        </p>
      )}

      {idea.moments.length > 0 && (
        <ol className={styles.moments}>
          {idea.moments.map((moment) => (
            <li key={`${String(moment.ply)}-${moment.text}`} className={styles.moment}>
              <button
                type="button"
                className={styles.momentButton}
                onClick={() => {
                  replay.show(idea);
                  useIdeaReplayStore.getState().goTo(moment.ply + 1);
                }}
              >
                <span className={styles.move}>{ideaMoveLabel(idea, moment.ply, notation)}</span>{' '}
                {moment.text}
              </button>
            </li>
          ))}
        </ol>
      )}

      {idea.motifs.length > 0 && (
        <div className={styles.motifs}>
          <p className={styles.meta}>
            Motivi: {idea.motifs.map((motif) => motif.name).join(' · ')}
          </p>
          <div className={styles.motifActions}>
            {idea.motifs.map((motif) =>
              motif.puzzleTheme === null ? null : (
                <TrainThemeButton key={motif.key} theme={motif.puzzleTheme} name={motif.name} />
              ),
            )}
          </div>
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

      <p className={styles.line}>
        {formatLine(
          { moveNumber: idea.firstMoveNumber, turn: idea.firstSide, san: idea.moves },
          notation,
          MAX_LINE_PLIES,
        )}
      </p>

      {isShown ? (
        <div className={styles.player} role="group" aria-label="La variante sulla scacchiera">
          <div className={styles.controls}>
            <Button
              variant="icon"
              aria-label="Inizio della variante"
              disabled={replay.frame === 0}
              onClick={() => {
                replay.goTo(0);
              }}
            >
              <Icon name="first" />
            </Button>
            <Button
              variant="icon"
              aria-label="Mossa precedente della variante"
              disabled={replay.frame === 0}
              onClick={() => {
                replay.goTo(replay.frame - 1);
              }}
            >
              <Icon name="previous" />
            </Button>
            <Button
              variant="icon"
              aria-label="Mossa successiva della variante"
              disabled={replay.frame === lastFrame}
              onClick={() => {
                replay.goTo(replay.frame + 1);
              }}
            >
              <Icon name="next" />
            </Button>
            <span className={styles.position} aria-live="polite">
              {replay.frame === 0
                ? 'Posizione di partenza'
                : ideaMoveLabel(idea, replay.frame - 1, notation)}
            </span>
          </div>
          {shownMoment !== undefined && <p className={styles.comment}>{shownMoment.text}</p>}
          <Button onClick={replay.close}>Torna alla partita</Button>
        </div>
      ) : (
        <Button
          onClick={() => {
            replay.show(idea);
          }}
        >
          Guarda sulla scacchiera
        </Button>
      )}
    </article>
  );
}
