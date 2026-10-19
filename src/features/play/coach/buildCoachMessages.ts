/**
 * Sceglie i messaggi del tutor da mostrare, uno alla volta, dopo una mossa.
 *
 * L'ordine:
 * 1. il **commento a ogni mossa appena giocata** (di solito la propria e la
 *    risposta dell'avversario): il giudizio di Stockfish, se c'è, poi cosa la
 *    mossa concede, cosa crea, quali minacce risolve;
 * 2. le **minacce e le tattiche** della posizione attuale (sempre);
 * 3. **solo le novità** della spiegazione (situazione, piano, principi):
 *    le frasi già mostrate alle mosse precedenti non si ripetono.
 *
 * @param explanation - la spiegazione della posizione attuale (da Rust).
 * @param comments - le mosse appena giocate, ognuna con il suo scenario (da Rust).
 * @param seen - le frasi già mostrate prima (vedi `messageKey`).
 * @param notation - la notazione scelta nelle Impostazioni.
 * @returns i messaggi, in ordine, e le chiavi delle frasi mostrate (da aggiungere a `seen`).
 */
import type { Fact } from '../../../bindings/Fact';
import type { LineView } from '../../../bindings/LineView';
import type { MoveRecord } from '../../../bindings/MoveRecord';
import type { Notation } from '../../../bindings/Notation';
import type { TutorExplanation } from '../../../bindings/TutorExplanation';
import type { TutorItem } from '../../../bindings/TutorItem';
import type { WhatIfView } from '../../../bindings/WhatIfView';
import { formatEvaluation } from '../../../lib/formatEvaluation';
import { formatSan } from '../../../lib/formatSan';

/** Una mossa appena giocata e cosa ha cambiato. */
export interface MoveComment {
  /** La mossa. */
  record: MoveRecord;
  /** Lo scenario calcolato da Rust per quella mossa. */
  scenario: WhatIfView;
}

/** Un messaggio del tutor, mostrato in un popup sopra la scacchiera. */
export interface CoachMessage {
  /** Il titoletto (es. "La mossa 12. Cf3" o "Il piano"). */
  heading: string;
  /** La frase, con le case e le frecce da mostrare sulla scacchiera. */
  fact: Fact;
  /** Una sequenza di mosse da scrivere sotto la frase (la risposta migliore), o `null`. */
  line: LineView | null;
  /** La scheda della base di conoscenza che approfondisce, o `null`. */
  cardId: string | null;
}

/** Il titolo della parte con le minacce attuali (lo stesso usato da Rust). */
const NOW_SECTION = 'Cosa succede adesso';
/** La frase che dice chi muove: utile solo la prima volta. */
const TURN_PREFIX = 'Tocca al';

/** La chiave che identifica una frase, per non ripeterla. */
export function messageKey(fact: Fact): string {
  return `${fact.text}|${fact.moves.join(' ')}`;
}

/** Il numero di una mossa come nei libri: "12." per il Bianco, "12…" per il Nero. */
function moveLabel(record: MoveRecord, notation: Notation): string {
  const number = `${String(record.moveNumber)}${record.side === 'white' ? '.' : '…'}`;
  return `${number} ${formatSan(record.san, notation)}`;
}

/** I messaggi del commento a una mossa. */
function commentMessages({ record, scenario }: MoveComment, notation: Notation): CoachMessage[] {
  const heading = `La mossa ${moveLabel(record, notation)}`;
  const { evaluationBefore, evaluationAfter } = scenario;
  const evaluation =
    evaluationBefore !== null && evaluationAfter !== null
      ? ` (valutazione da ${formatEvaluation(evaluationBefore)} a ${formatEvaluation(evaluationAfter)})`
      : '';
  const main: CoachMessage = {
    heading,
    fact: {
      category: 'activity',
      side: record.side,
      text: `${scenario.verdict ?? 'Ecco cosa cambia con questa mossa'}${evaluation}`,
      squares: [],
      arrows: record.from === '' ? [] : [{ from: record.from, to: record.to }],
      moves: [],
    },
    line: scenario.bestReply,
    cardId: null,
  };
  const details = scenario.sections.flatMap((section) =>
    section.items.map((item) => ({
      heading: `${heading} · ${section.title}`,
      fact: item.fact,
      line: null,
      cardId: item.cardId,
    })),
  );
  return [main, ...details];
}

/** `true` per le frasi delle minacce e delle tattiche (quelle dei rilevatori). */
function isTactical(item: TutorItem): boolean {
  return item.fact.category === 'threats' || item.fact.category === 'tactics';
}

export function buildCoachMessages(
  explanation: TutorExplanation,
  comments: readonly MoveComment[],
  seen: ReadonlySet<string>,
  notation: Notation,
): { messages: CoachMessage[]; keys: string[] } {
  const isFirst = seen.size === 0;
  const keys: string[] = [];
  const messages = comments.flatMap((comment) => commentMessages(comment, notation));

  const threats: CoachMessage[] = [];
  const news: CoachMessage[] = [];
  for (const section of explanation.sections) {
    for (const item of section.items) {
      const key = messageKey(item.fact);
      const message = { heading: section.title, fact: item.fact, line: null, cardId: item.cardId };
      if (section.title === NOW_SECTION && isTactical(item)) {
        threats.push(message);
      } else if (item.fact.text.startsWith(TURN_PREFIX)) {
        if (isFirst) {
          news.push(message);
        }
      } else if (!seen.has(key)) {
        news.push(message);
      }
      keys.push(key);
    }
  }
  return { messages: [...messages, ...threats, ...news], keys };
}
