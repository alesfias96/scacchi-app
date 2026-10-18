/**
 * Chessboard — la scacchiera grafica, usata in tutte le sezioni dell'app.
 *
 * Cosa fa: disegna una posizione usando chessground, la scacchiera di Lichess
 * (pezzi che scivolano, coordinate, evidenziazione dello scacco), dentro una
 * cornice. Questo componente è un "involucro": traduce le proprietà React
 * nella configurazione di chessground, così il resto dell'app non dipende
 * direttamente dalla libreria.
 *
 * Lo stile della scacchiera e dei pezzi NON si passa come proprietà: il
 * componente lo legge da solo dalle impostazioni dell'utente. Così ogni
 * scacchiera dell'app, in qualsiasi sezione, rispetta sempre le scelte fatte
 * nelle Impostazioni e si aggiorna subito quando cambiano.
 *
 * Riceve:
 * - `boardFen`: la disposizione dei pezzi (prima parte della FEN);
 * - `turn`: il colore che deve muovere;
 * - `inCheck`: se il re di turno è sotto scacco (la sua casa viene evidenziata);
 * - `orientation`: il lato della scacchiera in basso (predefinito: bianco);
 * - `lastMove`: facoltativa, l'ultima mossa giocata, evidenziata sulle sue due case;
 * - `legalMoves` e `onMove`: facoltativi. Se c'è `onMove` la scacchiera è
 *   interattiva: il giocatore di turno può trascinare (o cliccare) i suoi
 *   pezzi, ma solo verso le case permesse da `legalMoves`. A mossa fatta
 *   viene chiamato `onMove` con casa di partenza e di arrivo. I pezzi che si
 *   possono muovere mostrano la "mano" al passaggio del cursore. Senza `onMove`
 *   la scacchiera è in sola lettura;
 * - `resyncKey`: facoltativo. Cambiarlo riporta i pezzi alla posizione
 *   ricevuta, ad esempio dopo una promozione annullata (quando il pedone è
 *   già stato spostato sulla scacchiera ma la mossa non è stata giocata).
 * Restituisce: la scacchiera con la cornice, quadrata, larga quanto il suo contenitore.
 */
import { Chessground } from '@lichess-org/chessground';
import type { Api } from '@lichess-org/chessground/api';
import type { Config } from '@lichess-org/chessground/config';
import { useEffect, useRef } from 'react';
import type { Key } from '@lichess-org/chessground/types';
import type { LegalMove } from '../../bindings/LegalMove';
import type { Side } from '../../bindings/Side';
import { useLoadedSettings } from '../../state/settingsStore';
import { buildDests } from './buildDests';
import styles from './Chessboard.module.css';
import { isSquareKey } from './isSquareKey';
import { prefersReducedMotion } from './prefersReducedMotion';

// Stili globali della scacchiera: unica eccezione alla regola dei CSS Modules
// (vedi README di questa cartella).
// - base: la struttura della scacchiera (fornito da chessground);
// - brown: i colori di coordinate, ultima mossa e case evidenziate (fornito da chessground;
//   le sue case marroni sono coperte dalle nostre immagini);
// - themes: le nostre scacchiere e i nostri set di pezzi.
import '@lichess-org/chessground/assets/chessground.base.css';
import '@lichess-org/chessground/assets/chessground.brown.css';
import './themes/index.css';

export interface ChessboardProps {
  /** Disposizione dei pezzi in notazione FEN (solo la prima parte). */
  boardFen: string;
  /** Il colore che deve muovere. */
  turn: Side;
  /** `true` se il re di turno è sotto scacco. */
  inCheck: boolean;
  /** Il lato mostrato in basso. Predefinito: `"white"`. */
  orientation?: Side;
  /** L'ultima mossa giocata, da evidenziare; `null` o assente se non c'è. */
  lastMove?: LegalMove | null;
  /** Le mosse permesse al giocatore di turno (servono solo con `onMove`). */
  legalMoves?: readonly LegalMove[];
  /** Chiamata quando l'utente sposta un pezzo; se assente la scacchiera è in sola lettura. */
  onMove?: (from: string, to: string) => void;
  /** Cambiarlo riporta i pezzi alla posizione ricevuta (vedi descrizione sopra). */
  resyncKey?: number;
}

/**
 * Durata dell'animazione di un pezzo che si sposta, in millisecondi: abbastanza
 * da seguire il movimento con l'occhio, abbastanza breve da non far aspettare.
 */
const MOVE_ANIMATION_MS = 220;

/** Nessuna mossa possibile: valore usato quando `legalMoves` non viene passato. */
const NO_MOVES: readonly LegalMove[] = [];

export function Chessboard({
  boardFen,
  turn,
  inCheck,
  orientation = 'white',
  lastMove = null,
  legalMoves = NO_MOVES,
  onMove,
  resyncKey = 0,
}: ChessboardProps) {
  /** Lo stile di scacchiera e pezzi scelto nelle Impostazioni. */
  const { board, pieces } = useLoadedSettings();
  /** L'elemento della pagina dentro cui chessground disegna la scacchiera. */
  const containerRef = useRef<HTMLDivElement>(null);
  /** Il "telecomando" di chessground, per aggiornare la scacchiera già disegnata. */
  const boardApiRef = useRef<Api | null>(null);
  /**
   * La funzione `onMove` più recente. chessground la chiama a mossa fatta;
   * tenerla in un riferimento evita di riconfigurare la scacchiera ogni
   * volta che il componente genitore crea una nuova funzione.
   */
  const onMoveRef = useRef(onMove);
  useEffect(() => {
    onMoveRef.current = onMove;
  }, [onMove]);

  /** `true` se l'utente può muovere i pezzi. */
  const isInteractive = onMove !== undefined;
  /** Le due case dell'ultima mossa (testi semplici, per confrontarle facilmente). */
  const lastFrom = lastMove?.from;
  const lastTo = lastMove?.to;

  // Crea la scacchiera al primo disegno e la aggiorna quando cambiano le proprietà.
  // `resyncKey` è tra le dipendenze apposta: cambiandolo si riapplica la posizione.
  useEffect(() => {
    /** Le case da evidenziare, solo se entrambe valide. */
    const lastMoveKeys: Key[] | undefined =
      lastFrom !== undefined && lastTo !== undefined && isSquareKey(lastFrom) && isSquareKey(lastTo)
        ? [lastFrom, lastTo]
        : undefined;

    const config: Config = {
      fen: boardFen,
      turnColor: turn,
      check: inCheck,
      orientation,
      coordinates: true,
      viewOnly: !isInteractive,
      // Pezzi che scivolano da una casa all'altra; spenti se il sistema chiede
      // di ridurre le animazioni.
      animation: { enabled: !prefersReducedMotion(), duration: MOVE_ANIMATION_MS },
      // `lastMove` va sempre indicato, anche vuoto: così chessground toglie
      // l'evidenziazione quando si torna alla posizione di partenza.
      lastMove: lastMoveKeys,
      movable: {
        // Si muovono solo i pezzi di chi è di turno, solo verso case legali.
        free: false,
        color: isInteractive ? turn : undefined,
        dests: isInteractive ? buildDests(legalMoves) : new Map(),
        showDests: true,
        events: {
          after: (from, to) => {
            onMoveRef.current?.(from, to);
          },
        },
      },
    };

    if (boardApiRef.current !== null) {
      boardApiRef.current.set(config);
    } else if (containerRef.current !== null) {
      boardApiRef.current = Chessground(containerRef.current, config);
    }
  }, [
    boardFen,
    turn,
    inCheck,
    orientation,
    isInteractive,
    legalMoves,
    lastFrom,
    lastTo,
    resyncKey,
  ]);

  // Quando il componente viene rimosso, libera le risorse di chessground.
  useEffect(() => {
    return () => {
      boardApiRef.current?.destroy();
      boardApiRef.current = null;
    };
  }, []);

  // data-board e data-pieces attivano gli stili scelti (vedi themes/);
  // data-movable indica il colore che può muovere (cursore a mano sui suoi pezzi).
  return (
    <div
      className={styles.frame}
      data-board={board}
      data-pieces={pieces}
      data-movable={isInteractive ? turn : undefined}
    >
      <div ref={containerRef} className={styles.board} />
    </div>
  );
}
