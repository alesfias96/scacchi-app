/**
 * TrainingPage — la sezione "Allenamento": puzzle per allenare il calcolo.
 *
 * Cosa fa: a sinistra la scacchiera del puzzle, a destra la colonna con le
 * schede (sempre aperta, con il suo scorrimento interno, come in "Gioca"):
 * - "Il tuo livello": il rating dei puzzle, quanti risolti, quanti da ripassare;
 * - "Allenamento": le scelte (modalità, mosse da trovare, tema, difficoltà) e
 *   il pulsante "Nuovo puzzle";
 * - "Puzzle": il messaggio, l'avanzamento, i pulsanti Aiuto / Mostra soluzione
 *   / Prossimo puzzle;
 * - "Soluzione", a puzzle finito: rating, difficoltà, temi, soluzione e
 *   perché funziona (passando il mouse su un fatto, la scacchiera mostra le case).
 * Il puzzle comincia dopo la mossa dell'avversario; si muovono i pezzi come
 * in "Gioca" (anche la promozione). Una mossa sbagliata torna indietro e si
 * può riprovare; una giusta riceve subito la risposta dell'avversario.
 * Nella visualizzazione, dopo i secondi scelti, i pezzi spariscono.
 *
 * Tutte le regole (mossa giusta o sbagliata, rating, ripasso) sono in Rust.
 *
 * Riceve: niente.
 * Restituisce: la pagina della sezione.
 */
import { useCallback, useEffect, useMemo, useState } from 'react';
import type { Fact } from '../../bindings/Fact';
import type { LegalMove } from '../../bindings/LegalMove';
import type { TrainingSetup } from '../../bindings/TrainingSetup';
import { Chessboard } from '../../components/chessboard/Chessboard';
import { PanelCard } from '../../components/panel-card/PanelCard';
import {
  PromotionPicker,
  type PromotionPiece,
} from '../../components/promotion-picker/PromotionPicker';
import { useInitialPosition } from '../../hooks/useInitialPosition';
import { useMoveSound } from '../../hooks/useMoveSound';
import { useLoadedSettings, useSettingsStore } from '../../state/settingsStore';
import { useTrainingStore } from '../../state/trainingStore';
import { PuzzleReveal } from './puzzle-reveal/PuzzleReveal';
import { PuzzleStatus } from './puzzle-status/PuzzleStatus';
import { TrainingSetupPanel } from './training-setup/TrainingSetupPanel';
import styles from './TrainingPage.module.css';
import { usePiecesHidden } from './usePiecesHidden';

/** Una promozione in attesa della scelta del pezzo: le due case della mossa. */
interface PendingPromotion {
  from: string;
  to: string;
}

/** Nessuna mossa possibile (a puzzle finito). */
const NO_MOVES: readonly LegalMove[] = [];

export function TrainingPage() {
  const { notation, sounds, trainingSetup } = useLoadedSettings();
  const updateSettings = useSettingsStore((state) => state.update);
  const overview = useTrainingStore((state) => state.overview);
  const attempt = useTrainingStore((state) => state.attempt);
  const error = useTrainingStore((state) => state.error);
  const loadOverview = useTrainingStore((state) => state.loadOverview);
  const start = useTrainingStore((state) => state.start);
  const play = useTrainingStore((state) => state.play);
  const hint = useTrainingStore((state) => state.hint);
  const showSolution = useTrainingStore((state) => state.showSolution);
  const initial = useInitialPosition();

  /** La promozione in attesa della scelta del pezzo, oppure `null`. */
  const [pendingPromotion, setPendingPromotion] = useState<PendingPromotion | null>(null);
  /** Cambiandolo, la scacchiera torna alla posizione ricevuta da Rust. */
  const [resyncKey, setResyncKey] = useState(0);
  /** Il fatto della soluzione indicato con il mouse, da evidenziare. */
  const [highlighted, setHighlighted] = useState<Fact | null>(null);

  // All'apertura della sezione legge il riepilogo (rating, temi, ripassi).
  useEffect(() => {
    void loadOverview();
  }, [loadOverview]);

  const game = attempt?.game ?? null;
  useMoveSound(game, sounds === 'on');
  const { hidden, memorizing } = usePiecesHidden(attempt, trainingSetup.visualizationSeconds);

  // A ogni nuovo puzzle l'evidenziazione si toglie.
  const puzzleId = attempt?.puzzleId;
  useEffect(() => {
    setHighlighted(null);
  }, [puzzleId]);

  /** I segni sulla scacchiera: il pezzo dell'aiuto, oppure le case del fatto indicato. */
  const hintSquare = attempt?.hintSquare ?? null;
  const marks = useMemo(() => {
    if (highlighted !== null) {
      return { arrows: highlighted.arrows, circles: highlighted.squares };
    }
    return hintSquare === null ? null : { arrows: [], circles: [hintSquare] };
  }, [highlighted, hintSquare]);

  /** Cambia le scelte e le ricorda. */
  const changeSetup = useCallback(
    (setup: TrainingSetup) => {
      void updateSettings({ trainingSetup: setup });
    },
    [updateSettings],
  );

  /** Comincia un nuovo puzzle con le scelte attuali. */
  const startPuzzle = useCallback(() => {
    setPendingPromotion(null);
    void start(trainingSetup);
  }, [start, trainingSetup]);

  /** Prova una mossa; la scacchiera torna sempre alla posizione decisa da Rust. */
  const playMove = useCallback(
    async (uci: string) => {
      await play(uci);
      setResyncKey((key) => key + 1);
    },
    [play],
  );

  /** Chiamata quando l'utente sposta un pezzo: se è una promozione chiede il pezzo. */
  const handleMove = useCallback(
    (from: string, to: string) => {
      const candidates = (game?.position.legalMoves ?? []).filter(
        (move) => move.from === from && move.to === to,
      );
      const [onlyMove] = candidates;
      if (candidates.length > 1) {
        setPendingPromotion({ from, to });
      } else if (onlyMove !== undefined) {
        void playMove(onlyMove.uci);
      } else {
        setResyncKey((key) => key + 1);
      }
    },
    [game, playMove],
  );

  /** Gioca la promozione con il pezzo scelto. */
  const choosePromotion = useCallback(
    (piece: PromotionPiece) => {
      if (pendingPromotion !== null) {
        void playMove(`${pendingPromotion.from}${pendingPromotion.to}${piece}`);
      }
      setPendingPromotion(null);
    },
    [pendingPromotion, playMove],
  );

  /** Rinuncia alla promozione: il pedone torna indietro. */
  const cancelPromotion = useCallback(() => {
    setPendingPromotion(null);
    setResyncKey((key) => key + 1);
  }, []);

  const canMove = attempt !== null && !attempt.finished;

  return (
    <section className={styles.page}>
      {/* Il titolo resta per i lettori di schermo, ma non occupa spazio. */}
      <h1 className={styles.hiddenTitle}>Allenamento</h1>

      <div className={styles.workspace}>
        <div className={styles.boardArea}>
          <div className={styles.boardWrap}>
            {game !== null && attempt !== null ? (
              <Chessboard
                boardFen={game.position.boardFen}
                turn={game.position.turn}
                inCheck={game.position.inCheck}
                orientation={attempt.orientation}
                lastMove={game.lastMove}
                legalMoves={canMove ? game.position.legalMoves : NO_MOVES}
                onMove={canMove ? handleMove : undefined}
                marks={marks}
                hidePieces={hidden}
                resyncKey={resyncKey}
              />
            ) : (
              initial.status === 'ready' && (
                <Chessboard
                  boardFen={initial.position.boardFen}
                  turn={initial.position.turn}
                  inCheck={initial.position.inCheck}
                />
              )
            )}
            {pendingPromotion !== null && game !== null && (
              <PromotionPicker
                side={game.position.turn}
                onChoose={choosePromotion}
                onCancel={cancelPromotion}
              />
            )}
          </div>
          {error !== null && (
            <p className={styles.error} role="alert">
              {error}
            </p>
          )}
        </div>

        <aside className={styles.sidePanel} aria-label="Allenamento">
          <PanelCard title="Il tuo livello">
            {overview === null ? (
              <p className={styles.message}>Leggo i tuoi risultati…</p>
            ) : (
              <dl className={styles.stats}>
                <dt>Rating dei puzzle</dt>
                <dd>{overview.rating}</dd>
                <dt>Puzzle risolti</dt>
                <dd>
                  {overview.solved} su {overview.attempted}
                </dd>
                <dt>Da ripassare oggi</dt>
                <dd>{overview.dueReviews}</dd>
              </dl>
            )}
          </PanelCard>

          <TrainingSetupPanel
            setup={trainingSetup}
            themes={overview?.themes ?? []}
            dueReviews={overview?.dueReviews ?? 0}
            onChange={changeSetup}
            onStart={startPuzzle}
          />

          {attempt === null ? (
            <PanelCard title="Puzzle">
              <p className={styles.message}>
                Scegli come allenarti e premi «Nuovo puzzle». Il puzzle comincia dopo la mossa
                dell’avversario: tocca a te trovare la continuazione migliore.
              </p>
            </PanelCard>
          ) : (
            <PuzzleStatus
              attempt={attempt}
              memorizing={memorizing}
              notation={notation}
              onHint={() => {
                void hint();
              }}
              onShowSolution={() => {
                void showSolution();
              }}
              onNext={startPuzzle}
            />
          )}

          {attempt?.reveal && (
            <PuzzleReveal
              reveal={attempt.reveal}
              rating={attempt.rating}
              ratingChange={attempt.ratingChange}
              notation={notation}
              onHighlight={setHighlighted}
            />
          )}
        </aside>
      </div>
    </section>
  );
}
