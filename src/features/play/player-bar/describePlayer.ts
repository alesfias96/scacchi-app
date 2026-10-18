/**
 * Prepara cosa mostrare nella striscia di un giocatore, a partire dalla partita.
 *
 * - Contro il computer: "Stockfish" con il livello per il computer, "Tu" per la persona.
 * - A due giocatori: "Bianco" e "Nero".
 * - Il tempo del giocatore, se la partita ha l'orologio, e se sta scorrendo.
 * - "Sta pensando" mentre il computer cerca la sua mossa.
 *
 * @param session - la partita descritta da Rust.
 * @param side - il colore del giocatore da descrivere.
 * @returns le proprietà per `PlayerBar`.
 */
import type { SessionView } from '../../../bindings/SessionView';
import type { Side } from '../../../bindings/Side';
import type { PlayerBarProps } from './PlayerBar';

/** Il nome di ciascun colore. */
const SIDE_NAMES: Record<Side, string> = {
  white: 'Bianco',
  black: 'Nero',
};

export function describePlayer(session: SessionView, side: Side): PlayerBarProps {
  const { computer, clock } = session;
  const isComputer = computer?.side === side;
  return {
    side,
    name: computer === null ? SIDE_NAMES[side] : isComputer ? 'Stockfish' : 'Tu',
    detail:
      computer !== null && isComputer
        ? `${computer.level.name} · ~${String(computer.level.elo)} Elo`
        : undefined,
    clock:
      clock === null
        ? null
        : { ms: side === 'white' ? clock.whiteMs : clock.blackMs, running: clock.running === side },
    thinking: isComputer && session.computerThinking,
  };
}
