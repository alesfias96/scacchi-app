/**
 * Puzzle di prova per i test della sezione "Allenamento" (non sono usati dall'app).
 *
 * Il Nero ha appena giocato h6; il Bianco deve trovare Dxf7#.
 */
import type { AttemptView } from '../bindings/AttemptView';

export const READY_ATTEMPT: AttemptView = {
  puzzleId: 'TEST1',
  mode: 'depth',
  game: {
    position: {
      fen: 'r1bqkbnr/pppp1pp1/2n4p/4p3/2B1P3/5Q2/PPPP1PPP/RNB1K1NR w KQkq - 0 4',
      boardFen: 'r1bqkbnr/pppp1pp1/2n4p/4p3/2B1P3/5Q2/PPPP1PPP/RNB1K1NR',
      turn: 'white',
      inCheck: false,
      legalMoves: [
        { from: 'f3', to: 'f7', uci: 'f3f7' },
        { from: 'f3', to: 'f6', uci: 'f3f6' },
      ],
    },
    moves: [
      {
        ply: 1,
        moveNumber: 3,
        side: 'black',
        san: 'h6',
        uci: 'h7h6',
        from: 'h7',
        to: 'h6',
        capture: false,
      },
    ],
    cursor: 1,
    lastMove: { from: 'h7', to: 'h6', uci: 'h7h6' },
    result: null,
    startingFen: 'r1bqkbnr/pppp1ppp/2n5/4p3/2B1P3/5Q2/PPPP1PPP/RNB1K1NR b KQkq - 3 3',
    tree: { nodes: [], roots: [], current: null },
  },
  solver: 'white',
  orientation: 'white',
  depth: 1,
  found: 0,
  feedback: 'ready',
  hintSquare: null,
  finished: false,
  clean: false,
  isReview: false,
  rating: 1500,
  ratingChange: null,
  reveal: null,
};

/** Lo stesso puzzle, risolto al primo colpo. */
export const SOLVED_ATTEMPT: AttemptView = {
  ...READY_ATTEMPT,
  found: 1,
  feedback: 'solved',
  finished: true,
  clean: true,
  rating: 1516,
  ratingChange: 16,
  reveal: {
    puzzleRating: 1480,
    themes: ['Matto in 1'],
    solution: {
      fen: READY_ATTEMPT.game.position.fen,
      turn: 'white',
      moveNumber: 4,
      san: ['Qxf7#'],
      firstMove: { from: 'f3', to: 'f7', uci: 'f3f7' },
    },
    facts: [],
  },
};
