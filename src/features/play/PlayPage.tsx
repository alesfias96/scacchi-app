/**
 * PlayPage — la sezione "Gioca": una partita a due giocatori sulla stessa scacchiera.
 *
 * Cosa fa: chiede a Rust la partita in corso e la mostra. Si muovono i pezzi
 * trascinandoli o con due clic; sono permesse solo le mosse legali (arrocco,
 * presa en passant e promozione compresi). Accanto alla scacchiera:
 * - l'annuncio di fine partita, quando la partita è finita;
 * - la situazione (chi muove, mosse possibili, scacco);
 * - l'elenco delle mosse con i comandi (scorri, nuova partita, annulla, gira);
 * - copia e incolla di posizioni (FEN) e partite (PGN).
 * Le frecce ←/→ e i tasti Home/Fine scorrono la partita.
 *
 * Tutte le regole sono applicate da Rust: questa pagina invia le mosse e
 * mostra la partita aggiornata che riceve.
 *
 * Riceve: niente.
 * Restituisce: la pagina della sezione.
 */
import { useCallback, useEffect, useMemo, useState } from 'react';
import type { LegalMove } from '../../bindings/LegalMove';
import type { Side } from '../../bindings/Side';
import { Chessboard } from '../../components/chessboard/Chessboard';
import { PanelCard } from '../../components/panel-card/PanelCard';
import { useGameKeyboard } from '../../hooks/useGameKeyboard';
import { exportPgn } from '../../services/gameService';
import { useGameStore } from '../../state/gameStore';
import { useLoadedSettings } from '../../state/settingsStore';
import { GameControls } from './game-controls/GameControls';
import { GameResultBanner } from './game-result-banner/GameResultBanner';
import { ImportExport } from './import-export/ImportExport';
import { MoveList } from './move-list/MoveList';
import { PositionPanel } from './position-panel/PositionPanel';
import { PromotionPicker, type PromotionPiece } from './promotion-picker/PromotionPicker';
import styles from './PlayPage.module.css';

/** Una promozione in attesa della scelta del pezzo: le due case della mossa. */
interface PendingPromotion {
  from: string;
  to: string;
}

/** Nessuna mossa possibile (a partita finita). */
const NO_MOVES: readonly LegalMove[] = [];

export function PlayPage() {
  const { notation } = useLoadedSettings();
  const game = useGameStore((state) => state.game);
  const loadError = useGameStore((state) => state.loadError);
  const actionError = useGameStore((state) => state.actionError);
  const load = useGameStore((state) => state.load);
  const play = useGameStore((state) => state.play);
  const undo = useGameStore((state) => state.undo);
  const goTo = useGameStore((state) => state.goTo);
  const newGame = useGameStore((state) => state.newGame);
  const importText = useGameStore((state) => state.importText);

  /** Il colore mostrato in basso sulla scacchiera. */
  const [orientation, setOrientation] = useState<Side>('white');
  /** La promozione in attesa della scelta del pezzo, oppure `null`. */
  const [pendingPromotion, setPendingPromotion] = useState<PendingPromotion | null>(null);
  /** Cambiandolo, la scacchiera torna alla posizione ricevuta da Rust. */
  const [resyncKey, setResyncKey] = useState(0);

  // All'apertura della sezione legge la partita in corso.
  useEffect(() => {
    void load();
  }, [load]);

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
  useGameKeyboard(keyboardActions, pendingPromotion === null);

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

  if (game === null) {
    return (
      <section className={styles.page}>
        <h1 className={styles.title}>Gioca</h1>
        <p className={styles.message}>Preparo la scacchiera…</p>
      </section>
    );
  }

  const { position, result } = game;

  return (
    <section className={styles.page}>
      <h1 className={styles.title}>Gioca</h1>

      <div className={styles.workspace}>
        <div className={styles.boardArea}>
          <Chessboard
            boardFen={position.boardFen}
            turn={position.turn}
            inCheck={position.inCheck}
            orientation={orientation}
            lastMove={game.lastMove}
            // A partita finita (anche per patta) non si muove più.
            legalMoves={result === null ? position.legalMoves : NO_MOVES}
            onMove={handleMove}
            resyncKey={resyncKey}
          />
          {pendingPromotion !== null && (
            <PromotionPicker
              side={position.turn}
              onChoose={choosePromotion}
              onCancel={cancelPromotion}
            />
          )}
          {actionError !== null && (
            <p className={styles.error} role="alert">
              {actionError}
            </p>
          )}
        </div>

        <div className={styles.sidePanel}>
          {result !== null && <GameResultBanner result={result} />}

          <PositionPanel position={position} />

          <PanelCard title="Mosse">
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
              onGoTo={(ply) => {
                void goTo(ply);
              }}
              onNewGame={() => {
                void newGame();
              }}
              onUndo={() => {
                void undo();
              }}
              onFlip={() => {
                setOrientation((side) => (side === 'white' ? 'black' : 'white'));
              }}
            />
          </PanelCard>

          <PanelCard title="Copia e incolla">
            <ImportExport fen={position.fen} onRequestPgn={exportPgn} onImport={importText} />
          </PanelCard>
        </div>
      </div>
    </section>
  );
}
