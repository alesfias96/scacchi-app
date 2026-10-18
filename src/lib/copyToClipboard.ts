/**
 * Copia un testo negli appunti del sistema, pronto da incollare altrove
 * (in un'altra app, in un sito, in un messaggio).
 *
 * @param text - il testo da copiare.
 * @returns una promessa che si risolve quando il testo è stato copiato.
 * @throws se il sistema non permette di scrivere negli appunti.
 */
export function copyToClipboard(text: string): Promise<void> {
  return navigator.clipboard.writeText(text);
}
