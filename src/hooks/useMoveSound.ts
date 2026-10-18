/**
 * useMoveSound — fa sentire i suoni della partita quando la scacchiera avanza.
 *
 * Cosa fa: ogni volta che arriva da Rust una nuova versione della partita la
 * confronta con quella precedente; se è stata giocata (o si è avanzati di) una
 * mossa, suona il colpo della mossa o della presa e, se la partita è finita,
 * la melodia di fine partita. Quali suoni far sentire lo decide
 * `pickGameSounds`; qui si tiene solo il ricordo della partita precedente.
 *
 * Riceve:
 * - `game`: la partita mostrata (o `null` se non è ancora arrivata);
 * - `enabled`: se i suoni sono attivi nelle Impostazioni.
 * Restituisce: niente.
 */
import { useEffect, useRef } from 'react';
import type { GameView } from '../bindings/GameView';
import { pickGameSounds } from '../sounds/pickGameSounds';
import { playCaptureSound } from '../sounds/playCaptureSound';
import { playGameEndSound } from '../sounds/playGameEndSound';
import { playMoveSound } from '../sounds/playMoveSound';

/** Dopo quanti secondi dal colpo della mossa parte la melodia di fine partita. */
const GAME_END_DELAY = 0.18;

export function useMoveSound(game: GameView | null, enabled: boolean) {
  /** La partita mostrata la volta precedente. */
  const previousRef = useRef<GameView | null>(null);

  useEffect(() => {
    if (game === null) {
      return;
    }
    const previous = previousRef.current;
    previousRef.current = game;
    if (!enabled) {
      return;
    }
    const sounds = pickGameSounds(previous, game);
    if (sounds.move === 'capture') {
      playCaptureSound();
    } else if (sounds.move === 'move') {
      playMoveSound();
    }
    if (sounds.end !== null) {
      playGameEndSound(sounds.end, GAME_END_DELAY);
    }
  }, [game, enabled]);
}
