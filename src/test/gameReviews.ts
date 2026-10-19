/**
 * Una revisione di prova per i test della sezione «Analisi partite» (non è usata dall'app).
 *
 * La partita è 1.e4 e5 2.Dh5 Cf6??, con l'errore grave del Nero alla seconda mossa.
 */
import type { GameReview } from '../bindings/GameReview';
import type { PositionView } from '../bindings/PositionView';

/** Una posizione semplice (bastano le parti usate dalla scacchiera). */
function position(boardFen: string, turn: 'white' | 'black'): PositionView {
  return {
    fen: `${boardFen} ${turn === 'white' ? 'w' : 'b'} - - 0 1`,
    boardFen,
    turn,
    inCheck: false,
    legalMoves: [],
  };
}

export const SAMPLE_REVIEW: GameReview = {
  startFen: 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1',
  positions: [
    position('rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR', 'white'),
    position('rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR', 'black'),
    position('rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR', 'white'),
    position('rnbqkbnr/pppp1ppp/8/4p2Q/4P3/8/PPPP1PPP/RNB1KBNR', 'black'),
    position('rnbqkb1r/pppp1ppp/5n2/4p2Q/4P3/8/PPPP1PPP/RNB1KBNR', 'white'),
  ],
  moves: [
    {
      ply: 1,
      moveNumber: 1,
      side: 'white',
      san: 'e4',
      played: { from: 'e2', to: 'e4', uci: 'e2e4' },
      quality: 'best',
      chanceLost: 0,
      evaluationBefore: { kind: 'centipawns', value: 30 },
      evaluationAfter: { kind: 'centipawns', value: 30 },
      best: { from: 'e2', to: 'e4', uci: 'e2e4' },
      bestSan: 'e4',
      phase: 'opening',
    },
    {
      ply: 2,
      moveNumber: 1,
      side: 'black',
      san: 'e5',
      played: { from: 'e7', to: 'e5', uci: 'e7e5' },
      quality: 'best',
      chanceLost: 0,
      evaluationBefore: { kind: 'centipawns', value: 30 },
      evaluationAfter: { kind: 'centipawns', value: 30 },
      best: { from: 'e7', to: 'e5', uci: 'e7e5' },
      bestSan: 'e5',
      phase: 'opening',
    },
    {
      ply: 3,
      moveNumber: 2,
      side: 'white',
      san: 'Qh5',
      played: { from: 'd1', to: 'h5', uci: 'd1h5' },
      quality: 'good',
      chanceLost: 3,
      evaluationBefore: { kind: 'centipawns', value: 30 },
      evaluationAfter: { kind: 'centipawns', value: 0 },
      best: { from: 'g1', to: 'f3', uci: 'g1f3' },
      bestSan: 'Nf3',
      phase: 'opening',
    },
    {
      ply: 4,
      moveNumber: 2,
      side: 'black',
      san: 'Nf6',
      played: { from: 'g8', to: 'f6', uci: 'g8f6' },
      quality: 'blunder',
      chanceLost: 45,
      evaluationBefore: { kind: 'centipawns', value: 0 },
      evaluationAfter: { kind: 'centipawns', value: 500 },
      best: { from: 'b8', to: 'c6', uci: 'b8c6' },
      bestSan: 'Nc6',
      phase: 'opening',
    },
  ],
  whiteChances: [53, 53, 53, 50, 95],
  white: { accuracy: 97, inaccuracies: 0, mistakes: 0, blunders: 0, worstPhase: null },
  black: { accuracy: 60, inaccuracies: 0, mistakes: 0, blunders: 1, worstPhase: 'opening' },
  critical: [
    {
      ply: 4,
      quality: 'blunder',
      text: 'Errore grave: la probabilità di vincere del Nero scende dal 50% al 5%',
      playedSan: 'Nf6',
      bestSan: 'Nc6',
      conceded: [
        {
          category: 'threats',
          side: 'white',
          text: 'Il pedone nero in e5 è in presa',
          squares: ['e5'],
          arrows: [],
          moves: [],
        },
      ],
      bestGains: [],
    },
  ],
};
