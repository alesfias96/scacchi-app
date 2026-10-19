/**
 * PlayPage — la sezione "Gioca": una partita contro il computer o a due giocatori.
 *
 * Cosa fa: chiede a Rust la partita in corso e la mostra. A sinistra la
 * scacchiera (con i giocatori, l'orologio e, se accesa, la barra di
 * valutazione accanto), grande quanto permette la finestra e sempre tutta
 * visibile; a destra la colonna "Partita", alta quanto la scacchiera e con il
 * suo scorrimento interno: in cima un selettore passa da «Partita» (tutte le
 * schede) a «Tutor» (la spiegazione della posizione). Si muovono i pezzi
 * trascinandoli o con due clic; sono permesse solo le mosse legali (arrocco,
 * presa en passant e promozione compresi), e solo quando Rust dice che si può
 * (ad esempio non mentre pensa il computer).
 * - Sopra e sotto la scacchiera: i due giocatori, con l'orologio se c'è.
 * - Accanto alla scacchiera, se l'analisi è accesa: la barra di valutazione;
 *   sulla scacchiera la freccia della mossa migliore.
 * - A fine partita: l'annuncio (motivo e punteggio) sopra la scacchiera, con
 *   «Analizza la partita» (apre la sezione «Analisi partite»).
 * - Nella colonna "Partita": le mosse, con le varianti, e i comandi (scorri,
 *   nuova partita, annulla, abbandona, gira) con il nome dell'apertura,
 *   l'analisi, le osservazioni dei rilevatori (passando il mouse su una, la
 *   scacchiera mostra le case coinvolte), la situazione, copia e incolla di posizioni (FEN) e partite
 *   (PGN), salvataggio e apertura di file `.pgn`.
 * La partita si salva da sola nell'archivio a ogni mossa; quando si apre una
 * partita (dall'archivio o all'avvio) il proprio colore va in basso.
 * Le frecce ←/→ e i tasti Home/Fine scorrono la partita. Ogni mossa ha il suo
 * suono (mossa, presa, fine partita), se i suoni sono attivi nelle Impostazioni.
 * La voce «Tutor» (o il tasto T) spiega la posizione mostrata; contro il
 * computer, a partita in corso, resta spento finché non lo si accende comunque.
 *
 * Tutte le regole sono applicate da Rust: questa pagina invia le richieste e
 * mostra la partita aggiornata che riceve (anche quando cambia da sola, per
 * una mossa del computer o il tempo scaduto).
 *
 * Con `coached` la stessa pagina diventa la sezione **Tutor** del menu: dopo
 * ogni mossa il tutor la commenta con dei messaggi sotto la scacchiera, uno
 * alla volta, mostrando sulla scacchiera le case di cui parla (vedi `coach/`).
 * La scacchiera è un po' più piccola, per lasciare posto ai messaggi.
 * La partita è la stessa di «Gioca»: si passa da una sezione all'altra senza perderla.
 *
 * Riceve: `coached` (facoltativo), `true` nella modalità Tutor.
 * Restituisce: la pagina della sezione.
 */
import { useNavigate } from '@tanstack/react-router';
import { useCallback, useEffect, useMemo, useState } from 'react';
import type { Fact } from '../../bindings/Fact';
import type { GameSetup } from '../../bindings/GameSetup';
import type { LegalMove } from '../../bindings/LegalMove';
import type { Side } from '../../bindings/Side';
import { Button } from '../../components/button/Button';
import { Chessboard } from '../../components/chessboard/Chessboard';
import { EvalBar } from '../../components/eval-bar/EvalBar';
import { PanelCard } from '../../components/panel-card/PanelCard';
import { TabSwitch, type TabOption } from '../../components/tab-switch/TabSwitch';
import { useGameKeyboard } from '../../hooks/useGameKeyboard';
import { useMoveSound } from '../../hooks/useMoveSound';
import { usePositionReport } from '../../hooks/usePositionReport';
import { useTutorTab, type SideTab } from '../../hooks/useTutorTab';
import { useTutorTarget } from '../../hooks/useTutorTarget';
import { exportPgn, savePgnFile } from '../../services/gameService';
import { useEngineStore } from '../../state/engineStore';
import { useGameStore } from '../../state/gameStore';
import { useObservationsStore } from '../../state/observationsStore';
import { useReviewStore } from '../../state/reviewStore';
import type { TutorTarget } from '../../state/tutorStore';
import { useLoadedSettings, useSettingsStore } from '../../state/settingsStore';
import { AnalysisPanel } from './analysis-panel/AnalysisPanel';
import { GameControls } from './game-controls/GameControls';
import { GameResultBanner } from './game-result-banner/GameResultBanner';
import { AddNoteButton } from '../notes/add-note-button/AddNoteButton';
import { playLink } from '../notes/noteLinks';
import { ImportExport } from './import-export/ImportExport';
import { MoveList } from './move-list/MoveList';
import { NewGameDialog } from './new-game-dialog/NewGameDialog';
import { ObservationsPanel } from './observations-panel/ObservationsPanel';
import { OpeningLabel } from './opening-label/OpeningLabel';
import { PgnFileButtons } from './pgn-file/PgnFileButtons';
import { PlayerBar } from './player-bar/PlayerBar';
import { describePlayer } from './player-bar/describePlayer';
import { PositionPanel } from './position-panel/PositionPanel';
import {
  PromotionPicker,
  type PromotionPiece,
} from '../../components/promotion-picker/PromotionPicker';
import { classNames } from '../../lib/classNames';
import { TutorPanel } from '../tutor/TutorPanel';
import { CoachPanel } from './coach/CoachPanel';
import { CoachPopup } from './coach/CoachPopup';
import { useCoachMessages } from './coach/useCoachMessages';
import { useCoachPopup } from './coach/useCoachPopup';
import { CoachIdeasPanel } from './coach/CoachIdeasPanel';
import { useCoachIdeas } from './coach/useCoachIdeas';
import { useIdeaReplayStore } from '../../state/ideaReplayStore';
import styles from './PlayPage.module.css';

/** Una promozione in attesa della scelta del pezzo: le due case della mossa. */
interface PendingPromotion {
  from: string;
  to: string;
}

/** Le due voci in cima alla colonna di destra. */
const SIDE_TABS: readonly TabOption<SideTab>[] = [
  { value: 'main', label: 'Partita' },
  { value: 'tutor', label: 'Tutor' },
];

/** Nessuna mossa possibile (quando non si può muovere). */
const NO_MOVES: readonly LegalMove[] = [];

export interface PlayPageProps {
  /** `true` nella modalità Tutor: dopo ogni mossa compaiono i commenti del tutor. */
  coached?: boolean;
}

export function PlayPage({ coached = false }: PlayPageProps) {
  /** Il titolo della sezione. */
  const title = coached ? 'Tutor' : 'Gioca';
  const { notation, sounds, gameSetup } = useLoadedSettings();
  const updateSettings = useSettingsStore((state) => state.update);
  const navigate = useNavigate();
  const startReview = useReviewStore((state) => state.start);
  const session = useGameStore((state) => state.session);
  const loadError = useGameStore((state) => state.loadError);
  const actionError = useGameStore((state) => state.actionError);
  const load = useGameStore((state) => state.load);
  const play = useGameStore((state) => state.play);
  const undo = useGameStore((state) => state.undo);
  const goTo = useGameStore((state) => state.goTo);
  const goToNode = useGameStore((state) => state.goToNode);
  const promoteVariation = useGameStore((state) => state.promoteVariation);
  const deleteVariation = useGameStore((state) => state.deleteVariation);
  const resign = useGameStore((state) => state.resign);
  const newGame = useGameStore((state) => state.newGame);
  const importText = useGameStore((state) => state.importText);
  const openFile = useGameStore((state) => state.openFile);
  const storageError = useGameStore((state) => state.storageError);
  const dismissStorageError = useGameStore((state) => state.dismissStorageError);
  const analysisEnabled = useEngineStore((state) => state.analysisEnabled);
  const latestAnalysis = useEngineStore((state) => state.latest);
  const engineError = useEngineStore((state) => state.engineError);
  const listenToEngine = useEngineStore((state) => state.listen);
  const setAnalysisEnabled = useEngineStore((state) => state.setAnalysisEnabled);
  const dismissEngineError = useEngineStore((state) => state.dismissError);
  const observationsEnabled = useObservationsStore((state) => state.enabled);
  const setObservationsEnabled = useObservationsStore((state) => state.setEnabled);

  /** La voce scelta in cima alla colonna di destra (il tasto T passa al tutor). */
  const [sideTab, setSideTab] = useTutorTab();
  /** Il colore mostrato in basso sulla scacchiera. */
  const [orientation, setOrientation] = useState<Side>('white');
  /** La promozione in attesa della scelta del pezzo, oppure `null`. */
  const [pendingPromotion, setPendingPromotion] = useState<PendingPromotion | null>(null);
  /** Cambiandolo, la scacchiera torna alla posizione ricevuta da Rust. */
  const [resyncKey, setResyncKey] = useState(0);
  /** `true` mentre è aperta la finestra "Nuova partita". */
  const [isNewGameOpen, setIsNewGameOpen] = useState(false);
  /** L'osservazione indicata con il mouse, da evidenziare sulla scacchiera. */
  const [highlighted, setHighlighted] = useState<Fact | null>(null);

  // All'apertura della sezione legge la partita in corso e ascolta il motore.
  useEffect(() => {
    void load();
    void listenToEngine();
  }, [load, listenToEngine]);

  const game = session?.game ?? null;
  useMoveSound(game, sounds === 'on');

  // Le osservazioni della posizione mostrata (solo se accese). A ogni nuova
  // posizione l'evidenziazione si toglie: le case di prima non valgono più.
  const shownFen = game?.position.fen ?? null;
  const report = usePositionReport(shownFen, observationsEnabled);
  useEffect(() => {
    setHighlighted(null);
  }, [shownFen]);
  const marks = useMemo(
    () =>
      highlighted === null ? null : { arrows: highlighted.arrows, circles: highlighted.squares },
    [highlighted],
  );

  // A ogni partita diversa (nuova, ripresa all'avvio, aperta dall'archivio)
  // il colore della persona va in basso: il nero se il computer ha il bianco.
  const sessionId = session?.id;
  const computerSide = session?.computer?.side;
  useEffect(() => {
    if (sessionId !== undefined) {
      setOrientation(computerSide === 'white' ? 'black' : 'white');
    }
  }, [sessionId, computerSide]);

  // Modalità Tutor: i commenti dopo ogni mossa, un messaggio alla volta. Mentre
  // un messaggio è aperto la scacchiera mostra le sue case al posto delle osservazioni.
  const [coachWithEngine, setCoachWithEngine] = useState(true);
  const [coachCardId, setCoachCardId] = useState<string | null>(null);
  const coach = useCoachMessages(game, sessionId, coached, coachWithEngine, notation);
  const popup = useCoachPopup(coach.messages);
  const coachMessage = popup.isOpen ? coach.messages[popup.index] : undefined;
  const boardMarks = useMemo(
    () =>
      coachMessage === undefined
        ? marks
        : { arrows: coachMessage.fact.arrows, circles: coachMessage.fact.squares },
    [coachMessage, marks],
  );
  const coachCard = coach.explanation?.cards.find((card) => card.id === coachCardId) ?? null;
  // Lo sguardo lungo della modalità Tutor: parte dopo i commenti (serve Stockfish).
  const coachIdeas = useCoachIdeas(
    game,
    sessionId,
    coached && coachWithEngine,
    !coach.isLoading,
    computerSide === undefined ? null : computerSide === 'white' ? 'black' : 'white',
  );

  // «Guarda l'idea sulla scacchiera»: al posto della partita si vede la variante.
  // Appena la partita cambia (o si esce dalla sezione) si torna alla partita.
  const replayIdea = useIdeaReplayStore((state) => state.idea);
  const replayFrame = useIdeaReplayStore((state) => state.frame);
  const closeReplay = useIdeaReplayStore((state) => state.close);
  const replayed = replayIdea?.frames[replayFrame] ?? null;
  const replayMoment = replayIdea?.moments.find((moment) => moment.ply === replayFrame - 1);
  useEffect(() => {
    closeReplay();
    return closeReplay;
  }, [shownFen, closeReplay]);

  /** A fine partita: comincia l'analisi e apre la sezione «Analisi partite». */
  const analyzeGame = useCallback(async () => {
    if (sessionId === undefined) {
      return;
    }
    await startReview(sessionId);
    await navigate({ to: '/analisi', search: { partita: sessionId } });
  }, [sessionId, startReview, navigate]);

  /** Riporta i pezzi alla posizione vera (dopo una mossa annullata o rifiutata). */
  const resyncBoard = useCallback(() => {
    setResyncKey((key) => key + 1);
  }, []);

  /** Gioca una mossa; se Rust la rifiuta, rimette a posto i pezzi. */
  const playMove = useCallback(
    async (uci: string) => {
      await play(uci);
      if (useGameStore.getState().actionError !== null) {
        resyncBoard();
      }
    },
    [play, resyncBoard],
  );

  /**
   * Chiamata quando l'utente sposta un pezzo sulla scacchiera.
   * Se la mossa può essere una promozione (più mosse legali con le stesse
   * case, una per pezzo) chiede quale pezzo; altrimenti la gioca subito.
   */
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
        resyncBoard();
      }
    },
    [game, playMove, resyncBoard],
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
    resyncBoard();
  }, [resyncBoard]);

  /**
   * Avvia la partita scelta nella finestra "Nuova partita": ricorda le scelte
   * e, contro il computer, spegne analisi e osservazioni (sarebbero un aiuto).
   */
  const startGame = useCallback(
    async (setup: GameSetup) => {
      setIsNewGameOpen(false);
      void updateSettings({ gameSetup: setup });
      if (setup.opponent === 'computer') {
        if (useEngineStore.getState().analysisEnabled) {
          void setAnalysisEnabled(false);
        }
        setObservationsEnabled(false);
      }
      await newGame(setup);
    },
    [updateSettings, setAnalysisEnabled, setObservationsEnabled, newGame],
  );

  // Il tutor spiega la posizione mostrata. Contro il computer, a partita in
  // corso, resta spento (sarebbe un aiuto) finché non lo si accende comunque.
  const startingFen = game?.startingFen;
  const shownMoves = useMemo(
    () => (game === null ? [] : game.moves.slice(0, game.cursor).map((move) => move.uci)),
    [game],
  );
  const isComputerGameRunning = computerSide !== undefined && game?.result === null;
  const tutorTarget = useMemo<TutorTarget | null>(
    () =>
      sessionId === undefined || startingFen === undefined
        ? null
        : {
            startFen: startingFen,
            moves: shownMoves,
            orientation,
            lock: isComputerGameRunning ? 'computerGame' : null,
            lockKey: sessionId,
          },
    [sessionId, startingFen, shownMoves, orientation, isComputerGameRunning],
  );
  useTutorTarget(tutorTarget);

  /** Il numero di mosse della partita e quella mostrata (per la tastiera). */
  const cursor = game?.cursor ?? 0;
  const totalMoves = game?.moves.length ?? 0;

  /** Le azioni della tastiera: stesse dei pulsanti di navigazione. */
  const keyboardActions = useMemo(
    () => ({
      onPrevious: () => {
        if (cursor > 0) {
          void goTo(cursor - 1);
        }
      },
      onNext: () => {
        if (cursor < totalMoves) {
          void goTo(cursor + 1);
        }
      },
      onFirst: () => {
        void goTo(0);
      },
      onLast: () => {
        void goTo(totalMoves);
      },
    }),
    [cursor, totalMoves, goTo],
  );
  useGameKeyboard(keyboardActions, pendingPromotion === null && !isNewGameOpen);

  if (loadError !== null) {
    return (
      <section className={styles.page}>
        <h1 className={styles.title}>{title}</h1>
        <p className={styles.error} role="alert">
          Non riesco a leggere la partita dal nucleo Rust: {loadError}
        </p>
      </section>
    );
  }

  if (session === null || game === null) {
    return (
      <section className={styles.page}>
        <h1 className={styles.title}>{title}</h1>
        <p className={styles.message}>Preparo la scacchiera…</p>
      </section>
    );
  }

  const { position, result } = game;
  /** L'analisi mostrata: solo se riguarda la posizione sulla scacchiera. */
  const analysis =
    analysisEnabled && latestAnalysis?.line.fen === position.fen ? latestAnalysis : null;
  const opponentSide: Side = orientation === 'white' ? 'black' : 'white';

  return (
    <section className={styles.page}>
      {/* Il titolo resta per i lettori di schermo, ma non occupa spazio. */}
      <h1 className={styles.hiddenTitle}>{title}</h1>

      <div className={styles.workspace}>
        <div className={classNames(styles.boardArea, coached && styles.coached)}>
          <PlayerBar {...describePlayer(session, opponentSide)} />
          <div className={styles.boardWrap}>
            {analysisEnabled && (
              // Accanto alla scacchiera, fuori dal suo spazio: comparendo non la rimpicciolisce.
              <div className={styles.evalSlot}>
                <EvalBar evaluation={analysis?.evaluation ?? null} orientation={orientation} />
              </div>
            )}
            <Chessboard
              boardFen={replayed?.boardFen ?? position.boardFen}
              turn={replayed?.turn ?? position.turn}
              inCheck={replayed?.inCheck ?? position.inCheck}
              orientation={orientation}
              lastMove={replayed === null ? game.lastMove : replayed.lastMove}
              legalMoves={session.canMove && replayed === null ? position.legalMoves : NO_MOVES}
              // Quando non si può muovere (tocca al computer, partita finita…)
              // la scacchiera è in sola lettura.
              onMove={session.canMove && replayed === null ? handleMove : undefined}
              arrow={replayed === null ? (analysis?.line.firstMove ?? null) : null}
              marks={
                replayed === null
                  ? boardMarks
                  : replayMoment === undefined
                    ? null
                    : { arrows: replayMoment.arrows, circles: replayMoment.squares }
              }
              resyncKey={resyncKey}
            />
            {result !== null && (
              <div className={styles.resultOverlay}>
                <GameResultBanner
                  result={result}
                  action={
                    <Button
                      onClick={() => {
                        void analyzeGame();
                      }}
                    >
                      Analizza la partita
                    </Button>
                  }
                />
              </div>
            )}
            {pendingPromotion !== null && (
              <PromotionPicker
                side={position.turn}
                onChoose={choosePromotion}
                onCancel={cancelPromotion}
              />
            )}
          </div>
          <PlayerBar {...describePlayer(session, orientation)} />
          {coached && (
            // Sotto la scacchiera, in uno spazio di altezza fissa: la scacchiera resta tutta visibile.
            <div className={styles.coachSlot}>
              {popup.isOpen ? (
                <CoachPopup
                  messages={coach.messages}
                  index={popup.index}
                  notation={notation}
                  onIndexChange={popup.setIndex}
                  onClose={popup.close}
                  onOpenCard={setCoachCardId}
                />
              ) : (
                <div className={styles.coachIdle}>
                  {coach.isLoading ? (
                    <p>Il tutor sta guardando la posizione…</p>
                  ) : coach.messages.length > 0 ? (
                    <Button onClick={popup.reopen}>Rivedi i commenti</Button>
                  ) : (
                    <p>Gioca una mossa: il tutor la commenterà qui.</p>
                  )}
                </div>
              )}
            </div>
          )}
          {actionError !== null && (
            <p className={styles.error} role="alert">
              {actionError}
            </p>
          )}
          {storageError !== null && (
            <div className={styles.notice} role="alert">
              <span>Archivio: {storageError}</span>
              <Button onClick={dismissStorageError}>Chiudi</Button>
            </div>
          )}
          {engineError !== null && (
            <div className={styles.notice} role="alert">
              <span>Stockfish: {engineError}</span>
              <Button onClick={dismissEngineError}>Chiudi</Button>
            </div>
          )}
        </div>

        <aside className={styles.sidePanel} aria-label="Partita">
          <TabSwitch
            label="Contenuto della colonna (tasto T)"
            options={SIDE_TABS}
            value={sideTab}
            onChange={setSideTab}
          />
          <div className={styles.sideContent}>
            {sideTab === 'tutor' ? (
              <TutorPanel />
            ) : (
              <>
                {coached && <CoachIdeasPanel ideas={coachIdeas} notation={notation} />}
                {coached && (
                  <CoachPanel
                    withEngine={coachWithEngine}
                    onWithEngineChange={setCoachWithEngine}
                    card={coachCard}
                    onCloseCard={() => {
                      setCoachCardId(null);
                    }}
                  />
                )}
                <PanelCard title="Mosse">
                  <OpeningLabel opening={session.opening} />
                  <MoveList
                    tree={game.tree}
                    notation={notation}
                    canPromote={session.canEditVariations}
                    onSelect={(node) => {
                      void goToNode(node);
                    }}
                    onPromote={(node) => {
                      void promoteVariation(node);
                    }}
                    onDelete={(node) => {
                      void deleteVariation(node);
                    }}
                  />
                  <GameControls
                    cursor={game.cursor}
                    totalMoves={game.moves.length}
                    canUndo={session.canUndo}
                    canResign={session.canResign}
                    onGoTo={(ply) => {
                      void goTo(ply);
                    }}
                    onNewGame={() => {
                      setIsNewGameOpen(true);
                    }}
                    onUndo={() => {
                      void undo();
                    }}
                    onResign={() => {
                      void resign();
                    }}
                    onFlip={() => {
                      setOrientation((side) => (side === 'white' ? 'black' : 'white'));
                    }}
                  />
                </PanelCard>

                <AnalysisPanel
                  enabled={analysisEnabled}
                  onToggle={(enabled) => {
                    void setAnalysisEnabled(enabled);
                  }}
                  update={analysis}
                  isGameOver={result !== null}
                  notation={notation}
                />

                <ObservationsPanel
                  enabled={observationsEnabled}
                  onToggle={setObservationsEnabled}
                  report={report}
                  notation={notation}
                  onHighlight={setHighlighted}
                />

                <PositionPanel position={position} />

                <PanelCard title="Copia e incolla">
                  <ImportExport fen={position.fen} onRequestPgn={exportPgn} onImport={importText} />
                </PanelCard>

                <PanelCard title="File PGN">
                  <PgnFileButtons onSave={savePgnFile} onOpen={openFile} />
                </PanelCard>

                <PanelCard title="Appunti">
                  <AddNoteButton link={playLink(session)} />
                </PanelCard>
              </>
            )}
          </div>
        </aside>
      </div>

      {isNewGameOpen && (
        <NewGameDialog
          initialSetup={gameSetup}
          onStart={(setup) => {
            void startGame(setup);
          }}
          onClose={() => {
            setIsNewGameOpen(false);
          }}
        />
      )}
    </section>
  );
}

/**
 * La sezione Tutor del menu: la pagina «Gioca» con i commenti del tutor.
 *
 * Riceve: niente.
 * Restituisce: la pagina.
 */
export function CoachedPlayPage() {
  return <PlayPage coached />;
}
