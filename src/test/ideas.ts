/**
 * Dati di prova delle idee del tutor (lo "sguardo lungo"), come li manderebbe Rust.
 */
import type { IdeaView } from '../bindings/IdeaView';
import type { MoveIdea } from '../bindings/MoveIdea';
import type { PositionIdeas } from '../bindings/PositionIdeas';

/** Un'idea del Bianco in due mosse: il cavallo va in f3 e poi in g5. */
export const KNIGHT_IDEA: IdeaView = {
  side: 'white',
  headline: "L'idea: il cavallo va g1–f3–g5 — si vede tra 2 mosse",
  keyPly: 2,
  evaluation: { kind: 'centipawns', value: 40 },
  moves: ['Nf3', 'Nc6', 'Ng5'],
  firstMoveNumber: 2,
  firstSide: 'white',
  moments: [
    {
      ply: 2,
      text: 'Il cavallo bianco in g1 va g1–f3–g5: si avvicina al re avversario',
      squares: ['g5'],
      arrows: [{ from: 'f3', to: 'g5' }],
      cardId: null,
    },
  ],
  frames: [
    { boardFen: 'start', turn: 'white', inCheck: false, lastMove: null },
    {
      boardFen: 'after-nf3',
      turn: 'black',
      inCheck: false,
      lastMove: { from: 'g1', to: 'f3', uci: 'g1f3' },
    },
    {
      boardFen: 'after-nc6',
      turn: 'white',
      inCheck: false,
      lastMove: { from: 'b8', to: 'c6', uci: 'b8c6' },
    },
    {
      boardFen: 'after-ng5',
      turn: 'black',
      inCheck: false,
      lastMove: { from: 'f3', to: 'g5', uci: 'f3g5' },
    },
  ],
  cards: [],
};

/** Le idee di una posizione: due varianti. */
export const POSITION_IDEAS: PositionIdeas = {
  fen: 'fen',
  ideas: [KNIGHT_IDEA, { ...KNIGHT_IDEA, headline: "L'idea: un piano tranquillo" }],
  comparison: 'Stockfish preferisce la prima idea: vale +0.4 per il Bianco, la seconda solo +0.1.',
};

/** L'idea di una mossa, con una mossa migliore. */
export const MOVE_IDEA_WITH_BETTER: MoveIdea = {
  played: { ...KNIGHT_IDEA, headline: "L'idea: un piano tranquillo" },
  better: KNIGHT_IDEA,
  comparison:
    "Stockfish preferiva un'altra mossa: dopo la tua la posizione vale +0.1 per il Bianco, con la sua +0.4.",
};
