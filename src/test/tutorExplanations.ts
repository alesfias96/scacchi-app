/**
 * Spiegazioni di prova per i test del tutor (non sono usate dall'app).
 *
 * La posizione è quella iniziale, con una spiegazione corta e una scheda.
 */
import type { Fact } from '../bindings/Fact';
import type { TutorExplanation } from '../bindings/TutorExplanation';
import type { WhatIfView } from '../bindings/WhatIfView';

const START_FEN = 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1';

/** Una frase semplice. */
function fact(text: string, squares: string[] = []): Fact {
  return { category: 'activity', side: 'white', text, squares, arrows: [], moves: [] };
}

export const START_EXPLANATION: TutorExplanation = {
  fen: START_FEN,
  position: {
    fen: START_FEN,
    boardFen: 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR',
    turn: 'white',
    inCheck: false,
    legalMoves: [{ from: 'e2', to: 'e4', uci: 'e2e4' }],
  },
  turn: 'white',
  sections: [
    {
      title: 'La situazione',
      items: [
        { fact: fact('Tocca al Bianco'), cardId: null },
        { fact: fact('Il Bianco ha più pedoni al centro', ['e4']), cardId: 'principles/center' },
      ],
    },
  ],
  cards: [
    {
      id: 'principles/center',
      topic: 'principle',
      title: 'Il controllo del centro',
      context: 'Il Bianco ha più pedoni al centro',
      summary: 'Le case centrali sono le più importanti.',
      ideas: ['I pedoni al centro danno spazio.'],
      plans: [{ label: 'Chi ha più pedoni al centro (il Bianco)', steps: ['Sostieni i pedoni.'] }],
      mistakes: ['Avanzare troppo presto.'],
    },
  ],
};

/** Lo scenario dopo 1.e4, con il giudizio di Stockfish. */
export const AFTER_E4_SCENARIO: WhatIfView = {
  san: 'e4',
  mover: 'white',
  position: {
    fen: 'rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1',
    boardFen: 'rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR',
    turn: 'black',
    inCheck: false,
    legalMoves: [{ from: 'e7', to: 'e5', uci: 'e7e5' }],
  },
  lastMove: { from: 'e2', to: 'e4', uci: 'e2e4' },
  verdict: 'Ottima scelta: è tra le mosse migliori',
  evaluationBefore: { kind: 'centipawns', value: 30 },
  evaluationAfter: { kind: 'centipawns', value: 40 },
  bestReply: null,
  sections: [],
};
