/**
 * Scrive un fatto dei rilevatori come frase completa.
 *
 * Rust scrive la frase e, a parte, le mosse citate in notazione internazionale
 * (ad esempio "Il Bianco può dare scacco matto in una mossa con" + `Qxf7#`).
 * Qui le mosse vengono tradotte nella notazione scelta e aggiunte alla frase.
 *
 * @param fact - il fatto ricevuto da Rust.
 * @param notation - la notazione scelta nelle Impostazioni.
 * @returns la frase da mostrare, ad esempio "… in una mossa con ♛xf7#".
 */
import type { Fact } from '../../../bindings/Fact';
import type { Notation } from '../../../bindings/Notation';
import { formatSan } from '../../../lib/formatSan';

export function formatFact(fact: Fact, notation: Notation): string {
  if (fact.moves.length === 0) {
    return fact.text;
  }
  const moves = fact.moves.map((san) => formatSan(san, notation)).join(' oppure ');
  return `${fact.text} ${moves}`;
}
