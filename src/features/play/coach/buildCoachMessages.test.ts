/**
 * Test della scelta dei messaggi del tutor dopo una mossa.
 */
import { describe, expect, it } from 'vitest';
import type { Fact } from '../../../bindings/Fact';
import type { MoveRecord } from '../../../bindings/MoveRecord';
import type { TutorExplanation } from '../../../bindings/TutorExplanation';
import { AFTER_E4_SCENARIO, START_EXPLANATION } from '../../../test/tutorExplanations';
import { buildCoachMessages, messageKey } from './buildCoachMessages';

const E4: MoveRecord = {
  ply: 1,
  moveNumber: 1,
  side: 'white',
  san: 'e4',
  uci: 'e2e4',
  from: 'e2',
  to: 'e4',
  capture: false,
};

const THREAT: Fact = {
  category: 'threats',
  side: 'black',
  text: 'Il pedone bianco in e4 è in presa',
  squares: ['e4'],
  arrows: [],
  moves: [],
};

/** La spiegazione di prova con una minaccia in "Cosa succede adesso". */
const WITH_THREAT: TutorExplanation = {
  ...START_EXPLANATION,
  sections: [
    ...START_EXPLANATION.sections,
    { title: 'Cosa succede adesso', items: [{ fact: THREAT, cardId: null }] },
  ],
};

describe('buildCoachMessages', () => {
  it('comincia dal commento alla mossa, con il giudizio e la freccia della mossa', () => {
    const { messages } = buildCoachMessages(
      START_EXPLANATION,
      [{ record: E4, scenario: AFTER_E4_SCENARIO }],
      new Set(),
      'italiana',
    );

    expect(messages[0]?.heading).toBe('La mossa 1. e4');
    expect(messages[0]?.fact.text).toBe(
      'Ottima scelta: è tra le mosse migliori (valutazione da +0.3 a +0.4)',
    );
    expect(messages[0]?.fact.arrows).toEqual([{ from: 'e2', to: 'e4' }]);
  });

  it('la prima volta mostra tutta la situazione, poi solo le novità', () => {
    const first = buildCoachMessages(START_EXPLANATION, [], new Set(), 'italiana');
    expect(first.messages.map((message) => message.fact.text)).toEqual([
      'Tocca al Bianco',
      'Il Bianco ha più pedoni al centro',
    ]);

    const again = buildCoachMessages(START_EXPLANATION, [], new Set(first.keys), 'italiana');
    expect(again.messages).toEqual([]);
  });

  it('le minacce si ripetono sempre, prima delle novità', () => {
    const seen = new Set([messageKey(THREAT)]);

    const { messages } = buildCoachMessages(WITH_THREAT, [], seen, 'italiana');

    expect(messages[0]?.fact).toEqual(THREAT);
    expect(messages[0]?.heading).toBe('Cosa succede adesso');
  });

  it('tiene la scheda da approfondire', () => {
    const { messages } = buildCoachMessages(START_EXPLANATION, [], new Set(), 'italiana');

    expect(messages[1]?.cardId).toBe('principles/center');
  });
});
