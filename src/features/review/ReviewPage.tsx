/**
 * ReviewPage — la sezione «Analisi partite»: la revisione di una partita giocata.
 *
 * Cosa fa: si apre dall'Archivio (o da «Gioca» a fine partita) con la partita
 * scelta. Se la partita non è ancora stata analizzata propone «Analizza la
 * partita» e mostra l'avanzamento (Stockfish guarda ogni posizione per circa un
 * secondo; si può fermare). Quando l'analisi è pronta:
 * - a sinistra la scacchiera, sotto il grafico della partita (un clic porta a
 *   quella mossa) e i comandi per scorrere (anche con ←/→, Home e Fine);
 * - a destra, alta quanto la scacchiera e con lo scorrimento interno: il
 *   riepilogo (precisione ed errori di ciascuno), i momenti critici spiegati e
 *   le mosse con i simboli ?!, ?, ??.
 * «Mostra sulla scacchiera» di un momento critico porta alla posizione prima
 * dell'errore: la freccia verde è la mossa migliore, quella blu la mossa giocata.
 *
 * Riceve: niente (la partita arriva dall'indirizzo: `/analisi?partita=…`).
 * Restituisce: la pagina della sezione.
 */
import { Link, useSearch } from '@tanstack/react-router';
import { useCallback, useEffect, useMemo, useState } from 'react';
import type { GameReview } from '../../bindings/GameReview';
import type { Side } from '../../bindings/Side';
import { Button } from '../../components/button/Button';
import { AddNoteButton } from '../notes/add-note-button/AddNoteButton';
import { Chessboard } from '../../components/chessboard/Chessboard';
import { EvalGraph, type GraphMark } from '../../components/eval-graph/EvalGraph';
import { Icon } from '../../components/icon/Icon';
import { useGameKeyboard } from '../../hooks/useGameKeyboard';
import { useReviewStore } from '../../state/reviewStore';
import { useLoadedSettings } from '../../state/settingsStore';
import { CriticalMoments } from './critical-moments/CriticalMoments';
import { qualitySeverity } from './describeQuality';
import { ReviewMoveList } from './review-moves/ReviewMoveList';
import { ReviewSummary } from './review-summary/ReviewSummary';
import styles from './ReviewPage.module.css';
import { useGameReview } from './useGameReview';

/** La posizione mostrata: dopo la mossa `position`, oppure prima dell'errore di un momento critico. */
interface BoardView {
  /** La partita a cui si riferisce (cambiando partita si riparte dall'inizio). */
  gameId: string | null;
  /** La posizione (0 = partenza, 1 = dopo la prima mossa…). */
  position: number;
  /** La mezza mossa del momento critico mostrato, oppure `null`. */
  focus: number | null;
}

/** La revisione pronta, con la scacchiera, il grafico e le schede. */
function ReviewBoard({ gameId, review }: { gameId: string; review: GameReview }) {
  const { notation } = useLoadedSettings();
  const [view, setView] = useState<BoardView>({ gameId, position: 0, focus: null });
  const [orientation, setOrientation] = useState<Side>('white');
  const current = view.gameId === gameId ? view : { gameId, position: 0, focus: null };
  const last = review.moves.length;

  const goTo = useCallback(
    (position: number) => {
      setView({ gameId, position: Math.min(Math.max(position, 0), last), focus: null });
    },
    [gameId, last],
  );
  const showMoment = useCallback(
    (ply: number) => {
      setView({ gameId, position: ply - 1, focus: ply });
    },
    [gameId],
  );

  const keyboard = useMemo(
    () => ({
      onPrevious: () => {
        goTo(current.position - 1);
      },
      onNext: () => {
        goTo(current.position + 1);
      },
      onFirst: () => {
        goTo(0);
      },
      onLast: () => {
        goTo(last);
      },
    }),
    [goTo, current.position, last],
  );
  useGameKeyboard(keyboard);

  const marks: GraphMark[] = review.moves.flatMap((move) => {
    const severity = qualitySeverity(move.quality);
    return severity === null ? [] : [{ ply: move.ply, severity }];
  });

  const position = review.positions[current.position] ?? review.positions[0];
  const focused = current.focus === null ? undefined : review.moves[current.focus - 1];
  const lastMove =
    current.focus === null ? (review.moves[current.position - 1]?.played ?? null) : null;
  const boardMarks = useMemo(
    () =>
      focused === undefined
        ? null
        : { arrows: [{ from: focused.played.from, to: focused.played.to }], circles: [] },
    [focused],
  );

  if (position === undefined) {
    return null;
  }

  return (
    <div className={styles.workspace}>
      <div className={styles.boardArea}>
        <Chessboard
          boardFen={position.boardFen}
          turn={position.turn}
          inCheck={position.inCheck}
          orientation={orientation}
          lastMove={lastMove}
          arrow={focused?.best ?? null}
          marks={boardMarks}
        />
        <EvalGraph
          chances={review.whiteChances}
          marks={marks}
          current={current.position}
          onSelect={goTo}
        />
        <div className={styles.controls}>
          <Button
            variant="icon"
            aria-label="Inizio"
            onClick={() => {
              goTo(0);
            }}
          >
            <Icon name="first" />
          </Button>
          <Button
            variant="icon"
            aria-label="Mossa precedente"
            onClick={() => {
              goTo(current.position - 1);
            }}
          >
            <Icon name="previous" />
          </Button>
          <Button
            variant="icon"
            aria-label="Mossa successiva"
            onClick={() => {
              goTo(current.position + 1);
            }}
          >
            <Icon name="next" />
          </Button>
          <Button
            variant="icon"
            aria-label="Fine"
            onClick={() => {
              goTo(last);
            }}
          >
            <Icon name="last" />
          </Button>
          <Button
            onClick={() => {
              setOrientation((side) => (side === 'white' ? 'black' : 'white'));
            }}
          >
            Gira la scacchiera
          </Button>
          <AddNoteButton link={{ fen: position.fen, gameId, ply: current.position }} />
          {focused !== undefined && (
            <span className={styles.legend}>
              Freccia verde: la mossa migliore · freccia blu: la mossa giocata
            </span>
          )}
        </div>
      </div>

      <aside className={styles.sidePanel} aria-label="Analisi">
        <div className={styles.sideContent}>
          <ReviewSummary white={review.white} black={review.black} />
          <CriticalMoments
            moments={review.critical}
            moves={review.moves}
            selectedPly={current.focus}
            notation={notation}
            onShow={showMoment}
          />
          <ReviewMoveList
            moves={review.moves}
            current={current.position}
            notation={notation}
            onSelect={goTo}
          />
        </div>
      </aside>
    </div>
  );
}

export function ReviewPage() {
  // "strict: false": la pagina legge l'indirizzo senza dipendere dalla rotta (anche nei test).
  const { partita } = useSearch({ strict: false });
  const gameId = partita ?? null;
  const runningId = useReviewStore((state) => state.runningId);
  const progress = useReviewStore((state) => state.progress);
  const error = useReviewStore((state) => state.error);
  const listen = useReviewStore((state) => state.listen);
  const start = useReviewStore((state) => state.start);
  const cancel = useReviewStore((state) => state.cancel);
  const reviewState = useGameReview(gameId);

  useEffect(() => {
    void listen();
  }, [listen]);

  if (gameId === null) {
    return (
      <section className={styles.page}>
        <h1 className={styles.title}>Analisi partite</h1>
        <p className={styles.message}>
          Scegli una partita nell’<Link to="/archivio">Archivio</Link> e premi «Analizza»: il tutor
          troverà i momenti critici e ti spiegherà gli errori.
        </p>
      </section>
    );
  }

  const isRunningHere = runningId === gameId;
  const progressHere = progress?.gameId === gameId ? progress : null;

  return (
    <section className={styles.page}>
      <h1 className={styles.hiddenTitle}>Analisi partite</h1>
      {error !== null && (
        <p className={styles.error} role="alert">
          {error}
        </p>
      )}
      {reviewState.status === 'loading' && <p className={styles.message}>Leggo l’analisi…</p>}
      {reviewState.status === 'error' && (
        <p className={styles.error} role="alert">
          {reviewState.message}
        </p>
      )}
      {reviewState.status === 'missing' && (
        <div className={styles.start}>
          <h2 className={styles.title}>Analisi partite</h2>
          {isRunningHere ? (
            <>
              <p className={styles.message}>
                Stockfish sta guardando la partita, una posizione alla volta
                {progressHere === null
                  ? '…'
                  : ` (${String(progressHere.done)} di ${String(progressHere.total)}).`}
              </p>
              <progress
                className={styles.progress}
                value={progressHere?.done ?? 0}
                max={progressHere?.total ?? 1}
              />
              <Button
                onClick={() => {
                  void cancel();
                }}
              >
                Ferma l’analisi
              </Button>
            </>
          ) : (
            <>
              <p className={styles.message}>
                Questa partita non è ancora stata analizzata. L’analisi richiede circa un secondo
                per ogni posizione e continua anche se cambi sezione.
              </p>
              <Button
                variant="primary"
                disabled={runningId !== null}
                onClick={() => {
                  void start(gameId);
                }}
              >
                Analizza la partita
              </Button>
              {runningId !== null && (
                <p className={styles.message}>È già in corso l’analisi di un’altra partita.</p>
              )}
            </>
          )}
        </div>
      )}
      {reviewState.status === 'ready' && (
        <ReviewBoard gameId={gameId} review={reviewState.review} />
      )}
    </section>
  );
}
