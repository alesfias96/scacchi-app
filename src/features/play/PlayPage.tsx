/**
 * PlayPage — la sezione "Gioca": una partita contro il computer o a due giocatori.
 *
 * Cosa fa: chiede a Rust la partita in corso e la mostra. Si muovono i pezzi
 * trascinandoli o con due clic; sono permesse solo le mosse legali (arrocco,
 * presa en passant e promozione compresi), e solo quando Rust dice che si può
 * (ad esempio non mentre pensa il computer).
 * - Sopra e sotto la scacchiera: i due giocatori, con l'orologio se c'è.
 * - Accanto alla scacchiera, se l'analisi è accesa: la barra di valutazione;
 *   sulla scacchiera la freccia della mossa migliore.
 * - Nel pannello: l'annuncio di fine partita, le mosse con i comandi (scorri,
 *   nuova partita, annulla, abbandona, gira) con il nome dell'apertura,
 *   l'analisi, la situazione, copia e incolla di posizioni (FEN) e partite
 *   (PGN), salvataggio e apertura di file `.pgn`.
 * La partita si salva da sola nell'archivio a ogni mossa; quando si apre una
 * partita (dall'archivio o all'avvio) il proprio colore va in basso.
 * Le frecce ←/→ e i tasti Home/Fine scorrono la partita. Ogni mossa ha il suo
 * suono (mossa, presa, fine partita), se i suoni sono attivi nelle Impostazioni.
 *
 * Tutte le regole sono applicate da Rust: questa pagina invia le richieste e
 * mostra la partita aggiornata che riceve (anche quando cambia da sola, per
 * una mossa del computer o il tempo scaduto).
 *
 * Riceve: niente.
 * Restituisce: la pagina della sezione.
 */
import { useCallback, useEffect, useMemo, useState } from 'react';
import type { GameSetup } from '../../bindings/GameSetup';
import type { LegalMove } from '../../bindings/LegalMove';
import type { Side } from '../../bindings/Side';
import { Button } from '../../components/button/Button';
import { Chessboard } from '../../components/chessboard/Chessboard';
import { EvalBar } from '../../components/eval-bar/EvalBar';
import { PanelCard } from '../../components/panel-card/PanelCard';
import { useGameKeyboard } from '../../hooks/useGameKeyboard';
import { useMoveSound } from '../../hooks/useMoveSound';
import { exportPgn, savePgnFile } from '../../services/gameService';
import { useEngineStore } from '../../state/engineStore';
import { useGameStore } from '../../state/gameStore';
import { useLoadedSettings, useSettingsStore } from '../../state/settingsStore';
import { AnalysisPanel } from './analysis-panel/AnalysisPanel';
import { GameControls } from './game-controls/GameControls';
import { GameResultBanner } from './game-result-banner/GameResultBanner';
import { ImportExport } from './import-export/ImportExport';
import { MoveList } from './move-list/MoveList';
import { NewGameDialog } from './new-game-dialog/NewGameDialog';
import { OpeningLabel } from './opening-label/OpeningLabel';
import { PgnFileButtons } from './pgn-file/PgnFileButtons';
import { PlayerBar } from './player-bar/PlayerBar';
import { describePlayer } from './player-bar/describePlayer';
import { PositionPanel } from './position-panel/PositionPanel';
import { PromotionPicker, type PromotionPiece } from './promotion-picker/PromotionPicker';
import styles from './PlayPage.module.css';

/** Una promozione in attesa della scelta del pezzo: le due case della mossa. */
interface PendingPromotion {
  from: string;
  to: string;
}

/** Nessuna mossa possibile (quando non si può muovere). */
const NO_MOVES: readonly LegalMove[] = [];

export function PlayPage() {
  const { notation, sounds, gameSetup } = useLoadedSettings();
  const updateSettings = useSettingsStore((state) => state.update);
  const session = useGameStore((state) => state.session);
  const loadError = useGameStore((state) => state.loadError);
  const actionError = useGameStore((state) => state.actionError);
  const load = useGameStore((state) => state.load);
  const play = useGameStore((state) => state.play);
  const undo = useGameStore((state) => state.undo);
  const goTo = useGameStore((state) => state.goTo);
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

  /** Il colore mostrato in basso sulla scacchiera. */
  const [orientation, setOrientation] = useState<Side>('white');
  /** La promozione in attesa della scelta del pezzo, oppure `null`. */
  const [pendingPromotion, setPendingPromotion] = useState<PendingPromotion | null>(null);
  /** Cambiandolo, la scacchiera torna alla posizione ricevuta da Rust. */
  const [resyncKey, setResyncKey] = useState(0);
  /** `true` mentre è aperta la finestra "Nuova partita". */
  const [isNewGameOpen, setIsNewGameOpen] = useState(false);

  // All'apertura della sezione legge la partita in corso e ascolta il motore.
  useEffect(() => {
    void load();
    void listenToEngine();
  }, [load, listenToEngine]);

  const game = session?.game ?? null;
  useMoveSound(game, sounds === 'on');

  // A ogni partita diversa (nuova, ripresa all'avvio, aperta dall'archivio)
  // il colore della persona va in basso: il nero se il computer ha il bianco.
  const sessionId = session?.id;
  const computerSide = session?.computer?.side;
  useEffect(() => {
    if (sessionId !== undefined) {
      setOrientation(computerSide === 'white' ? 'black' : 'white');
    }
  }, [sessionId, computerSide]);

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
   * e spegne l'analisi contro il computer (sarebbe un aiuto).
   */
  const startGame = useCallback(
    async (setup: GameSetup) => {
      setIsNewGameOpen(false);
      void updateSettings({ gameSetup: setup });
      if (setup.opponent === 'computer' && useEngineStore.getState().analysisEnabled) {
        void setAnalysisEnabled(false);
      }
      await newGame(setup);
    },
    [updateSettings, setAnalysisEnabled, newGame],
  );

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
        <h1 className={styles.title}>Gioca</h1>
        <p className={styles.error} role="alert">
          Non riesco a leggere la partita dal nucleo Rust: {loadError}
        </p>
      </section>
    );
  }

  if (session === null || game === null) {
    return (
      <section className={styles.page}>
        <h1 className={styles.title}>Gioca</h1>
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
      <h1 className={styles.title}>Gioca</h1>

      <div className={styles.workspace}>
        <div className={styles.boardArea}>
          <PlayerBar {...describePlayer(session, opponentSide)} />
          <div className={styles.boardRow}>
            {analysisEnabled && (
              <EvalBar evaluation={analysis?.evaluation ?? null} orientation={orientation} />
            )}
            <div className={styles.boardWrap}>
              <Chessboard
                boardFen={position.boardFen}
                turn={position.turn}
                inCheck={position.inCheck}
                orientation={orientation}
                lastMove={game.lastMove}
                legalMoves={session.canMove ? position.legalMoves : NO_MOVES}
                // Quando non si può muovere (tocca al computer, partita finita…)
                // la scacchiera è in sola lettura.
                onMove={session.canMove ? handleMove : undefined}
                arrow={analysis?.line.firstMove ?? null}
                resyncKey={resyncKey}
              />
              {pendingPromotion !== null && (
                <PromotionPicker
                  side={position.turn}
                  onChoose={choosePromotion}
                  onCancel={cancelPromotion}
                />
              )}
            </div>
          </div>
          <PlayerBar {...describePlayer(session, orientation)} />
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

        <div className={styles.sidePanel}>
          {result !== null && <GameResultBanner result={result} />}

          <PanelCard title="Mosse">
            <OpeningLabel opening={session.opening} />
            <MoveList
              moves={game.moves}
              cursor={game.cursor}
              notation={notation}
              onSelect={(ply) => {
                void goTo(ply);
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

          <PositionPanel position={position} />

          <PanelCard title="Copia e incolla">
            <ImportExport fen={position.fen} onRequestPgn={exportPgn} onImport={importText} />
          </PanelCard>

          <PanelCard title="File PGN">
            <PgnFileButtons onSave={savePgnFile} onOpen={openFile} />
          </PanelCard>
        </div>
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
